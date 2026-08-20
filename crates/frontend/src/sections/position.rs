use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{
    Disableable,
    button::Button,
    h_flex,
    input::{InputState, NumberInput},
    radio::{Radio, RadioGroup},
    v_flex,
};
use gpui_ext::{GPUIStructHelper, Writer, section};

use crate::{sections::UpdateSettings, writer::config::Settings};

pub struct Position {
    x_pos: Entity<InputState>,
    y_pos: Entity<InputState>,
}

impl GPUIStructHelper<bool> for Position {
    fn new(window: &mut Window, cx: &mut Context<Self>, _: Option<bool>) -> Self {
        let profile = Settings::get(cx).get_default_profile();
        Self {
            x_pos: cx.new(|cx| {
                let mut is = InputState::new(window, cx)
                    .validate(|v, _| v.parse::<u64>().is_ok())
                    .default_value("0");
                is.set_value(profile.position.map_or(0, |v| v.0).to_string(), window, cx);
                is
            }),
            y_pos: cx.new(|cx| {
                let mut is = InputState::new(window, cx)
                    .validate(|v, _| v.parse::<u64>().is_ok())
                    .default_value("0");
                is.set_value(profile.position.map_or(0, |v| v.1).to_string(), window, cx);
                is
            }),
        }
    }
}
impl UpdateSettings for Position {}
impl Render for Position {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let profile = Settings::get(cx).get_default_profile();

        section("Position", cx).child(
            RadioGroup::horizontal("position")
                .child("Cursor Position")
                .child(
                    Radio::new("pos").w_full().label("Specific position").child(
                        h_flex()
                            .w_full()
                            .child(
                                v_flex()
                                    .p_2()
                                    .w_full()
                                    .child(
                                        NumberInput::new(&self.x_pos)
                                            .prefix("x:")
                                            .w_full()
                                            .disabled(profile.position.is_none()),
                                    )
                                    .child(
                                        NumberInput::new(&self.y_pos)
                                            .prefix("y:")
                                            .w_full()
                                            .disabled(profile.position.is_none()),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .p_2()
                                    .child(
                                        Button::new("Set")
                                            .disabled(true)
                                            .tooltip(
                                                "Currently disabled due to issues with wayland.",
                                            )
                                            .label("Set to current Position")
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                let pos = wayclick_click::get_pos();
                                                this.x_pos.update(cx, |is, cx| {
                                                    is.set_value(pos.0.to_string(), window, cx)
                                                });
                                                this.y_pos.update(cx, |is, cx| {
                                                    is.set_value(pos.1.to_string(), window, cx)
                                                });
                                                this.update_settings(
                                                    |profile| {
                                                        profile.position =
                                                            Some((pos.0 as u16, pos.1 as u16));
                                                    },
                                                    cx,
                                                )
                                            })),
                                    )
                                    .child(
                                        Button::new("test")
                                            .disabled(profile.position.is_none())
                                            .label("Test (Move mouse to pos)")
                                            .on_click(move |_, _, _| {
                                                wayclick_click::move_mouse(
                                                    profile.position.unwrap(),
                                                );
                                            }),
                                    ),
                            ),
                    ),
                )
                .selected_index(Some(match profile.position.is_some() {
                    false => 0,
                    true => 1,
                }))
                .on_click(cx.listener(|view, selected_index: &usize, _, cx| {
                    let pos = match selected_index {
                        0 => None,
                        _ => Some((
                            view.x_pos
                                .read_with(cx, |v, _| v.value().parse::<u64>())
                                .unwrap() as u16,
                            view.y_pos
                                .read_with(cx, |v, _| v.value().parse::<u64>())
                                .unwrap() as u16,
                        )),
                    };
                    view.update_settings(|profile| profile.position = pos, cx);
                    cx.notify();
                })),
        )
    }
}
