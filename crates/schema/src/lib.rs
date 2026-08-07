use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{create_dir_all, read, write},
    path::PathBuf,
};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Serialize, Deserialize, Clone, EnumIter, Display)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

/// Information about a specific profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Which mouse button to click
    pub click: MouseButton,
    /// How many times to click for (0 = inf)
    pub repeat: usize,
    /// How long to wait before we start.
    pub initial: u64,
    /// How long to wait in between each click (in ms)
    pub delay: u64,
    /// Where to click on the screen.
    pub position: Option<(u16, u16)>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            click: MouseButton::Left,
            repeat: 0,
            initial: 0,
            delay: 100,
            position: None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Settings {
    /// List of all profiles
    pub profiles: HashMap<String, Profile>,
    /// The default profile to use if not specified.
    ///
    /// Will default to last profile used.
    pub default_profile: String,
    /// The theme to use for the app
    pub active_theme: String,
}

impl Settings {
    /// Load the data from the file.
    ///
    /// If we don't have the file, create a default one.
    /// If we don't have the dir, create one.
    /// If we failed to create something, don't care not our issue to deal with.
    pub fn load() -> Self {
        let path = dir().join("Settings.json");
        let Ok(data) = read(path) else {
            let default = Self::default();
            default.save();
            return default;
        };
        serde_json::from_slice(&data).expect("Json data got maulformed. Please fix.")
    }

    /// Save the settings to disk.
    pub fn save(&self) {
        let path = dir().join("settings.json");
        write(path, serde_json::to_vec(self).expect("This shouldn't fail"))
            .expect("Unable to write data")
    }

    pub fn get_profile(&self, profile: Option<impl Into<String>>) -> Profile {
        let profile_name = profile
            .map(|p| p.into())
            .unwrap_or(self.default_profile.clone());
        self.profiles
            .get(&profile_name)
            .map(|p| p.to_owned())
            .unwrap()
    }

    pub fn get_default_profile(&self) -> Profile {
        self.profiles
            .get(&self.default_profile)
            .map(|p| p.to_owned())
            .unwrap()
    }

    pub fn get_profile_mut(&mut self, profile: Option<impl Into<String>>) -> Option<&mut Profile> {
        let profile_name = profile
            .map(|p| p.into())
            .unwrap_or(self.default_profile.clone());
        self.profiles.get_mut(&profile_name)
    }
    pub fn get_default_profile_mut(&mut self) -> &mut Profile {
        self.profiles.get_mut(&self.default_profile).unwrap()
    }
}

pub fn dir() -> PathBuf {
    let dir = dirs::data_dir().unwrap().join("wayclick");
    create_dir_all(&dir).expect("Failed to create directory");
    dir
}
