use crate::{
    duration_input::DurationInput,
    macros::definitions::{MacroType, RawMacroEntry, SelectDirection, SelectMouseAction, ToVec},
};
use enigo::Direction;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, Keystroke, KeystrokeEvent, ParentElement,
    Render, Styled, Subscription, Window, div,
};
use gpui_component::{
    StyledExt,
    button::Button,
    gray_600,
    input::{InputEvent, InputState, NumberInput},
    select::{Select, SelectEvent, SelectState},
};
use regex::Regex;

/// Main UI for editing a macro entry.
pub struct MacroEntryUI {
    pub raw: RawMacroEntry,
    pub editing: bool,

    key_editor: Entity<KeyEditor>,
    repeat_input: Entity<InputState>,
    direction_state: Entity<SelectState<Vec<SelectDirection>>>,
    macro_type_state: Entity<SelectState<Vec<MacroType>>>,
    mouse_state: Entity<SelectState<Vec<SelectMouseAction>>>,
    duration_input: Entity<DurationInput>,
    _subscriptions: Vec<Subscription>,
}
impl MacroEntryUI {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        // How does on explain this?
        // Basically, every entity stores information about itself and data around it, the UI then references these entities.
        // Subscriptions are used to listen to sub events and act on them.
        // Yes, this might look like a mess but eh.

        let key_editor = KeyEditor::view(window, cx);
        let repeat_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Integer value")
                .pattern(Regex::new(r"^\d+$").unwrap())
        });
        let direction_state = cx.new(|cx| {
            SelectState::new(
                SelectDirection::to_vec(),
                SelectDirection::default_index(),
                window,
                cx,
            )
        });
        let mouse_state = cx.new(|cx| {
            SelectState::new(
                SelectMouseAction::to_vec(),
                SelectMouseAction::default_index(),
                window,
                cx,
            )
        });
        let macro_type_state = cx.new(|cx| {
            SelectState::new(MacroType::to_vec(), MacroType::default_index(), window, cx)
        });
        let duration_input = DurationInput::view(window, cx);
        duration_input
            .as_mut(cx)
            .load_value(100)
            .visible_days(false)
            .visible_hours(false);

        let duration_sub = cx.subscribe(&duration_input, |this, duration, _: &InputEvent, cx| {
            this.raw.length = u64::from(duration.read(cx)) as u32;
            println!("{:?}", this.raw);
        });

        let direction_sub = cx.subscribe(
            &direction_state,
            |this,
             entity,
             _: &SelectEvent<Vec<SelectDirection>>,
             cx: &mut Context<'_, MacroEntryUI>| {
                this.raw.direction = Direction::from(entity.read(cx).selected_value().unwrap())
            },
        );
        let macro_type_sub = cx.subscribe(
            &macro_type_state,
            |this, entity, _: &SelectEvent<Vec<MacroType>>, cx: &mut Context<'_, MacroEntryUI>| {
                this.raw.macro_type = entity.read(cx).selected_value().unwrap().to_owned();
            },
        );
        let data_sub = cx.subscribe(
            &mouse_state,
            |this,
             entity,
             _: &SelectEvent<Vec<SelectMouseAction>>,
             cx: &mut Context<'_, MacroEntryUI>| {
                this.raw.data = u64::from(entity.read(cx).selected_value().unwrap()) as u16;
            },
        );

        let _subscriptions = vec![duration_sub, direction_sub, macro_type_sub, data_sub];

        Self {
            raw: RawMacroEntry::default(),
            editing: false,
            key_editor,
            repeat_input,
            direction_state,
            macro_type_state,
            mouse_state,
            duration_input,
            _subscriptions,
        }
    }
    /// Load data from a u64. Used as a cheap way of storing data
    pub fn load(&mut self, data: u64, window: &mut Window, cx: &mut App) -> Result<(), String> {
        self.raw = RawMacroEntry::try_from(data)?;

        // cx.update_entity(&self.key_editor, |entity, cx| {
        // entity.keystroke =
        // });
        cx.update_entity(&self.repeat_input, |entity, cx| {
            entity.set_value(self.raw.repeat.to_string(), window, cx);
        });
        cx.update_entity(&self.direction_state, |entity, cx| {
            entity.set_selected_value(&SelectDirection::from(self.raw.direction), window, cx);
        });
        cx.update_entity(&self.macro_type_state, |entity, cx| {
            entity.set_selected_value(&self.raw.macro_type, window, cx);
        });
        cx.update_entity(&self.mouse_state, |entity, cx| {
            entity.set_selected_value(&SelectMouseAction::from(self.raw.data as u64), window, cx);
        });
        cx.update_entity(&self.duration_input, |entity, cx| {
            entity.load_value(self.raw.length as u64);
        });

        Ok(())
    }
    /// Save the current data to a u64.
    pub fn save(&self) -> u64 {
        u64::from(self.raw)
    }
}
impl Render for MacroEntryUI {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.editing {
            div()
                .h_flex()
                .bg(gray_600())
                .p_4()
                .child(Select::new(&self.direction_state))
                .child(Select::new(&self.macro_type_state))
                .child(match self.raw.macro_type {
                    MacroType::Mouse => div().child(Select::new(&self.mouse_state)),
                    MacroType::Key => div().child(self.key_editor.clone()),
                })
                .child(if self.raw.direction == Direction::Click {
                    "Every"
                } else {
                    "For"
                })
                .child(self.duration_input.clone())
                .child("Repeat")
                .child(NumberInput::new(&self.repeat_input).suffix(" times"))
                .child(
                    Button::new("macro-finish")
                        .label("Submit Changes")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.editing = false;
                            cx.notify();
                        })),
                )
        } else {
            div().child(
                Button::new("macro-button")
                    .label(format!("{}", self.raw))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.editing = true;
                        cx.notify();
                    })),
            )
        }
    }
}

/// Custom key input.
///
/// TODO: Use something else other than keystroke, this doesn't read everything.
pub struct KeyEditor {
    keystroke: Keystroke,
    key_subscription: Option<Vec<Subscription>>,
}
impl KeyEditor {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            keystroke: Keystroke::default(),
            key_subscription: None,
        }
    }

    fn subscribe(&mut self, cx: &mut Context<Self>, window: &mut Window) {
        let key_down_listener = cx.listener(|this, event: &KeystrokeEvent, window, cx| {
            println!("input! {:?}", event);
            this.keystroke = event.keystroke.to_owned();
            // this sends a slightly quicker update than waiting for the subscription to be dropped.
            cx.notify();
            this.unsubscribe();
        });
        let sub = cx.intercept_keystrokes(key_down_listener);
        self.key_subscription = Some(vec![sub]);
        // self.key_subscription = Some(vec![]);
    }

    fn unsubscribe(&mut self) {
        self.key_subscription = None;
    }

    fn as_code(&self) -> u16 {
        0
    }
}

impl Render for KeyEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Button::new("key-listener")
            .label(if self.key_subscription.is_some() {
                "Listening for input...".into()
            } else {
                self.keystroke.key.to_string()
            })
            .on_click(cx.listener(|this, ce, window, cx| {
                this.subscribe(cx, window);
            }))
    }
}
