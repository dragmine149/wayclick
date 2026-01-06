use crate::ui::ClickUI;
use gpui::*;
use gpui_component::*;
use gpui_component_assets::Assets;

pub fn ui_main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

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
