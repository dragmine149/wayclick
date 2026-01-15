use gpui::{
    App, AppContext, Context, Entity, ParentElement, Render, Styled, Subscription, Window, div, px,
};
use gpui_component::{
    StyledExt,
    input::{InputEvent, InputState, NumberInput, NumberInputEvent, StepAction},
};
use regex::Regex;
use std::fmt::{Debug, Display};

const DAY_LENGTH: u64 = 24 * 60 * 60 * 1000;
const HOUR_LENGTH: u64 = 60 * 60 * 1000;
const MINUTE_LENGTH: u64 = 60 * 1000;
const SECOND_LENGTH: u64 = 1000;

#[derive(Debug)]
pub struct DurationInput {
    days_input: Entity<InputState>,
    pub days_value: u64,
    pub days_visible: bool,
    hours_input: Entity<InputState>,
    pub hours_value: u64,
    pub hours_visible: bool,
    minutes_input: Entity<InputState>,
    pub minutes_value: u64,
    pub minutes_visible: bool,
    seconds_input: Entity<InputState>,
    pub seconds_value: u64,
    pub seconds_visible: bool,
    millisecond_input: Entity<InputState>,
    pub millisecond_value: u64,
    pub millisecond_visible: bool,
    _subscriptions: Vec<Subscription>,
}
impl DurationInput {
    pub fn view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx))
    }
    pub fn to_view(self, window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| self)
    }
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let days_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("0")
                .pattern(Regex::new(r"^\d+$").unwrap())
        });
        let hours_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("0")
                .pattern(Regex::new(r"^[0-23]\d*$").unwrap())
        });
        let minutes_input = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("0")
                .pattern(Regex::new(r"^[0-59]\d*$").unwrap())
        });
        let seconds_input = cx.new(|cx| {
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
            cx.subscribe_in(&days_input, window, Self::on_input_event),
            cx.subscribe_in(&days_input, window, Self::on_number_input_event),
            cx.subscribe_in(&hours_input, window, Self::on_input_event),
            cx.subscribe_in(&hours_input, window, Self::on_number_input_event),
            cx.subscribe_in(&minutes_input, window, Self::on_input_event),
            cx.subscribe_in(&minutes_input, window, Self::on_number_input_event),
            cx.subscribe_in(&seconds_input, window, Self::on_input_event),
            cx.subscribe_in(&seconds_input, window, Self::on_number_input_event),
            cx.subscribe_in(&millisecond_input, window, Self::on_input_event),
            cx.subscribe_in(&millisecond_input, window, Self::on_number_input_event),
        ];

        Self {
            days_input,
            days_value: 0,
            days_visible: true,
            hours_input,
            hours_value: 0,
            hours_visible: true,
            minutes_input,
            minutes_value: 0,
            minutes_visible: true,
            seconds_input,
            seconds_value: 0,
            seconds_visible: true,
            millisecond_input,
            millisecond_value: 0,
            millisecond_visible: true,
            _subscriptions,
        }
    }
    pub fn load_value(&mut self, mut value: u64) -> &mut Self {
        self.days_value = value / DAY_LENGTH;
        value %= DAY_LENGTH;
        self.hours_value = value / HOUR_LENGTH;
        value %= HOUR_LENGTH;
        self.minutes_value = value / MINUTE_LENGTH;
        value %= MINUTE_LENGTH;
        self.seconds_value = value / SECOND_LENGTH;
        value %= SECOND_LENGTH;
        self.millisecond_value = value;

        self
    }
    pub fn get_value(&self) -> u64 {
        u64::from(self)
    }

    pub fn visible_days(&mut self, visible: bool) -> &mut Self {
        self.days_visible = visible;
        self
    }
    pub fn visible_hours(&mut self, visible: bool) -> &mut Self {
        self.hours_visible = visible;
        self
    }
    pub fn visible_minutess(&mut self, visible: bool) -> &mut Self {
        self.minutes_visible = visible;
        self
    }
    pub fn visible_secondss(&mut self, visible: bool) -> &mut Self {
        self.seconds_visible = visible;
        self
    }
    pub fn visible_milliseconds(&mut self, visible: bool) -> &mut Self {
        self.millisecond_visible = visible;
        self
    }

    fn on_input_event(
        &mut self,
        state: &Entity<InputState>,
        event: &InputEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change => {
                let text = state.read(cx).value();
                if let Ok(value) = text.parse::<u64>() {
                    match state {
                        val if val == &self.millisecond_input => {
                            self.millisecond_value = value;
                        }
                        val if val == &self.seconds_input => self.seconds_value = value,
                        val if val == &self.minutes_input => self.minutes_value = value,
                        val if val == &self.hours_input => self.hours_value = value,
                        val if val == &self.days_input => self.days_value = value,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn on_number_input_event(
        &mut self,
        this: &Entity<InputState>,
        event: &NumberInputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let multi = match event {
            NumberInputEvent::Step(StepAction::Increment) => 1,
            NumberInputEvent::Step(StepAction::Decrement) => -1,
        };

        match this {
            val if val == &self.millisecond_input => {
                self.millisecond_value = self
                    .millisecond_value
                    .saturating_add_signed(multi)
                    .clamp(0, 999);
                this.update(cx, |input, cx| {
                    input.set_value(self.millisecond_value.to_string(), window, cx);
                })
            }
            val if val == &self.seconds_input => {
                self.seconds_value = self.seconds_value.saturating_add_signed(multi).clamp(0, 59);
                this.update(cx, |input, cx| {
                    input.set_value(self.seconds_value.to_string(), window, cx);
                })
            }
            val if val == &self.minutes_input => {
                self.minutes_value = self.minutes_value.saturating_add_signed(multi).clamp(0, 59);
                this.update(cx, |input, cx| {
                    input.set_value(self.minutes_value.to_string(), window, cx);
                })
            }
            val if val == &self.hours_input => {
                self.hours_value = self.hours_value.saturating_add_signed(multi).clamp(0, 24);
                this.update(cx, |input, cx| {
                    input.set_value(self.hours_value.to_string(), window, cx);
                })
            }
            val if val == &self.days_input => {
                self.days_value = self.days_value.saturating_add_signed(multi).clamp(0, 31);
                this.update(cx, |input, cx| {
                    input.set_value(self.days_value.to_string(), window, cx);
                })
            }
            _ => {}
        }
    }
}
impl Display for DurationInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(
                f,
                "{} days {} hours {} minutes {} seconds {} milliseconds",
                self.days_value,
                self.hours_value,
                self.minutes_value,
                self.seconds_value,
                self.millisecond_value
            )
        } else {
            write!(
                f,
                "{}{}{}{}{}",
                if self.days_value > 1 {
                    format!("{} d", self.days_value)
                } else {
                    "".into()
                },
                if self.hours_value > 1 {
                    format!("{} h", self.hours_value)
                } else {
                    "".into()
                },
                if self.minutes_value > 1 {
                    format!("{} min", self.minutes_value)
                } else {
                    "".into()
                },
                if self.seconds_value > 1 {
                    format!("{} s", self.seconds_value)
                } else {
                    "".into()
                },
                if self.millisecond_value > 1 {
                    format!("{} ms", self.millisecond_value)
                } else {
                    "".into()
                },
            )
        }
    }
}
impl Render for DurationInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let mut items = vec![];
        if self.days_visible {
            items.push(
                NumberInput::new(&self.days_input)
                    .suffix("days")
                    .min_w(px(150.)),
            );
        }
        if self.hours_visible {
            items.push(
                NumberInput::new(&self.hours_input)
                    .suffix("hours")
                    .min_w(px(150.)),
            );
        }
        if self.minutes_visible {
            items.push(
                NumberInput::new(&self.minutes_input)
                    .suffix("mins")
                    .min_w(px(150.)),
            );
        }
        if self.seconds_visible {
            items.push(
                NumberInput::new(&self.seconds_input)
                    .suffix("secs")
                    .min_w(px(150.)),
            );
        }
        if self.millisecond_visible {
            items.push(
                NumberInput::new(&self.millisecond_input)
                    .suffix("ms")
                    .min_w(px(150.)),
            );
        }

        div().h_flex().children(items)
    }
}
impl From<&DurationInput> for u64 {
    fn from(value: &DurationInput) -> Self {
        (value.days_value * DAY_LENGTH)
            + (value.hours_value * HOUR_LENGTH)
            + (value.minutes_value * MINUTE_LENGTH)
            + (value.seconds_value * SECOND_LENGTH)
            + value.millisecond_value
    }
}
impl From<DurationInput> for u64 {
    fn from(value: DurationInput) -> Self {
        Self::from(&value)
    }
}
