//! The host's own log files (the service's, the window manager's, the app windows'), bounded
//! the way the guest's console log is ([`crate::console_log`]): a file over
//! [`ROTATE_AT_BYTES`] moves to `<name>.1`, replacing the copy there, and a new file starts.
//!
//! The console log moves aside when a boot starts, but these processes run for days and the
//! window manager writes the input trace when it is on, so a log grew without end: the dev.40
//! window manager's log was larger than the 64 MiB the diagnostics keep of it. A
//! [`RotatingLog`] moves its file aside when it opens and, while it writes, each time its own
//! writes add up to the check interval.
//!
//! Several processes may write one file (every app window is a process of its own). A writer
//! whose file another process moved aside notices at its next check (the file at the path is
//! no longer the one it writes) and opens the new one.

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::console_log::rotate_if_large;

/// The size over which a host log moves aside. With the older copy, a log keeps up to twice
/// this, which is what the diagnostics take of each file (64 MiB).
pub const ROTATE_AT_BYTES: u64 = 32 * 1024 * 1024;

/// How much a writer writes between two looks at the file's size.
pub const CHECK_EVERY_BYTES: u64 = 256 * 1024;

/// How often a log that is not written through a [`RotatingLog`] (the service's standard
/// error, which the system writes through a handle) is looked at, and moved aside with
/// [`rotate_if_large`] and [`ROTATE_AT_BYTES`] when it is over.
pub const CHECK_INTERVAL: std::time::Duration = std::time::Duration::from_secs(60);

/// The diagnostics' name for the older copy of a host log named `name` there, such as
/// `host/dsa-wm.1.log` for `host/dsa-wm.log`.
pub fn old_copy_name(name: &str) -> String {
    match name.strip_suffix(".log") {
        Some(stem) => format!("{stem}.1.log"),
        None => format!("{name}.1"),
    }
}

/// A log file that one or more processes append lines to, kept under its limit.
pub struct RotatingLog {
    path: PathBuf,
    limit: u64,
    check_every: u64,
    file: Option<File>,
    /// What this writer wrote since it last looked at the size.
    unchecked: u64,
}

impl RotatingLog {
    /// Opens the log at `path` with the host logs' limit ([`ROTATE_AT_BYTES`]), making its
    /// folder, and moves it aside first when it is over the limit.
    pub fn open(path: &Path) -> io::Result<RotatingLog> {
        Self::open_with_limits(path, ROTATE_AT_BYTES, CHECK_EVERY_BYTES)
    }

    /// [`open`](Self::open) with the limit and the check interval given.
    pub fn open_with_limits(path: &Path, limit: u64, check_every: u64) -> io::Result<RotatingLog> {
        if let Some(folder) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(folder)?;
        }
        // A file another program holds open without letting it move stays where it is; the
        // next check tries again.
        let _ = rotate_if_large(path, limit);
        let file = append(path)?;
        Ok(RotatingLog {
            path: path.to_path_buf(),
            limit,
            check_every: check_every.max(1),
            file: Some(file),
            unchecked: 0,
        })
    }

    /// The log's path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Appends `line` and a line end, then moves the file aside when this writer's writes
    /// since the last look add up to the check interval and the file is over the limit.
    pub fn write_line(&mut self, line: &str) -> io::Result<()> {
        let mut bytes = Vec::with_capacity(line.len() + 1);
        bytes.extend_from_slice(line.as_bytes());
        bytes.push(b'\n');
        self.write(&bytes)
    }

    /// Appends `bytes` as they are, as [`write_line`](Self::write_line) does.
    pub fn write(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.file.is_none() {
            self.file = Some(append(&self.path)?);
        }
        let file = self.file.as_mut().expect("opened above");
        file.write_all(bytes)?;
        file.flush()?;
        self.unchecked += bytes.len() as u64;
        if self.unchecked >= self.check_every {
            self.unchecked = 0;
            self.keep_bounded();
        }
        Ok(())
    }

    /// Moves the file aside when it is over the limit, and follows to the file at the path
    /// when another process moved the one this writer has.
    fn keep_bounded(&mut self) {
        let on_disk = match std::fs::metadata(&self.path) {
            Ok(metadata) => Some(metadata.len()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => None,
            // Unreadable for a moment: the next check looks again.
            Err(_) => return,
        };
        let ours = self
            .file
            .as_ref()
            .and_then(|file| file.metadata().ok())
            .map(|metadata| metadata.len());
        match on_disk {
            Some(size) if size > self.limit => {
                // Closed first: a file this process holds open may not move on every system.
                self.file = None;
                let _ = rotate_if_large(&self.path, self.limit);
                self.file = append(&self.path).ok();
            }
            Some(size) if Some(size) == ours => {}
            // Another writer moved it aside (and may not have written the new one yet).
            _ => self.file = append(&self.path).ok(),
        }
    }
}

fn append(path: &Path) -> io::Result<File> {
    OpenOptions::new().create(true).append(true).open(path)
}
