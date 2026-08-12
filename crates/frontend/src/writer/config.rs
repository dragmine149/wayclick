use gpui_ext::{Writer, writer::Save};
use wayclick_schema::{Profile, Settings};

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
