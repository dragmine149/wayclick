use crate::writer::config::Settings;
use gpui::Context;
use gpui_ext::writer::Writer;
use wayclick_schema::Profile;

pub(crate) mod button;
pub(crate) mod controls;
pub(crate) mod delay;
pub(crate) mod notification;
pub(crate) mod position;
pub(crate) mod repeat;

pub trait UpdateSettings {
    fn update_settings<F>(&self, update_fn: F, cx: &mut Context<Self>)
    where
        Self: Sized,
        F: Fn(&mut Profile),
    {
        update_fn(Settings::get_mut(cx).get_default_profile_mut())
    }
}
