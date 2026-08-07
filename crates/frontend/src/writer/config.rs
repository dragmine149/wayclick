use crate::writer::{Save, Writer};
use wayclick_schema::{Profile, Settings};

impl Writer for Settings {
    fn get_name() -> &'static str {
        "Settings"
    }
}
impl Save for Settings {
    fn pre_save(&mut self) {}
    fn post_load(&mut self) {
        if self.profiles.len() == 0 {
            self.default_profile = "default".to_string();
            self.profiles
                .insert(self.default_profile.clone(), Profile::default());
        }
    }
}
