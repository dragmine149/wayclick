use gpui::{
    App, AppContext, Context, Entity, Hsla, IntoElement, KeyEvent, ParentElement, Render, Rgba,
    Styled, Subscription, Window, div, px,
};
use gpui_component::{
    DivInspector, Sizable, StyledExt,
    button::Button,
    group_box::GroupBox,
    input::{InputEvent, InputState, NumberInput, NumberInputEvent, StepAction},
    label::Label,
};
use regex::Regex;

use crate::counter::NumberInputStory;

pub struct IntervalInput {
    hour_input: Entity<InputState>,
    hour_value: u64,
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
        let hour_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("0")
                .pattern(Regex::new(r"^\d+$").unwrap())
        });
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
                .pattern(Regex::new(r"^[0-1000]\d*$").unwrap())
        });

        let _subscriptions = vec![
            cx.subscribe_in(&hour_input, window, Self::on_input_event),
            cx.subscribe_in(&hour_input, window, Self::on_number_input_event),
            cx.subscribe_in(&minute_input, window, Self::on_input_event),
            cx.subscribe_in(&minute_input, window, Self::on_number_input_event),
            cx.subscribe_in(&second_input, window, Self::on_input_event),
            cx.subscribe_in(&second_input, window, Self::on_number_input_event),
            cx.subscribe_in(&millisecond_input, window, Self::on_input_event),
            cx.subscribe_in(&millisecond_input, window, Self::on_number_input_event),
        ];

        Self {
            hour_input,
            hour_value: 0,
            minute_input,
            minute_value: 0,
            second_input,
            second_value: 0,
            millisecond_input,
            millisecond_value: 100,
            _subscriptions,
        }
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
                    if state == &self.hour_input {
                        self.hour_value = value;
                    } else if state == &self.minute_input {
                        self.minute_value = value;
                    } else if state == &self.second_input {
                        self.second_value = value;
                    } else if state == &self.millisecond_input {
                        self.millisecond_value = value;
                    }
                    println!("Change: {}", text);
                }
            }
            _ => {} // InputEvent::PressEnter { secondary } => todo!(),
                    // InputEvent::Focus => todo!(),
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
                    if this == &self.hour_input {
                        self.hour_value = self.hour_value.saturating_sub(1);
                        this.update(cx, |input, cx| {
                            println!("Update value to {}", self.hour_value.to_string());
                            input.set_value(self.hour_value.to_string(), window, cx);
                        })
                    } else if this == &self.minute_input {
                        self.minute_value = self.minute_value.saturating_sub(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.minute_value.to_string(), window, cx);
                        })
                    } else if this == &self.second_input {
                        self.second_value = self.second_value.saturating_sub(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.second_value.to_string(), window, cx);
                        })
                    } else if this == &self.millisecond_input {
                        self.millisecond_value = self.millisecond_value.saturating_sub(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.millisecond_value.to_string(), window, cx);
                        })
                    }
                }
                StepAction::Increment => {
                    if this == &self.hour_input {
                        self.hour_value = self.hour_value.saturating_add(1);
                        this.update(cx, |input, cx| {
                            println!("Update value to {}", self.hour_value.to_string());
                            input.set_value(self.hour_value.to_string(), window, cx);
                        })
                    } else if this == &self.minute_input {
                        self.minute_value = self.minute_value.saturating_add(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.minute_value.to_string(), window, cx);
                        })
                    } else if this == &self.second_input {
                        self.second_value = self.second_value.saturating_add(1);
                        this.update(cx, |input, cx| {
                            input.set_value(self.second_value.to_string(), window, cx);
                        })
                    } else if this == &self.millisecond_input {
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
                NumberInput::new(&self.hour_input).prefix("hours:"),
                NumberInput::new(&self.minute_input).prefix("minutes: "),
                NumberInput::new(&self.second_input).prefix("seconds: "),
                NumberInput::new(&self.millisecond_input).prefix("milliseconds: "),
            ]))
    }
}

pub struct ClickUI {
    interval: Entity<IntervalInput>,
    counter_input: Entity<NumberInputStory>,
    inspector: Entity<DivInspector>,
}

impl ClickUI {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }

    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let interval = IntervalInput::view(window, cx);
        let counter_input = NumberInputStory::view(window, cx);

        let inspector = cx.new(|cx| DivInspector::new(window, cx));

        Self {
            interval,
            counter_input,
            inspector,
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
            .child(self.interval.clone())
            .child(self.counter_input.clone())
            .child(
                Button::new("Test btn")
                    .on_click(|_, _, _| {
                        println!("Button clicked!");
                    })
                    .label("click me"),
            )
    }
}
