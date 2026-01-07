use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(default)]
    minutes: Option<u64>,
    #[serde(default)]
    seconds: Option<u64>,
    #[serde(default)]
    milliseconds: Option<u64>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            minutes: 0,
            seconds: 0,
            milliseconds: 100,
        }
    }
}

fn save_dir() -> PathBuf {
    dirs::config_dir().unwrap().join("wayclick.json")
}

pub struct Settings;
impl Settings {
    pub fn load_data() -> Data {
        serde_json::from_str::<Data>(&fs::read_to_string(save_dir()).unwrap_or("{}".into()))
            .unwrap_or_default()
    }

    pub fn save_data(data: Data) {
        let stored_data = Settings::load_data();
    }
}
