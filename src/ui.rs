use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};
use gpui_component::{
    DivInspector, StyledExt,
    button::Button,
    group_box::GroupBox,
    input::{InputEvent, InputState, NumberInput, NumberInputEvent, StepAction},
    label::Label,
};
use regex::Regex;

use crate::counter::NumberInputStory;

pub struct IntervalInput {
    hour_input: Entity<InputState>,
    hour_value: usize,
    minute_input: Entity<InputState>,
    minute_value: usize,
    second_input: Entity<InputState>,
    second_value: usize,
    millisecond_input: Entity<InputState>,
    millisecond_value: usize,
}
impl IntervalInput {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let hour_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("0")
                .placeholder("Enter Hours")
                .validate(|s, _| s.parse::<usize>().is_ok())
                .pattern(Regex::new(r"^\d+$").unwrap())
        });
        let _ = cx.subscribe_in(
            &hour_input,
            window,
            |view, state, event, window, cx| match event {
                InputEvent::Change => {
                    let text = state.read(cx).value();
                    if let Ok(new_value) = text.parse::<usize>() {
                        view.hour_value = new_value;
                    }
                }
                NumberInputEvent::Step(StepAction::Increment) => state.update(cx, |input, cx| {
                    input.set_value(view.hour_value.to_string(), window, cx);
                }),
                _ => {}
            },
        );

        let minute_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("0")
                .placeholder("Enter Minutes")
                .validate(|s, _| s.parse::<usize>().is_ok())
                .pattern(Regex::new(r"^[0-59]\d*$").unwrap())
        });
        let second_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("0")
                .placeholder("Enter Minutes")
                .validate(|s, _| s.parse::<usize>().is_ok())
                .pattern(Regex::new(r"^[0-59]\d*$").unwrap())
        });
        let millisecond_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("100")
                .placeholder("Enter Minutes")
                .validate(|s, _| s.parse::<usize>().is_ok())
                .pattern(Regex::new(r"^[0-1000]\d*$").unwrap())
        });

        Self {
            hour_input,
            hour_value: 0,
            minute_input,
            minute_value: 0,
            second_input,
            second_value: 0,
            millisecond_input,
            millisecond_value: 100,
        }
    }
}

impl Render for IntervalInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(
            GroupBox::new()
                .border_12()
                .h_flex()
                .title("Interval")
                .child(div().h_flex().children(vec![
                    NumberInput::new(&self.hour_input).prefix("hours: "),
                    NumberInput::new(&self.minute_input).prefix("minutes: "),
                    NumberInput::new(&self.second_input).prefix("seconds: "),
                    NumberInput::new(&self.millisecond_input).prefix("milliseconds: "),
                ])),
        )
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
        let interval = cx.new(|cx| IntervalInput::new(window, cx));
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
            .child(div().child("Something").child(self.interval.clone()))
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
