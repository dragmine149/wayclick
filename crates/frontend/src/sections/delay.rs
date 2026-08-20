use crate::{sections::UpdateSettings, writer::config::Settings};
use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Subscription, Window,
};
use gpui_component::{
    h_flex,
    input::{InputEvent, InputState, NumberInput},
    v_flex,
};
use gpui_ext::{GPUIStructHelper, Writer, section};

pub struct Delay {
    hour: Entity<InputState>,
    mins: Entity<InputState>,
    secs: Entity<InputState>,
    mili: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}
impl GPUIStructHelper<bool> for Delay {
    fn new(window: &mut Window, cx: &mut Context<Self>, _: Option<bool>) -> Self {
        let profile = Settings::get(cx).get_default_profile();
        let ms = profile.delay;
        let hour = ms / 3_600_000;
        let min = (ms % 3_600_000) / 60_000;
        let sec = (ms % 3_600_000 % 60_000) / 1000;
        let ms = ms % 3_600_000 % 60_000 % 1000;

        let hour = cx.new(|cx| {
            let mut is = InputState::new(window, cx)
                .placeholder("hours")
                .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v <= 24))
                .default_value("0");
            is.set_value(hour.to_string(), window, cx);
            is
        });
        let mins = cx.new(|cx| {
            let mut is = InputState::new(window, cx)
                .placeholder("minutes")
                .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v <= 60))
                .default_value("0");
            is.set_value(min.to_string(), window, cx);
            is
        });
        let secs = cx.new(|cx| {
            let mut is = InputState::new(window, cx)
                .placeholder("seconds")
                .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v <= 60))
                .default_value("0");
            is.set_value(sec.to_string(), window, cx);
            is
        });
        let mili = cx.new(|cx| {
            let mut is = InputState::new(window, cx)
                .placeholder("milliseconds")
                .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v <= 1000))
                .default_value("100");
            is.set_value(ms.to_string(), window, cx);
            is
        });

        let subscriptions = vec![
            cx.subscribe(&hour, |v, _, e: &InputEvent, cx| {
                if matches!(e, InputEvent::Change) {
                    v.update_delay(cx)
                }
            }),
            cx.subscribe(&mins, |v, _, e: &InputEvent, cx| {
                if matches!(e, InputEvent::Change) {
                    v.update_delay(cx)
                }
            }),
            cx.subscribe(&secs, |v, _, e: &InputEvent, cx| {
                if matches!(e, InputEvent::Change) {
                    v.update_delay(cx)
                }
            }),
            cx.subscribe(&mili, |v, _, e: &InputEvent, cx| {
                if matches!(e, InputEvent::Change) {
                    v.update_delay(cx)
                }
            }),
        ];

        Self {
            hour,
            mins,
            secs,
            mili,
            _subscriptions: subscriptions,
        }
    }
}
impl UpdateSettings for Delay {}

impl Delay {
    fn update_delay(&self, cx: &mut Context<Self>) {
        let h = self
            .hour
            .read_with(cx, |v, _| v.value().parse::<u64>())
            .unwrap();
        let m = self
            .mins
            .read_with(cx, |v, _| v.value().parse::<u64>())
            .unwrap();
        let s = self
            .secs
            .read_with(cx, |v, _| v.value().parse::<u64>())
            .unwrap();
        let ms = self
            .mili
            .read_with(cx, |v, _| v.value().parse::<u64>())
            .unwrap();
        let time = (h * 3_600_000) + (m * 60_000) + (s * 1000) + ms;
        self.update_settings(|profile| profile.delay = time, cx);
    }
}

impl Render for Delay {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        section("Delay", cx).child(
            h_flex()
                .w_full()
                .child(
                    v_flex()
                        .w_full()
                        .child("Hours")
                        .child(NumberInput::new(&self.hour)),
                )
                .child(
                    v_flex()
                        .w_full()
                        .child("Minutes")
                        .child(NumberInput::new(&self.mins)),
                )
                .child(
                    v_flex()
                        .w_full()
                        .child("Seconds")
                        .child(NumberInput::new(&self.secs)),
                )
                .child(
                    v_flex()
                        .w_full()
                        .child("Milliseconds")
                        .child(NumberInput::new(&self.mili)),
                ),
        )
    }
}
