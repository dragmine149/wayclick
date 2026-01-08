use std::{
    env::current_exe,
    ops::Sub,
    process::{Command, Stdio},
};

use gpui::{
    App, AppContext, Context, Entity, FocusHandle, Hsla, IntoElement, ParentElement, Render,
    Styled, Subscription, Window, div,
};
use gpui_component::{
    Disableable, DivInspector, Selectable, StyledExt,
    button::{Button, ButtonGroup},
    group_box::GroupBox,
    input::{InputEvent, InputState, NumberInput, NumberInputEvent, StepAction},
    label::Label,
};
use regex::Regex;

use crate::{
    cli::{daemon_start, daemon_stop},
    storage::{Data, DataBuilder, Settings},
};

pub struct IntervalInput {
    minute_input: Entity<InputState>,
    minute_value: u64,
    second_input: Entity<InputState>,
    second_value: u64,
    millisecond_input: Entity<InputState>,
    millisecond_value: u64,

    _subscriptions: Vec<Subscription>,
}
impl IntervalInput {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let minute_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("0")
                .pattern(Regex::new(r"^[0-59]\d*$").unwrap())
        });
        let second_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("0")
                .pattern(Regex::new(r"^[0-59]\d*$").unwrap())
        });
        let millisecond_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("100")
                .pattern(Regex::new(r"^[0-999]\d*$").unwrap())
        });

        let _subscriptions = vec![
            cx.subscribe_in(&minute_input, window, Self::on_input_event),
            cx.subscribe_in(&minute_input, window, Self::on_number_input_event),
            cx.subscribe_in(&second_input, window, Self::on_input_event),
            cx.subscribe_in(&second_input, window, Self::on_number_input_event),
            cx.subscribe_in(&millisecond_input, window, Self::on_input_event),
            cx.subscribe_in(&millisecond_input, window, Self::on_number_input_event),
            cx.observe_window_activation(window, Self::on_focus),
        ];

        Self {
            minute_input,
            minute_value: 0,
            second_input,
            second_value: 0,
            millisecond_input,
            millisecond_value: 100,
            _subscriptions,
        }
    }

    fn on_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !window.is_window_active() {
            return;
        }

        let data = Settings::load_data();

        self.millisecond_value = data.milliseconds.unwrap();
        self.millisecond_input.update(cx, |input, cx| {
            input.set_value(self.millisecond_value.to_string(), window, cx)
        });
        self.second_value = data.seconds.unwrap();
        self.second_input.update(cx, |input, cx| {
            input.set_value(self.second_value.to_string(), window, cx)
        });
        self.minute_value = data.minutes.unwrap();
        self.minute_input.update(cx, |input, cx| {
            input.set_value(self.minute_value.to_string(), window, cx)
        });
    }

    fn on_input_event(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!("input");
        match event {
            InputEvent::Change => {
                let text = state.read(cx).value();
                if let Ok(value) = text.parse::<u64>() {
                    if state == &self.minute_input {
                        self.minute_value = value;
                    }
                    if state == &self.second_input {
                        self.second_value = value;
                    }
                    if state == &self.millisecond_input {
                        self.millisecond_value = value;
                    }
                    println!("Change: {}", text);

                    let data = DataBuilder::default()
                        .minutes(self.minute_value)
                        .seconds(self.second_value)
                        .milliseconds(self.millisecond_value)
                        .build()
                        .unwrap();
                    Settings::save_data(data);
                }
            }
            InputEvent::Focus => println!("Focused... {:?}", state),
            _ => {} // InputEvent::PressEnter { secondary } => todo!(),
                    // InputEvent::Blur => todo!(),
        }
    }

    fn on_number_input_event(
        &mut self,
        this: &Entity<InputState>,
        event: &NumberInputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!("num input");
        match event {
            NumberInputEvent::Step(step_action) => match step_action {
                StepAction::Decrement => {
                    if this == &self.minute_input {
                        self.minute_value = self.minute_value.saturating_sub(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.minute_value.to_string(), window, cx);
                        })
                    }
                    if this == &self.second_input {
                        self.second_value = self.second_value.saturating_sub(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.second_value.to_string(), window, cx);
                        })
                    }
                    if this == &self.millisecond_input {
                        self.millisecond_value = self.millisecond_value.saturating_sub(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.millisecond_value.to_string(), window, cx);
                        })
                    }
                }
                StepAction::Increment => {
                    if this == &self.minute_input {
                        self.minute_value = self.minute_value.saturating_add(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.minute_value.to_string(), window, cx);
                        })
                    }
                    if this == &self.second_input {
                        self.second_value = self.second_value.saturating_add(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.second_value.to_string(), window, cx);
                        })
                    }
                    if this == &self.millisecond_input {
                        self.millisecond_value = self.millisecond_value.saturating_add(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.millisecond_value.to_string(), window, cx);
                        })
                    }
                }
            },
        }
    }
}

