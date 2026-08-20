use enigo::Mouse;
use std::{
    env::current_exe,
    process::{Command, Stdio},
};

pub mod daemons;
pub mod fs;
pub mod notify;

pub use daemons::daemon_start;
pub use daemons::daemon_stop;
pub use daemons::is_clicking;
pub use daemons::toggle_daemon;

/// Start a new process in the background.
///
/// We dont need to worry about zombie processes as we clean it up by stopping it. This autoclicker is designed to run in the background with no UI interaction.
/// Waiting for this to finish would also break stuff that isn't on a separate thread.
#[allow(clippy::zombie_processes)]
pub fn start_subprocess() {
    let myself = current_exe().unwrap();
    Command::new(myself)
        .arg("start")
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap();
}

pub fn get_pos() -> (i32, i32) {
    enigo::Enigo::new(&enigo::Settings::default())
        .unwrap()
        .location()
        .unwrap()
}

pub fn move_mouse(position: (u16, u16)) {
    enigo::Enigo::new(&enigo::Settings::default())
        .unwrap()
        .move_mouse(position.0 as i32, position.1 as i32, enigo::Coordinate::Abs)
        .unwrap()
}
