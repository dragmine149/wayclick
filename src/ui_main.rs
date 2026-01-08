use crate::{
    storage::{Settings, theme_dir},
    ui::ClickUI,
};
use gpui::{AppContext, Application, SharedString, TitlebarOptions, WindowOptions};
use gpui_component::{Root, Theme, ThemeRegistry};
use gpui_component_assets::Assets;

/// The main UI of the app, built upon zed gpui.
pub fn ui_main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        // load the theme, thankfully we don't really have to make them.
        // TODO: Move this into it's own special location so that we can read the `zed_theme_dir()` as well as change it in the ui.
        let theme_name = SharedString::from(Settings::load_data().theme.unwrap_or_default());
        if let Err(err) = ThemeRegistry::watch_dir(theme_dir(), cx, move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
                Theme::global_mut(cx).apply_config(&theme);
                cx.refresh_windows();
            }
        }) {
            eprintln!("Failed to watch themes directory: {}", err);
        }

        // actually open gpui.
        cx.spawn(async move |cx| {
            let options = WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("WayClicker".into()),
                    ..Default::default()
                }),
                app_id: Some("wayclicker".into()),
                ..Default::default()
            };

            cx.open_window(options, |window, cx| {
                let view = ClickUI::view(window, cx);

                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
