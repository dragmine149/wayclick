use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{
    button::Button,
    menu::{DropdownMenu, PopupMenuItem},
    text::markdown,
    v_flex,
};
use gpui_ext::{GPUIStructHelper, Writer, section};
use strum::IntoEnumIterator;
use wayclick_schema::NotificationOption;

use crate::{sections::UpdateSettings, writer::config::Settings};

pub struct NotificationUI {}

impl GPUIStructHelper<bool> for NotificationUI {
    fn new(_: &mut Window, _: &mut Context<Self>, _: Option<bool>) -> Self {
        Self {}
    }
}
impl UpdateSettings for NotificationUI {}
impl Render for NotificationUI {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let profile = Settings::get(cx).get_default_profile();

        let viewa = cx.entity();
        let viewb = cx.entity();
        let viewc = cx.entity();

        section("Notification", cx).child(
            v_flex()
                .child("When to show notifications.")
                .child(
                    Button::new("start")
                        .label(format!("On Start: {}", profile.notification.started))
                        .dropdown_menu(move |mut menu, win, _| {
                            for option in NotificationOption::iter() {
                                menu = menu.item(PopupMenuItem::new(option.to_string()).on_click(
                                    win.listener_for(&viewa, move |this, _, _, cx| {
                                        this.update_settings(
                                            |profile| profile.notification.started = option.clone(),
                                            cx,
                                        );
                                    }),
                                ))
                            }
                            menu
                        }),
                )
                .child(
                    Button::new("active")
                        .label(format!("Active: {}", profile.notification.active))
                        .dropdown_menu(move |mut menu, win, _| {
                            for option in NotificationOption::iter() {
                                menu = menu.item(PopupMenuItem::new(option.to_string()).on_click(
                                    win.listener_for(&viewb, move |this, _, _, cx| {
                                        this.update_settings(
                                            |profile| profile.notification.active = option.clone(),
                                            cx,
                                        );
                                    }),
                                ))
                            }
                            menu
                        }),
                )
                .child(
                    Button::new("stop")
                        .label(format!("On Stopped: {}", profile.notification.stopped))
                        .dropdown_menu(move |mut menu, win, _| {
                            for option in NotificationOption::iter() {
                                menu = menu.item(PopupMenuItem::new(option.to_string()).on_click(
                                    win.listener_for(&viewc, move |this, _, _, cx| {
                                        this.update_settings(
                                            |profile| profile.notification.stopped = option.clone(),
                                            cx,
                                        );
                                    }),
                                ))
                            }
                            menu
                        }),
                )
                .child(
                    markdown(
                        "- None: Don't show the notification at all
- HistoryTimeout: Send the notification to history after 1 second
- CloseTimeout: Close the notification after 1 second",
                    )
                    .pt_2(),
                ),
        )
    }
}
