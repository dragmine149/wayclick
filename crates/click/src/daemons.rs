use crate::{
    fs::{obtain_lock, pid_file_path, read_pid, write_pid},
    notify::NotifyHandler,
};
use enigo::{Enigo, Mouse};
use nix::sys::signal::{Signal, kill};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};
use wayclick_schema::Settings;

/// Start the autoclicker.
/// TODO: Clean up this code and separate some of it out?
pub fn daemon_start(profile: Option<String>) {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    let data = Settings::load().get_profile(profile);
    let mut notification_handler = NotifyHandler::default();

    notification_handler.start(data.notification.started);

    let file = obtain_lock();
    write_pid(&file);

    // graceful shutdown on SIGTERM
    let mut enigo =
        Enigo::new(&enigo::Settings::default()).expect("Failed to set up enigo. Can't autoclick");
    println!("Daemon running… press Ctrl-C or send SIGTERM to stop.");

    let initial_timeout = Duration::from_secs(data.initial);
    thread::sleep(initial_timeout);
    notification_handler.active_start(data.notification.active);

    let delay = Duration::from_millis(data.delay);
    let mut next = Instant::now() + delay;
    let mut clicks = if data.repeat == 0 {
        None
    } else {
        Some(data.repeat)
    };

    while running.load(Ordering::SeqCst) {
        thread::sleep(next.saturating_duration_since(Instant::now()));
        if let Some(pos) = data.position {
            _ = enigo.move_mouse(pos.0 as i32, pos.1 as i32, enigo::Coordinate::Abs);
        }
        enigo
            .button(enigo::Button::Left, enigo::Direction::Click)
            .expect("Failed to click, stopping immediately");
        if let Some(click) = clicks.as_mut() {
            *click -= 1;
            if *click == 0 {
                running.store(false, Ordering::SeqCst);
            }
        }

        next += delay;
    }
    notification_handler.stop(data.notification.stopped);

    println!("Daemon stopping…");
    file.set_len(0).unwrap()
}

/// Stop the autoclicker by getting the pid and sending a term signal.
/// TODO: Just in case of emergency, add a way to send a kill signal (SIGKILL)
pub fn daemon_stop() {
    match read_pid() {
        Some(pid) => {
            kill(pid, Signal::SIGTERM).unwrap_or_else(|_| {
                println!("Failed to send SIGTERM ({}) (app already closed?)", pid)
            });
            std::fs::remove_file(pid_file_path()).ok();
            println!("Sent SIGTERM to {pid}");
        }
        None => {
            eprintln!("Autoclicker isn't running.");
        }
    }
}

/// Toggle the state of the autoclicker.
///
/// State is gotten from if we have a pid file.
pub fn toggle_daemon(profile: Option<String>) {
    match read_pid() {
        Some(_) => daemon_stop(),
        None => daemon_start(profile),
    }
}

pub fn is_clicking() -> bool {
    read_pid().is_some()
}
