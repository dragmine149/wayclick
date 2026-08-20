use crate::sections::UpdateSettings;
use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{Disableable, button::Button, h_flex};
use gpui_ext::{GPUIStructHelper, section};

pub struct Controls {}

impl GPUIStructHelper<bool> for Controls {
    fn new(_: &mut Window, _: &mut Context<Self>, _: Option<bool>) -> Self {
        Self {}
    }
}
impl UpdateSettings for Controls {}
impl Render for Controls {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        section("Controls", cx).h_10().child(
            h_flex()
                .h_full()
                .p_2()
                .child(
                    Button::new("start")
                        .p_5()
                        .label("Start")
                        .size_20()
                        .disabled(wayclick_click::is_clicking())
                        .on_click(|_, _, _| {
                            wayclick_click::start_subprocess();
                        }),
                )
                .child(
                    Button::new("stop")
                        .p_5()
                        .label("stop")
                        .size_20()
                        .disabled(!wayclick_click::is_clicking())
                        .on_click(|_, _, _| {
                            wayclick_click::daemon_stop();
                        }),
                ),
        )
    }
}