impl Render for IntervalInput {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        GroupBox::new()
            .size_full()
            .border_1()
            .border_color(Hsla::black())
            .v_flex()
            .title("Interval")
            .child(div().h_flex().size_full().children(vec![
                NumberInput::new(&self.minute_input).prefix("minutes: "),
                NumberInput::new(&self.second_input).prefix("seconds: "),
                NumberInput::new(&self.millisecond_input).prefix("milliseconds: "),
            ]))
    }
}

pub struct InitialInterval {
    second_input: Entity<InputState>,
    second_value: u64,

    _subscriptions: Vec<Subscription>,
}

impl InitialInterval {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let second_input = cx.new(|cx| InputState::new(window, cx).default_value("0"));

        let _subscriptions = vec![
            cx.subscribe_in(&second_input, window, Self::on_input_event),
            cx.subscribe_in(&second_input, window, Self::on_number_input_event),
            cx.observe_window_activation(window, Self::on_focus),
        ];

        Self {
            second_input,
            second_value: 0,
            _subscriptions,
        }
    }

    fn on_focus(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !window.is_window_active() {
            return;
        }

        let data = Settings::load_data();
        self.second_value = data.initial.unwrap();
        self.second_input.update(cx, |input, cx| {
            input.set_value(self.second_value.to_string(), window, cx)
        });
    }

    fn on_input_event(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!("input");
        match event {
            InputEvent::Change => {
                let text = state.read(cx).value();
                if let Ok(value) = text.parse::<u64>() {
                    if state == &self.second_input {
                        self.second_value = value;
                    }
                    println!("Change: {}", text);

                    let data = DataBuilder::default()
                        .initial(self.second_value)
                        .build()
                        .unwrap();
                    Settings::save_data(data);
                }
            }
            InputEvent::Focus => println!("Focused... {:?}", state),
            _ => {} // InputEvent::PressEnter { secondary } => todo!(),
                    // InputEvent::Blur => todo!(),
        }
    }

    fn on_number_input_event(
        &mut self,
        this: &Entity<InputState>,
        event: &NumberInputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        println!("num input");
        match event {
            NumberInputEvent::Step(step_action) => match step_action {
                StepAction::Decrement => {
                    if this == &self.second_input {
                        self.second_value = self.second_value.saturating_sub(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.second_value.to_string(), window, cx);
                        })
                    }
                }
                StepAction::Increment => {
                    if this == &self.second_input {
                        self.second_value = self.second_value.saturating_add(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.second_value.to_string(), window, cx);
                        })
                    }
                }
            },
        }
    }
}

impl Render for InitialInterval {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        GroupBox::new()
            .size_full()
            .border_1()
            .border_color(Hsla::black())
            .v_flex()
            .title("Interval")
            .child(div().h_flex().size_full().children(vec![
                NumberInput::new(&self.second_input).suffix("seconds before autoclicking"),
            ]))
    }
}

pub struct Activate {
    is_clicking: bool,
}
impl Activate {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self { is_clicking: false }
    }
}
impl Render for Activate {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            ButtonGroup::new("activation-group")
                .children(vec![
                    Button::new("activate")
                        .label("Start Autoclicking")
                        .disabled(self.is_clicking),
                    Button::new("deactivate")
                        .label("Stop Autoclicking")
                        .disabled(!self.is_clicking),
                ])
                .on_click(cx.listener(|view, clicks: &Vec<usize>, window, cx| {
                    if clicks.contains(&0) {
                        println!("Start clicking");
                        view.is_clicking = true;
                        // Command::new(program);
                        let myself = current_exe().unwrap();
                        let child = Command::new(myself)
                            .arg("start")
                            .stdout(Stdio::inherit())
                            .spawn();
                    }
                    if clicks.contains(&1) {
                        println!("Stop clicking");
                        view.is_clicking = false;
                        daemon_stop();
                    }
                })),
        )
    }
}

pub struct ClickUI {
    interval: Entity<IntervalInput>,
    inital: Entity<InitialInterval>,
    inspector: Entity<DivInspector>,
    activate: Entity<Activate>,
    _subscriptions: Vec<Subscription>,
}

impl ClickUI {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let interval = IntervalInput::view(window, cx);
        let inital = InitialInterval::view(window, cx);
        let inspector = cx.new(|cx| DivInspector::new(window, cx));
        let activate = Activate::view(window, cx);

        let _subscriptions = vec![cx.observe_window_activation(window, |this, window, cx| {
            println!("window activation: {}", window.is_window_active())
        })];

        Self {
            interval,
            inital,
            inspector,
            activate,
            _subscriptions,
        }
    }
}

impl Render for ClickUI {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .items_center()
            .justify_center()
            .child(self.inspector.clone())
            .child(Label::new("WayClicker").text_3xl().font_bold())
            .child(self.inital.clone())
            .child(self.interval.clone())
            .child(self.activate.clone())
    }
}
