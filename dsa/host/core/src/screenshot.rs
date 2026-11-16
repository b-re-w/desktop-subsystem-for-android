//! The names a screenshot is saved under, the same on every host that names them this way.
//!
//! The file is named as the Snipping Tool names its own: "Screenshot 2026-11-08 143022.png",
//! the date and the local time to the second. A second screenshot in the same second gets
//! " (2)" before the extension, the next " (3)", and so on, so no file is ever written over.

use std::path::{Path, PathBuf};

/// A local date and time, to the second.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalTime {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
}

/// The file name of the `copy`th screenshot taken at `time` (1 for the first), starting with
/// `prefix`: "Screenshot 2026-11-08 143022.png", then "Screenshot 2026-11-08 143022 (2).png".
pub fn file_name(prefix: &str, time: &LocalTime, copy: u32) -> String {
    let stem = format!(
        "{prefix} {:04}-{:02}-{:02} {:02}{:02}{:02}",
        time.year, time.month, time.day, time.hour, time.minute, time.second
    );
    if copy <= 1 {
        format!("{stem}.png")
    } else {
        format!("{stem} ({copy}).png")
    }
}

/// The first path in `folder` named for a screenshot at `time` that `exists` says is free.
pub fn free_path(
    folder: &Path,
    prefix: &str,
    time: &LocalTime,
    exists: impl Fn(&Path) -> bool,
) -> PathBuf {
    let mut copy = 1;
    loop {
        let path = folder.join(file_name(prefix, time, copy));
        if !exists(&path) || copy >= 10_000 {
            return path;
        }
        copy += 1;
    }
}
