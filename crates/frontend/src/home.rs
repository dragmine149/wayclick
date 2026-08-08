use crate::{section, thread_to_main, writer::Writer};
use gpui::{
    AnyWindowHandle, App, AppContext, AsyncApp, Context, Entity, IntoElement, ParentElement,
    Render, Styled, Subscription, WeakEntity, Window, div,
};
use gpui_component::{
    Disableable, Root, WindowExt,
    button::{Button, ButtonVariants},
    h_flex,
    input::{InputEvent, InputState, NumberInput},
    menu::{DropdownMenu, PopupMenuItem},
    notification::Notification,
    radio::{Radio, RadioGroup},
    v_flex,
};
use strum::IntoEnumIterator;
use wayclick_schema::{Profile, Settings, TransferData};

pub struct Home {
    hour: Entity<InputState>,
    mins: Entity<InputState>,
    secs: Entity<InputState>,
    mili: Entity<InputState>,
    repeat: Entity<InputState>,
    x_pos: Entity<InputState>,
    y_pos: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
    main_window: AnyWindowHandle,
}

impl Home {}

impl Home {
    pub fn view(window: &mut Window, cx: &mut App, data: TransferData) -> Entity<Self> {
        cx.new(|cx| Self::new(window, cx, data))
    }
    fn new(window: &mut Window, cx: &mut Context<Self>, data: TransferData) -> Self {
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

        thread_to_main(cx, data.rx, async move |this, cx, rx| {
            let new = rx.recv().await.unwrap().version;
            let current = env!("CARGO_PKG_VERSION");
            if new != current {
                _ = Self::weak_notify(
                    &this,
                    Notification::new()
                        .title("Update Available")
                        .message(format!("New version: {new}. Current version: {current}"))
                        .autohide(false)
                        .action(|_, _, cx| {
                            Button::new("Open github")
                                .primary()
                                .label("Open Github")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    cx.open_url("https://wayclick.dragmine.me");
                                    this.dismiss(window, cx);
                                }))
                        }),
                    cx,
                );
            }
        })
        .detach();

        Self {
            hour,
            mins,
            secs,
            mili,
            repeat: cx.new(|cx| {
                InputState::new(window, cx)
                    .validate(|v, _| v.parse::<u64>().is_ok_and(|v| v >= 1))
                    .default_value("1")
            }),
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
            _subscriptions: subscriptions,
            main_window: window.window_handle(),
        }
    }
}

impl Home {
    /// even even more shorthand for notification.
    ///
    /// # Usage
    /// ```rs
    /// let _ = Self::weak_notify(this, Notification::new(), cx);
    /// ```
    fn weak_notify(
        this: &WeakEntity<Self>,
        notification: Notification,
        cx: &mut AsyncApp,
    ) -> anyhow::Result<()> {
        this.update(cx, |this, cx| this.notify(notification, cx))?
    }

    /// Shorthand for notification, saves repeating it a bit.
    fn notify(&mut self, notification: Notification, cx: &mut Context<Self>) -> anyhow::Result<()> {
        cx.update_window(self.main_window, |_, win, cx| {
            win.push_notification(notification, cx);
        })
    }

    fn update_settings<F>(&self, update_fn: F, cx: &mut Context<Self>)
    where
        F: Fn(&mut Profile),
    {
        update_fn(Settings::get_mut(cx).get_default_profile_mut())
    }
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

impl Render for Home {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let notification_layer = Root::render_notification_layer(window, cx);

        let view = cx.entity();
        let profile = Settings::get(cx).get_default_profile();
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
                            Button::new("mouse_button")
                                .label(format!("{} mouse button", profile.click))
                                .dropdown_menu(move |mut menu, win, _| {
                                    for button in wayclick_schema::MouseButton::iter() {
                                        menu = menu.item(
                                            PopupMenuItem::new(button.to_string()).on_click(
                                                win.listener_for(&view, move |this, _, _, cx| {
                                                    this.update_settings(
                                                        |profile| {
                                                            profile.click = button.clone();
                                                        },
                                                        cx,
                                                    );
                                                }),
                                            ),
                                        );
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
                                    NumberInput::new(&self.repeat).disabled(profile.repeat == 0),
                                ))
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
                        ),
                    ),
            )
            .child(
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
                                                    .disabled(profile.position.is_none())
                                                    .label("Set to current Position")
                                                    .on_click(cx.listener(
                                                        |this, _, window, cx| {
                                                            let pos = wayclick_click::get_pos();
                                                            this.x_pos.update(cx, |is, cx| {
                                                                is.set_value(
                                                                    pos.0.to_string(),
                                                                    window,
                                                                    cx,
                                                                )
                                                            });
                                                            this.y_pos.update(cx, |is, cx| {
                                                                is.set_value(
                                                                    pos.1.to_string(),
                                                                    window,
                                                                    cx,
                                                                )
                                                            });
                                                            this.update_settings(
                                                                |profile| {
                                                                    profile.position = Some((
                                                                        pos.0 as u16,
                                                                        pos.1 as u16,
                                                                    ));
                                                                },
                                                                cx,
                                                            )
                                                        },
                                                    )),
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
                ),
            )
            .child(
                section("Controls", cx).h_10().child(
                    h_flex()
                        .h_full()
                        .child(
                            Button::new("start")
                                .p_2()
                                .label("Start")
                                .h_3()
                                .disabled(wayclick_click::is_clicking())
                                .on_click(|_, _, _| {
                                    wayclick_click::start_subprocess();
                                }),
                        )
                        .child(
                            Button::new("stop")
                                .p_2()
                                .label("stop")
                                .h_3()
                                .disabled(!wayclick_click::is_clicking())
                                .on_click(|_, _, _| {
                                    wayclick_click::daemon_stop();
                                }),
                        ),
                ),
            )
            .children(notification_layer)
    }
}
