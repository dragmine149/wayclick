use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, ReadDir},
    path::PathBuf,
};

/// Custom data struct storing all of the settings.
#[derive(Debug, Serialize, Deserialize, Builder)]
#[builder(setter(into, strip_option), default)]
#[serde(default)]
pub struct Data {
    pub minutes: Option<u64>,
    pub seconds: Option<u64>,
    pub milliseconds: Option<u64>,
    pub initial: Option<u64>,
    pub theme: Option<String>,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            minutes: Some(0),
            seconds: Some(0),
            milliseconds: Some(100),
            initial: Some(0),
            theme: Some("Catppuccin Mocha".into()),
        }
    }
}
impl Data {
    /// Merge the data struct with default data.
    pub fn merge_default(&self) -> Data {
        let default = Data::default();
        self.merge(default)
    }

    /// Forcable verify the data and re-save it.
    /// Yes we don't need to limit it, but it's just nicer to do so.
    pub fn verify(&mut self) {
        if self.minutes.unwrap() > 59 {
            self.minutes = Some(59);
        }
        if self.seconds.unwrap() > 59 {
            self.seconds = Some(59);
        }
        if self.milliseconds.unwrap() > 999 {
            self.milliseconds = Some(999)
        }
        Settings::save_data(self);
    }
}

/// Trait for merging 2 of the same things into a new one.
trait Merge {
    fn merge(&self, other: Self) -> Self;
}
/// A custom trait for making a struct into a sharable object.
trait Sharable {
    /// Convert this struct into a sharable object.
    fn share(&self) -> String;

    /// Load a previously shared string.
    fn load_share(share_string: &str) -> Self;
}
impl Merge for Data {
    fn merge(&self, other: Self) -> Self {
        Self {
            milliseconds: self.milliseconds.or(other.milliseconds),
            seconds: self.seconds.or(other.seconds),
            minutes: self.minutes.or(other.minutes),
            initial: self.initial.or(other.initial),
            theme: self.theme.clone().or(other.theme),
        }
    }
}
fn num_to_char(n: u8) -> Option<char> {
    match n {
        0..=9 => Some((b'0' + n) as char),
        10..=35 => Some((b'A' + (n - 10)) as char),
        36..=59 => Some((b'a' + (n - 36)) as char),
        _ => None,
    }
}
fn char_to_num(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some((c as u8) - b'0'),
        'A'..='Z' => Some((c as u8) - b'A' + 10),
        'a'..='z' => Some((c as u8) - b'a' + 36),
        _ => None,
    }
}
impl Sharable for Data {
    fn share(&self) -> String {
        format!(
            "{:?}{:?}{:04}",
            num_to_char(self.minutes.unwrap_or_default() as u8),
            num_to_char(self.seconds.unwrap_or_default() as u8),
            self.milliseconds.unwrap_or_default()
        )
    }

    fn load_share(share_string: &str) -> Self {
        let mut chars = share_string.chars();
        let minutes = char_to_num(chars.next().unwrap())
            .expect("Invalid char at position 1: Can't convert to minutes");
        let seconds = char_to_num(chars.next().unwrap())
            .expect("Invalid char at position 2: Can't convert to seconds");
        let milliseconds = chars
            .take(4)
            .collect::<String>()
            .parse::<u64>()
            .expect("Failed to convert position 3..6 to milliseconds");
        Self {
            minutes: Some(minutes as u64),
            seconds: Some(seconds as u64),
            milliseconds: Some(milliseconds),
            ..Default::default()
        }
    }
}

/// Default config dir
fn config_dir() -> PathBuf {
    dirs::config_dir().unwrap().join("wayclick")
}
/// Path of settings
pub fn save_path() -> PathBuf {
    config_dir().join("settings.json")
}
/// Path of themes
pub fn theme_dir() -> PathBuf {
    config_dir().join("themes")
}
/// Path of zed themes so we can also use them.
pub fn zed_theme_dir() -> PathBuf {
    dirs::config_dir().unwrap().join("zed/themes")
}

pub struct Settings;
impl Settings {
    /// Load the data from the file system, or the default data if no data exists.
    pub fn load_data() -> Data {
        let path = save_path();
        let mut data = match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str::<Data>(&contents).unwrap_or_default(),
            Err(_) => Data::default(),
        };
        data.verify();
        data
    }

    /// Save data to the file system.
    pub fn save_data(data: &Data) {
        let path = save_path();

        // Load previously stored data to fill in any missing fields
        let stored = Settings::load_data();
        let merged = data.merge(stored);

        // Ensure the config directory exists, then write the merged settings
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let _ = fs::write(
            &path,
            serde_json::to_string_pretty(&merged).unwrap_or_else(|_| "{}".into()),
        );
    }
}
