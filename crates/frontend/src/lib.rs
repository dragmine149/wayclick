use crate::home::Home;
use gpui::{AppContext, KeyBinding, SharedString, TitlebarOptions, WindowOptions, actions};
use gpui_component::Root;
use gpui_ext::load_theme;
use std::{fs, path::PathBuf};
use wayclick_schema::{Settings, TransferData};
pub(crate) mod home;
pub(crate) mod writer;

actions!([Quit]);

pub fn main(config_dir: PathBuf, data: TransferData) {
    gpui_platform::application()
        .with_assets(gpui_component_assets::Assets)
        .run(move |cx| {
            gpui_component::init(cx);
            // TODO: Sort out themes.
            gpui_component::Theme::change(gpui_component::ThemeMode::Dark, None, cx);

            writer::init_writers(cx, &config_dir);

            let theme_folder = config_dir.join("Themes");
            if !theme_folder.exists() {
                let _ = fs::create_dir(&theme_folder);
            }

            _ = gpui_component::ThemeRegistry::watch_dir(theme_folder.clone(), cx, move |cx| {
                let theme_name = Settings::get(cx).active_theme.clone();
                if theme_name.is_empty() {
                    return;
                }
                load_theme(cx, &SharedString::from(theme_name), |name, cx| {
                    Settings::get_mut(cx).active_theme = name.to_string();
                });
            });

            // TODO: Customise keybinds? *or at least add more*
            cx.on_app_quit(|cx| {
                Settings::force_save(cx);
                async {}
            })
            .detach();
            cx.bind_keys([KeyBinding::new("secondary-q", Quit, None)]);
            cx.on_action(|_: &Quit, cx| {
                for window in cx.windows() {
                    _ = window.update(cx, |_, window, _| {
                        window.remove_window();
                    })
                }
            });

            cx.open_window(
                WindowOptions {
                    app_id: Some(env!("CARGO_PKG_NAME").to_string()),
                    titlebar: Some(TitlebarOptions {
                        title: Some(
                            format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
                                .into(),
                        ),
                        ..Default::default()
                    }),

                    tabbing_identifier: Some(env!("CARGO_PKG_NAME").to_string()),
                    ..Default::default()
                },
                |window, cx| {
                    let home = Home::view(window, cx, data);
                    cx.new(|cx| Root::new(home, window, cx))
                },
            )
            .unwrap();
        });
}
