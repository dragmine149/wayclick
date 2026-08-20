use gpui::{Context, IntoElement, ParentElement, Render, Window};
use gpui_component::{
    button::Button,
    menu::{DropdownMenu, PopupMenuItem},
};
use gpui_ext::{GPUIStructHelper, Writer, section};
use strum::IntoEnumIterator;

use crate::{sections::UpdateSettings, writer::config::Settings};

pub struct MouseButton {}

impl GPUIStructHelper<bool> for MouseButton {
    fn new(_: &mut Window, _: &mut Context<Self>, _: Option<bool>) -> Self {
        Self {}
    }
}
impl UpdateSettings for MouseButton {}
impl Render for MouseButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let profile = Settings::get(cx).get_default_profile();
        let view = cx.entity();

        section("Button", cx).child(
            Button::new("mouse_button")
                .label(format!("{} mouse button", profile.click))
                .dropdown_menu(move |mut menu, win, _| {
                    for button in wayclick_schema::MouseButton::iter() {
                        menu = menu.item(PopupMenuItem::new(button.to_string()).on_click(
                            win.listener_for(&view, move |this, _, _, cx| {
                                this.update_settings(
                                    |profile| {
                                        profile.click = button.clone();
                                    },
                                    cx,
                                );
                            }),
                        ));
                    }
                    menu
                }),
        )
    }
}
