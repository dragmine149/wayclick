use gpui::{App, Global, Task};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::{io::Write, path::Path, sync::Arc, time::Duration};
use wayclick_schema::Settings;

pub mod config;

pub(crate) fn init_writers(cx: &mut App, path: &Path) {
    Settings::init(cx, path);
}
