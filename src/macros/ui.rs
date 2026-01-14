use evdev::{KeyCode, MiscCode};
use gpui::{
    self, App, AppContext, Context, Entity, IntoElement, Keystroke, KeystrokeEvent, ParentElement,
    Render, Styled, Subscription, Window, div,
};
use gpui_component::{
    StyledExt,
    button::{Button, Toggle},
    gray_900,
    input::{InputState, NumberInput},
};
use std::fmt::Debug;

enum Action {
    KeyEvent(KeyCode),
    MouseEvent(MiscCode),
}

impl Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::KeyEvent(key_code) => {
                write!(f, "{:?}", key_code)
            }
            Action::MouseEvent(misc_code) => write!(f, "{:?}", misc_code),
        }
    }
}
impl From<&Keystroke> for Action {
    fn from(value: &Keystroke) -> Self {
        let key_code = match value.key.as_str() {
            "w" => KeyCode::KEY_W,
            "a" => KeyCode::KEY_A,
            "s" => KeyCode::KEY_S,
            "d" => KeyCode::KEY_D,
            _ => KeyCode::KEY_UNKNOWN,
        };
        Self::KeyEvent(key_code)
    }
}

// Share/Store this in a way of
// |----||- -------| |------- -------- -------- --||---- -------- -------|
// 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
// press/releaase (1) mouse (6), key (10), length (27), repeat (21)
pub struct MacroItem {
    pressed: bool,
    action: Action,
    length: u64,

    edit: bool,
    length_edit: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
    key_editor: Entity<KeyEditor>,
}

impl MacroItem {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let length_edit = cx.new(|cx| InputState::new(window, cx).default_value(100.to_string()));
        let _subscriptions = vec![];
        let key_editor = KeyEditor::view(window, cx);

        Self {
            pressed: false,
            action: Action::KeyEvent(KeyCode::KEY_W),
            length: 100,
            edit: false,
            length_edit,
            _subscriptions,
            key_editor,
        }
    }
}

impl Render for MacroItem {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.edit {
            div().child(
                Button::new("macro-action")
                    .label(format!("{:?} for {}ms", self.action, self.length))
                    .on_click(cx.listener(|this, _, window, _| {
                        this.edit = true;
                        window.refresh();
                    })),
            )
        } else {
            div()
                .p_4()
                .bg(gray_900())
                .h_flex()
                .child(
                    Toggle::new("some-toggle")
                        .label(if self.pressed { "press" } else { "release" })
                        .checked(self.pressed)
                        .on_click(cx.listener(|view, checked, _, cx| {
                            view.pressed = *checked;
                            cx.notify();
                        })),
                )
                .child(self.key_editor.clone())
                .child("for")
                .child(NumberInput::new(&self.length_edit).min_w_12())
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

    fn subscribe(&mut self, cx: &mut Context<Self>) {
        let listener = cx.listener(|this, event: &KeystrokeEvent, window, cx| {
            println!("input! {:?}", event);
            this.keystroke = event.keystroke.to_owned();
            // this sends a slightly quicker update than waiting for the subscription to be dropped.
            cx.notify();
            this.unsubscribe();
        });
        let sub = cx.intercept_keystrokes(listener);
        self.key_subscription = Some(vec![sub]);
    }

    fn unsubscribe(&mut self) {
        self.key_subscription = None;
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
            .on_click(cx.listener(|this, _, _, cx| {
                this.subscribe(cx);
            }))
    }
}
