use clap::{Parser, Subcommand, builder::ArgPredicate};
use enigo::{Enigo, Mouse};
use nix::{
    fcntl::Flock,
    sys::signal::{Signal, kill},
    unistd::Pid,
};
use notify_rust::Notification;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::{fd::AsRawFd, unix::fs::OpenOptionsExt},
    path::PathBuf,
    process,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use crate::storage::{Data, Settings};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "An autoclicker designed for use with wayland systems."
)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Option<Subcommands>,
}

#[derive(Subcommand, Debug, Default)]
pub enum Subcommands {
    /// Starts the autoclicker / macro, defaults to your last saved settings.
    ///
    /// Note: Only one instance can be running at a time.
    Start,
    /// Stops the autoclicker.
    Stop,
    /// Toggles the current state of the autoclicker.
    Toggle,
    /// Opens up the UI for configuring the autoclicker.
    #[default]
    Ui,
}

fn pid_file_path() -> PathBuf {
    dirs::runtime_dir()
        .or_else(|| dirs::cache_dir())
        .expect("Can't find a place to write lock file")
        .join("wayclick.pid")
}

fn obtain_lock() -> Flock<File> {
    let path = pid_file_path();
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .mode(0o600)
        .open(&path)
        .expect("Cannot open pid-file");
    // exclusive, non-blocking lock
    let file = Flock::lock(file, nix::fcntl::FlockArg::LockExclusiveNonblock);
    if file.is_err() {
        eprintln!("Another instance is already running.");
        process::exit(1);
    }
    file.unwrap()
}

fn write_pid(mut file: &File) {
    let pid = process::id().to_string();
    file.set_len(0).unwrap();
    file.write_all(pid.as_bytes()).unwrap();
    file.sync_all().unwrap();
}

fn read_pid() -> Option<Pid> {
    let mut buf = String::new();
    File::open(pid_file_path())
        .ok()
        .and_then(|mut f| f.read_to_string(&mut buf).ok())
        .and_then(|_| buf.trim().parse::<i32>().ok())
        .map(Pid::from_raw)
}

pub fn daemon_start() {
    Notification::new()
        .summary("Wayclick")
        .body("Autoclicker is loading")
        .show()
        .unwrap();
    let file = obtain_lock();
    write_pid(&file);

    // graceful shutdown on SIGTERM
    let running = Arc::new(AtomicBool::new(true));
    let mut enigo = Enigo::new(&enigo::Settings::default()).unwrap();
    println!("Daemon running… press Ctrl-C or send SIGTERM to stop.");

    let data = Settings::load_data().merge_default();
    let time =
        data.milliseconds.unwrap() + data.seconds.unwrap() * 1000 + data.minutes.unwrap() * 60_000;
    let initial_timeout = Duration::from_secs(data.initial.unwrap() + 1);

    let mut notification = Notification::new()
        .summary("Wayclick")
        .body(&format!(
            "Starting autoclicking in {} seconds",
            data.initial.unwrap()
        ))
        .timeout(initial_timeout)
        .show()
        .unwrap();

    thread::sleep(initial_timeout);
    notification.body("Autoclicking...").timeout(0);
    notification.update();

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(time));
        enigo
            .button(enigo::Button::Left, enigo::Direction::Click)
            .unwrap();
    }
    notification.body("Autoclicking finished").timeout(2);
    notification.update();
    println!("Daemon stopping…");
}

pub fn daemon_stop() {
    match read_pid() {
        Some(pid) => {
            kill(pid, Signal::SIGTERM).expect(&format!("Failed to send SIGTERM ({})", pid));
            std::fs::remove_file(pid_file_path()).ok();
            println!("Sent SIGTERM to {pid}");
        }
        None => {
            eprintln!("No pid-file found; nothing to stop.");
        }
    }
}

pub fn toggle_daemon() {
    Notification::new()
        .summary("Wayclick")
        .body("e2")
        .show()
        .unwrap();
    match read_pid() {
        Some(_) => daemon_stop(),
        None => daemon_start(),
    }
}
