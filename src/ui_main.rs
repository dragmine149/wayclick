use crate::{
    storage::{Settings, theme_dir},
    ui::ClickUI,
};
use gpui::{
    AppContext, Application, Bounds, SharedString, TitlebarOptions, WindowBounds, WindowOptions,
    px, size,
};
use gpui_component::{Root, Theme, ThemeRegistry, TitleBar};
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

        let bounds = Bounds::centered(None, size(px(500.), px(500.)), cx);
        // actually open gpui.
        cx.spawn(async move |cx| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("WayClicker".into()),
                    ..Default::default()
                }),
                app_id: Some("wayclicker".into()),
                ..Default::default()
            };

            let window = cx
                .open_window(options, |window, cx| {
                    let view = ClickUI::view(window, cx);

                    // This first level on the window, should be a Root.
                    cx.new(|cx| Root::new(view, window, cx))
                })
                .expect("Failed to open window");

            window
                .update(cx, |_, window, _| {
                    window.set_window_title("WayClicker");
                })
                .expect("Failed to update window after launch");

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
