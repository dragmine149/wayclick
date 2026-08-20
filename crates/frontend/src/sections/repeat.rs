use crate::{sections::UpdateSettings, writer::config::Settings};
use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Window};
use gpui_component::{
    Disableable,
    input::{InputState, NumberInput},
    radio::{Radio, RadioGroup},
};
use gpui_ext::{GPUIStructHelper, Writer, section};

pub struct Repeat {
    repeat: Entity<InputState>,
}

impl GPUIStructHelper<bool> for Repeat {
    fn new(window: &mut Window, cx: &mut Context<Self>, _: Option<bool>) -> Self {
        Self {
            repeat: cx.new(|cx| {
                InputState::new(window, cx)
                    .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v >= 1))
                    .default_value("1")
            }),
        }
    }
}
impl UpdateSettings for Repeat {}
impl Render for Repeat {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let profile = Settings::get(cx).get_default_profile();

        section("Repeat", cx).child(
            RadioGroup::vertical("repeat")
                .child("Infinite")
                .child(
                    Radio::new("count")
                        .label("Specific number")
                        .child(NumberInput::new(&self.repeat).disabled(profile.repeat == 0)),
                )
                .selected_index(Some(match profile.repeat == 0 {
                    true => 0,
                    false => 1,
                }))
                .on_click(cx.listener(|view, selected_index: &usize, _, cx| {
                    let rep_value = match selected_index {
                        0 => 0,
                        _ => view
                            .repeat
                            .read_with(cx, |v, _| v.value().parse::<usize>())
                            .unwrap(),
                    };
                    view.update_settings(|profile| profile.repeat = rep_value, cx);

                    cx.notify();
                })),
        )
    }
}
