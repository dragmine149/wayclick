use crate::sections::{
    button::MouseButton, controls::Controls, delay::Delay, notification::NotificationUI,
    position::Position, repeat::Repeat,
};
use gpui::{Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{
    Root, WindowExt,
    button::{Button, ButtonVariants},
    dialog::{DialogFooter, DialogHeader, DialogTitle},
    h_flex,
    notification::Notification,
    text::markdown,
};
use gpui_ext::{GPUIStructHelper, notify::WeakNotify, thread_to_main_oneshot};
use wayclick_schema::{ServerResponse, TransferData};

pub struct Home {
    delay: Entity<Delay>,
    mouse_button: Entity<MouseButton>,
    repeat: Entity<Repeat>,
    notification: Entity<NotificationUI>,
    position: Entity<Position>,
    controls: Entity<Controls>,
}

impl WeakNotify for Home {}
impl GPUIStructHelper<TransferData> for Home {
    fn new(window: &mut Window, cx: &mut Context<Self>, data: Option<TransferData>) -> Self {
        thread_to_main_oneshot(cx, data.unwrap().rx, async |this, cx, rx| {
            let response = rx.recv().await.unwrap();
            let response = match response {
                Ok(v) => v,
                Err(e) => {
                    _ = Self::weak_notify(
                        &this,
                        Notification::new()
                            .title("Failed to fetch update information")
                            .message(e),
                        cx,
                    );
                    return;
                }
            };

            let res = response.clone();
            let new = response.version;
            let current = env!("CARGO_PKG_VERSION");
            if new != current {
                _ = Self::weak_notify(
                    &this,
                    Self::build_update_notification(current, new.as_str(), res.clone()),
                    cx,
                );
            }
        })
        .detach();

        Self {
            delay: Delay::view(window, cx, None),
            mouse_button: MouseButton::view(window, cx, None),
            repeat: Repeat::view(window, cx, None),
            notification: NotificationUI::view(window, cx, None),
            position: Position::view(window, cx, None),
            controls: Controls::view(window, cx, None),
        }
    }
}

impl Home {
    fn build_update_notification(
        current_ver: &str,
        new_ver: &str,
        changelog: ServerResponse,
    ) -> Notification {
        Notification::new()
            .title("Update Available")
            .message(format!(
                "New version: {}. Current version: {current_ver}",
                &new_ver
            ))
            .autohide(false)
            .action(move |_, _, cx| {
                let changelog = changelog.clone();
                Button::new("View changelog")
                    .secondary()
                    .label("View Changelog")
                    .on_click(cx.listener(move |this, _, window, cx| {
                        let changelog = changelog.clone();
                        window.open_dialog(cx, move |dialog, _, _| {
                            let changelog = changelog.clone();
                            dialog.content(move |content, _, _| {
                                content
                                    .child(
                                        DialogHeader::new().child(DialogTitle::new().child(
                                            format!("Changelog for {}", &changelog.version),
                                        )),
                                    )
                                    .child(markdown(&changelog.release_notes))
                                    .child(
                                        DialogFooter::new()
                                            .justify_center()
                                            .child(
                                                Button::new("close")
                                                    .flex()
                                                    .outline()
                                                    .label("Update Later")
                                                    .on_click(|_, window, cx| {
                                                        window.close_dialog(cx)
                                                    }),
                                            )
                                            .child(
                                                Button::new("Open Github")
                                                    .flex_1()
                                                    .primary()
                                                    .label("Open Github")
                                                    .on_click(|_, window, cx| {
                                                        window.close_dialog(cx);
                                                        cx.open_url("https://wayclick.dragmine.me");
                                                    }),
                                            ),
                                    )
                            })
                        });
                        println!("Viewing changelog");
                        this.dismiss(window, cx);
                    }))
            })
    }
}

impl Render for Home {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx);
        let dialog_layer = Root::render_dialog_layer(window, cx);

        div()
            .size_full()
            .p_4()
            .child(self.delay.clone())
            .child(
                h_flex()
                    .w_full()
                    .child(self.mouse_button.clone())
                    .child(self.repeat.clone())
                    .child(self.notification.clone()),
            )
            .child(self.position.clone())
            .child(self.controls.clone())
            .children(notification_layer)
            .children(dialog_layer)
    }
}
