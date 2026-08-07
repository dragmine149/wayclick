use crate::{home::Home, writer::Writer};
use gpui::{
    App, AppContext, Bounds, Context, Entity, Global, KeyBinding, ParentElement, SharedString,
    StyleRefinement, Styled, TitlebarOptions, Window, WindowBounds, WindowOptions, actions, px,
    size,
};
use gpui_component::{
    ActiveTheme, Root,
    group_box::{GroupBox, GroupBoxVariants},
    h_flex,
};
use std::{fs, path::PathBuf};
use wayclick_schema::Settings;
pub(crate) mod home;
pub(crate) mod writer;

/// Taken from gpio-component story/lib.rs
fn section(title: impl Into<SharedString>, cx: &mut App) -> GroupBox {
    let title = title.into();
    GroupBox::new()
        .w_full()
        .id(title.clone())
        .outline()
        .title(h_flex().justify_between().w_full().gap_4().child(title))
        .content_style(
            StyleRefinement::default()
                .rounded(cx.theme().radius_lg)
                .overflow_x_hidden()
                .items_center()
                .justify_center(),
        )
}

/// A trait to skip some of the announces of creating a new entity for a struct all the time.
pub(crate) trait GPUIStructHelper
where
    Self: 'static + Sized,
{
    fn view(window: &mut Window, cx: &mut App) -> Entity<Self>
    where
        Self: Sized,
    {
        cx.new(|cx| Self::new(window, cx))
    }
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self;
}

pub trait GlobalExt: Global + Sized {
    fn init_default(cx: &mut App)
    where
        Self: Default,
    {
        cx.set_global(Self::default());
    }
    /// Same as [Writer::get] but returns a clone of the data instead.
    fn get_copy(cx: &App) -> Self
    where
        Self: Clone,
    {
        cx.global::<Self>().clone()
    }
    fn get(cx: &App) -> &Self {
        cx.global::<Self>()
    }
    fn get_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<Self>()
    }
}

pub(crate) fn load_theme(cx: &mut App, theme_name: &SharedString) {
    if let Some(theme) = gpui_component::ThemeRegistry::global(cx)
        .themes()
        .get(theme_name)
        .cloned()
    {
        let glob_theme = gpui_component::Theme::global_mut(cx);
        glob_theme.apply_config(&theme);
        // println!("{:?}", theme.font_family);
        // println!("{:?}", glob_theme.font_family);
        Settings::get_mut(cx).active_theme = theme_name.to_string();
        println!("Loaded new theme: {:?}", theme_name);
    }
}

actions!([Quit]);

pub fn main(config_dir: PathBuf) {
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
                load_theme(cx, &SharedString::from(theme_name));
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

            let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);
            cx.open_window(
                WindowOptions {
                    app_id: Some(env!("CARGO_PKG_NAME").to_string()),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    titlebar: Some(TitlebarOptions {
                        title: Some(format!("wayclick {}", env!("CARGO_PKG_VERSION")).into()),
                        ..Default::default()
                    }),

                    tabbing_identifier: Some(env!("CARGO_PKG_NAME").to_string()),
                    ..Default::default()
                },
                |window, cx| cx.new(|cx| Root::new(Home::view(window, cx), window, cx)),
            )
            .unwrap();
        });
}
