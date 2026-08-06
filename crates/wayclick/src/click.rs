use enigo::{Enigo, Mouse};
use nix::{
    fcntl::{Flock, FlockArg},
    sys::signal::{Signal, kill},
    unistd::Pid,
};
use notify_rust::Notification;
use std::{
    env::current_exe,
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::OpenOptionsExt,
    path::PathBuf,
    process::{self, Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
use wayclick_schema::Settings;

/// Get the file path where we store the pid.
///
/// Yes this supports multiple systems, do i care that much? nah.
pub fn pid_file_path() -> PathBuf {
    dirs::runtime_dir()
        .or_else(dirs::cache_dir)
        .expect("Can't find a place to write lock file")
        .join("wayclick.pid")
}

/// Return a copy of a file where we've obtained the lock.
/// NOTE: The lock might not be fully required and seems to be unix only. As much as this doesn't matter, this could be a point of change.
fn obtain_lock() -> Flock<File> {
    let path = pid_file_path();
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .mode(0o600)
        .open(&path)
        .expect("Cannot open pid-file");
    // exclusive, non-blocking lock
    let file = Flock::lock(file, FlockArg::LockExclusiveNonblock);
    if file.is_err() {
        eprintln!("Another instance is already running.");
        process::exit(1);
    }
    file.unwrap()
}

/// Write out current pid to the file.
fn write_pid(mut file: &File) {
    let pid = process::id().to_string();
    file.set_len(0).unwrap();
    file.write_all(pid.as_bytes()).unwrap();
    file.sync_all().unwrap();
}

/// Attempts to read the pid from the file and returns a dedicated pid object.
fn read_pid() -> Option<Pid> {
    let mut buf = String::new();
    File::open(pid_file_path())
        .ok()
        .and_then(|mut f| f.read_to_string(&mut buf).ok())
        .and_then(|_| buf.trim().parse::<i32>().ok())
        .map(Pid::from_raw)
}

/// Start the autoclicker.
/// TODO: Clean up this code and separate some of it out?
pub fn daemon_start(profile: Option<String>) {
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

    let data = Settings::load().get_profile(profile);
    let initial_timeout = Duration::from_secs(data.initial);

    let mut notification = Notification::new()
        .summary("Wayclick")
        .body(&format!(
            "Starting autoclicking in {} seconds",
            data.initial
        ))
        .timeout(initial_timeout)
        .show()
        .unwrap();

    thread::sleep(initial_timeout);
    notification.body("Autoclicking...").timeout(0);
    notification.update();

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_millis(data.delay));
        enigo
            .button(enigo::Button::Left, enigo::Direction::Click)
            .unwrap();
        // enigo.button(enigo::Button::Left, enigo::Direction::)
        // enigo
        //     .key(enigo::Key::Shift, enigo::Direction::Press)
        //     .unwrap();
    }
    notification.body("Autoclicking finished").timeout(2);
    notification.update();
    println!("Daemon stopping…");
}

/// Stop the autoclicker by getting the pid and sending a term signal.
/// TODO: Just in case of emergency, add a way to send a kill signal (SIGKILL)
pub fn daemon_stop() {
    match read_pid() {
        Some(pid) => {
            kill(pid, Signal::SIGTERM)
                .unwrap_or_else(|_| panic!("Failed to send SIGTERM ({})", pid));
            std::fs::remove_file(pid_file_path()).ok();
            println!("Sent SIGTERM to {pid}");
        }
        None => {
            eprintln!("No pid-file found; nothing to stop.");
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

pub fn start_subprocess() {
    let myself = current_exe().unwrap();
    Command::new(myself)
        .arg("start")
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap();
}
