use gpui_ext::{Writer, writer::Save};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use wayclick_schema::Profile;

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl Default for Settings {
    fn default() -> Self {
        let mut default_hash = HashMap::new();
        default_hash.insert("default".to_string(), Profile::default());
        Self {
            profiles: default_hash,
            default_profile: "default".to_string(),
            active_theme: "".to_string(),
        }
    }
}

impl Settings {
    pub fn get_profile(&self, profile: Option<impl Into<String>>) -> Profile {
        let profile_name = profile
            .map(|p| p.into())
            .unwrap_or(self.default_profile.clone());
        self.profiles
            .get(&profile_name)
            .map(|p| p.to_owned())
            .unwrap_or_else(|| panic!("Invalid profile name ({profile_name}) or missing profile"))
    }

    pub fn get_default_profile(&self) -> Profile {
        self.profiles
            .get(&self.default_profile)
            .map(|p| p.to_owned())
            .unwrap_or_else(|| {
                panic!(
                    "Invalid profile name ({}) or missing profile",
                    self.default_profile
                )
            })
    }

    pub fn get_profile_mut(&mut self, profile: Option<impl Into<String>>) -> Option<&mut Profile> {
        let profile_name = profile
            .map(|p| p.into())
            .unwrap_or(self.default_profile.clone());
        self.profiles.get_mut(&profile_name)
    }
    pub fn get_default_profile_mut(&mut self) -> &mut Profile {
        self.profiles
            .get_mut(&self.default_profile)
            .unwrap_or_else(|| {
                panic!(
                    "Invalid profile name ({}) or missing profile",
                    self.default_profile
                )
            })
    }
}

impl Writer for Settings {
    fn get_name() -> &'static str {
        "Settings"
    }
}
impl Save for Settings {
    fn pre_save(&mut self) {}
    fn post_load(&mut self) {
        if self.profiles.is_empty() {
            self.default_profile = "default".to_string();
            self.profiles
                .insert(self.default_profile.clone(), Profile::default());
        }
    }
}
