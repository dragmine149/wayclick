use crate::writer::config::Settings;
use gpui::App;
use gpui_ext::Writer;
use std::path::Path;

pub mod config;

pub(crate) fn init_writers(cx: &mut App, path: &Path) {
    Settings::init(cx, path);
}
