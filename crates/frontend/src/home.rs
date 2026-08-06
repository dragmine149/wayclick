use crate::{GPUIStructHelper, section};
use gpui::{
    Action, AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString,
    StyleRefinement, Styled, Window, div,
};
use gpui_component::{
    Disableable,
    button::{Button, DropdownButton},
    h_flex,
    input::{InputState, NumberInput},
    menu::{DropdownMenu, PopupMenuItem},
    radio::{Radio, RadioGroup},
    setting::SettingField,
    v_flex,
};
use strum::IntoEnumIterator;

pub struct Home {
    hour: Entity<InputState>,
    mins: Entity<InputState>,
    secs: Entity<InputState>,
    mili: Entity<InputState>,
    repeat: Entity<InputState>,
    x_pos: Entity<InputState>,
    y_pos: Entity<InputState>,

    repeat_value: bool,
    position: bool,
}

impl Home {}

impl GPUIStructHelper for Home {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            hour: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("hours")
                    .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v <= 24))
            }),
            mins: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("minutes")
                    .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v <= 60))
            }),
            secs: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("seconds")
                    .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v <= 60))
            }),
            mili: cx.new(|cx| {
                InputState::new(window, cx)
                    .placeholder("milliseconds")
                    .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v <= 1000))
            }),
            repeat: cx.new(|cx| {
                InputState::new(window, cx)
                    .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v >= 1))
                    .default_value("1")
            }),
            repeat_value: false,
            x_pos: cx
                .new(|cx| InputState::new(window, cx).validate(|v, _| v.parse::<u64>().is_ok())),
            y_pos: cx
                .new(|cx| InputState::new(window, cx).validate(|v, _| v.parse::<u64>().is_ok())),
            position: false,
        }
    }
}

impl Render for Home {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .p_4()
            .child(
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
                ),
            )
            .child(
                h_flex()
                    .w_full()
                    .child(
                        section("Button", cx).child(
                            DropdownButton::new("mb")
                                .button(Button::new("mb").label(format!(
                                    "{} mouse button",
                                    wayclick_schema::MouseButton::Left
                                )))
                                .dropdown_menu(|mut menu, win, cx| {
                                    for button in wayclick_schema::MouseButton::iter() {
                                        menu = menu.item(PopupMenuItem::new(button.to_string()));
                                    }
                                    menu
                                }),
                        ),
                    )
                    .child(
                        section("Repeat", cx).child(
                            RadioGroup::vertical("repeat")
                                .child("Infinite")
                                .child(Radio::new("count").label("Specific number").child(
                                    NumberInput::new(&self.repeat).disabled(self.repeat_value),
                                ))
                                .selected_index(Some(match self.repeat_value {
                                    false => 0,
                                    true => 1,
                                }))
                                .on_click(cx.listener(|view, selected_index: &usize, _, cx| {
                                    view.repeat_value = *selected_index != 0;
                                    cx.notify()
                                })),
                        ),
                    ),
            )
            .child(
                section("Position", cx).child(
                    RadioGroup::horizontal("position")
                        .child("Cursor Position")
                        .child(
                            Radio::new("pos").w_full().label("Specific position").child(
                                v_flex()
                                    .w_full()
                                    .child(
                                        NumberInput::new(&self.x_pos)
                                            .prefix("x:")
                                            .w_full()
                                            .disabled(!self.position),
                                    )
                                    .child(
                                        NumberInput::new(&self.y_pos)
                                            .prefix("y:")
                                            .w_full()
                                            .disabled(!self.position),
                                    ),
                            ),
                        )
                        .selected_index(Some(match self.position {
                            false => 0,
                            true => 1,
                        }))
                        .on_click(cx.listener(|view, selected_index: &usize, _, cx| {
                            view.position = *selected_index != 0;
                            cx.notify()
                        })),
                ),
            )
            .child(
                section("Controls", cx).h_10().child(
                    h_flex()
                        .h_full()
                        .child(Button::new("start").p_2().label("Start").h_3())
                        .child(Button::new("stop").p_2().label("stop").h_3()),
                ),
            )
    }
}
