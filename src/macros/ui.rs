use crate::{
    duration_input::DurationInput,
    macros::definitions::{MacroType, RawMacroEntry, from_direction_str, from_mouse_str},
};
use enigo::Direction;
use gpui::{
    App, AppContext, Context, Entity, IntoElement, Keystroke, KeystrokeEvent, ParentElement,
    Render, Styled, Subscription, Window, div,
};
use gpui_component::{
    IndexPath, StyledExt,
    button::Button,
    gray_600,
    input::{InputEvent, InputState, NumberInput},
    select::{Select, SelectEvent, SelectState},
};
use regex::Regex;

pub struct MacroEntryUI {
    raw: RawMacroEntry,
    editing: bool,

    key_editor: Entity<KeyEditor>,
    repeat_input: Entity<InputState>,
    direction_state: Entity<SelectState<Vec<&'static str>>>,
    macro_type_state: Entity<SelectState<Vec<&'static str>>>,
    mouse_state: Entity<SelectState<Vec<&'static str>>>,
    duration_input: Entity<DurationInput>,
    _subscriptions: Vec<Subscription>,
}
impl MacroEntryUI {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let key_editor = KeyEditor::view(window, cx);
        let repeat_input = cx.new(
            |cx| {
                InputState::new(window, cx)
                    .placeholder("Integer value")
                    .pattern(Regex::new(r"^\d+$").unwrap())
            }, // Only positive integers
        );
        let direction_state = cx.new(|cx| {
            SelectState::new(
                vec!["Press", "Release", "Click"],
                Some(IndexPath::default()), // Select first item
                window,
                cx,
            )
        });
        let mouse_state = cx.new(|cx| {
            SelectState::new(
                vec!["Left", "Middle", "Right", "Fourth", "Fifth"],
                Some(IndexPath::default()), // Select first item
                window,
                cx,
            )
        });
        let macro_type_state = cx.new(|cx| {
            SelectState::new(
                vec!["Mouse", "Key"],
                Some(IndexPath::default()), // Select first item
                window,
                cx,
            )
        });
        let duration_input = DurationInput::view(window, cx);
        duration_input
            .as_mut(cx)
            .load_value(100)
            .visible_days(false)
            .visible_hours(false);

        let duration_sub = cx.subscribe(&duration_input, |this, duration, ev: &InputEvent, cx| {
            this.raw.length = u64::from(duration.read(cx)) as u32;
            println!("{:?}", this.raw);
        });

        let direction_sub = cx.subscribe(
            &direction_state,
            |this, entity, ev: &SelectEvent<Vec<&str>>, cx: &mut Context<'_, MacroEntryUI>| {
                this.raw.direction = from_direction_str(entity.read(cx).selected_value().unwrap())
                    .expect("Now how to deal with this...");
            },
        );
        let macro_type_sub = cx.subscribe(
            &macro_type_state,
            |this, entity, ev: &SelectEvent<Vec<&str>>, cx: &mut Context<'_, MacroEntryUI>| {
                this.raw.macro_type =
                    MacroType::try_from(entity.read(cx).selected_value().unwrap()).expect("...");
            },
        );
        let data_sub = cx.subscribe(
            &mouse_state,
            |this, entity, ev: &SelectEvent<Vec<&str>>, cx: &mut Context<'_, MacroEntryUI>| {
                this.raw.data = from_mouse_str(entity.read(cx).selected_value().unwrap())
                    .expect("Failed translate")
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
    pub fn load(&mut self, data: u64) -> Result<(), String> {
        self.raw = RawMacroEntry::try_from(data)?;
        Ok(())
    }
    pub fn save(&self) -> u64 {
        u64::from(self.raw)
    }
}
impl Render for MacroEntryUI {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
