use std::thread::JoinHandle;

use notify_rust::{Notification, NotificationHandle};
use wayclick_schema::NotificationOption;

pub struct NotifyHandler {
    active_config: Option<NotificationOption>,
    active_handle: Option<NotificationHandle>,
    handles: Vec<JoinHandle<()>>,
}

impl Default for NotifyHandler {
    fn default() -> Self {
        Self {
            active_config: None,
            active_handle: None,
            handles: vec![],
        }
    }
}

impl NotifyHandler {
    fn close_notification(&mut self, handle: NotificationHandle) {
        self.handles.push(std::thread::spawn(move || {
            // Written by T3 Chat
            std::thread::sleep(std::time::Duration::from_secs(1));
            handle.close();
        }));
    }

    fn manage_notification(&mut self, data: NotificationOption, notification: &mut Notification) {
        match data {
            NotificationOption::None => return,
            NotificationOption::HistoryTimeout => {
                // we don't care if it shows or not that much.
                _ = notification.timeout(1000).show();
            }
            NotificationOption::CloseTimeout => {
                let handle = notification.timeout(1000).show();
                if let Ok(handle) = handle {
                    self.close_notification(handle);
                }
            }
        }
    }

    /// Display the start notification
    pub fn start(&mut self, data: NotificationOption) {
        let mut noti = Notification::new();
        let notification = noti.summary("Wayclick").body("Autoclicker started");
        self.manage_notification(data, notification);
    }

    /// Display the stop notification. Will also stop the active notification at the same time.
    pub fn stop(&mut self, data: NotificationOption) {
        let mut noti = Notification::new();
        let notification = noti.summary("Wayclick").body("Autoclicker stopped");
        self.manage_notification(data, notification);
        self.active_stop();
    }

    /// Display the active notification
    pub fn active_start(&mut self, data: NotificationOption) {
        if matches!(data, NotificationOption::None) {
            return;
        }

        let mut noti = Notification::new();
        let notification = noti
            .summary("Wayclick")
            .body("Autoclicking")
            .timeout(0)
            .show();
        self.active_handle = notification.ok();
        self.active_config = Some(data);
    }

    /// Stop the active notification.
    pub fn active_stop(&mut self) {
        if let Some(handle) = self.active_handle.as_mut() {
            // if we have a notification it means we have a config.
            match self.active_config.as_ref().unwrap() {
                NotificationOption::None => {}
                NotificationOption::HistoryTimeout => {
                    handle.timeout(1);
                    _ = handle.update();
                }
                NotificationOption::CloseTimeout => {
                    handle.timeout(1);
                    _ = handle.update();
                }
            }
        }

        // the take in this check also accounts for resetting the handle.
        if matches!(
            self.active_config.as_ref().unwrap(),
            NotificationOption::CloseTimeout
        ) && let Some(handle) = self.active_handle.take()
        {
            self.close_notification(handle);
        }
    }
}

impl Drop for NotifyHandler {
    fn drop(&mut self) {
        // Just some basic cleanup.
        for handle in self.handles.drain(..) {
            _ = handle.join();
        }
    }
}
