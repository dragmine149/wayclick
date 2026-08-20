use nix::{
    fcntl::{Flock, FlockArg},
    unistd::Pid,
};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    os::unix::fs::OpenOptionsExt,
    path::PathBuf,
    process,
};

/// Get the file path where we store the pid.
///
/// Yes this supports multiple systems, do i care that much? nah.
pub(crate) fn pid_file_path() -> PathBuf {
    dirs::runtime_dir()
        .or_else(dirs::cache_dir)
        .expect("Can't find a place to write lock file")
        .join("wayclick.pid")
}

/// Return a copy of a file where we've obtained the lock.
/// NOTE: The lock might not be fully required and seems to be unix only. As much as this doesn't matter, this could be a point of change.
pub(crate) fn obtain_lock() -> Flock<File> {
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
pub(crate) fn write_pid(mut file: &File) {
    let pid = process::id().to_string();
    file.set_len(0).unwrap();
    file.write_all(pid.as_bytes()).unwrap();
    file.sync_all().unwrap();
}

/// Attempts to read the pid from the file and returns a dedicated pid object.
pub fn read_pid() -> Option<Pid> {
    let mut buf = String::new();
    File::open(pid_file_path())
        .ok()
        .and_then(|mut f| f.read_to_string(&mut buf).ok())
        .and_then(|_| buf.trim().parse::<i32>().ok())
        .map(Pid::from_raw)
}
