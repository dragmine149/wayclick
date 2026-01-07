use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(default)]
    pub minutes: Option<u64>,
    #[serde(default)]
    pub seconds: Option<u64>,
    #[serde(default)]
    pub milliseconds: Option<u64>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            minutes: Some(0),
            seconds: Some(0),
            milliseconds: Some(100),
        }
    }
}

trait Merge {
    type Other;
    fn merge(&self, other: Self::Other) -> Self;
}

impl Merge for Data {
    type Other = Data;

    fn merge(&self, other: Self::Other) -> Self {
        Self {
            milliseconds: self.milliseconds.or(other.milliseconds),
            seconds: self.seconds.or(other.seconds),
            minutes: self.minutes.or(other.minutes),
        }
    }
}

fn save_dir() -> PathBuf {
    dirs::config_dir().unwrap().join("wayclick.json")
}

pub struct Settings;
impl Settings {
    pub fn load_data() -> Data {
        let path = save_dir();
        match fs::read_to_string(&path) {
            Ok(contents) => serde_json::from_str::<Data>(&contents).unwrap_or_default(),
            Err(_) => Data::default(),
        }
    }

    pub fn save_data(data: Data) {
        let path = save_dir();

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

    /// Produce a sharable string representation of the settings.
    /// Currently JSON, kept intentionally simple so it's easy to extend.
    pub fn share(data: &Data) -> String {
        serde_json::to_string(data).unwrap_or_default()
    }
}
