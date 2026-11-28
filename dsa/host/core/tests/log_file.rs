//! The host's own log files (the service's, the window manager's, the app windows'), bounded
//! as the guest's console log is: over the limit, a file moves to `<name>.1` and a new one
//! starts.

use std::path::PathBuf;

use dsa_host_core::console_log::old_copy_path;
use dsa_host_core::log_file::{RotatingLog, ROTATE_AT_BYTES};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "dsa-log-file-{name}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn read(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

#[test]
fn fr21_the_host_logs_are_bounded_like_the_console_log() {
    // Enough for the window manager's input trace, and the diagnostics' 64 MiB a file with
    // the older copy.
    assert_eq!(ROTATE_AT_BYTES, 32 * 1024 * 1024);
}

#[test]
fn fr21_a_log_over_the_limit_when_it_opens_moves_aside_first() {
    let dir = scratch("open");
    let log = dir.join("dsa-wm.log");
    std::fs::write(&log, "an old session, too long\n").unwrap();
    let mut file = RotatingLog::open_with_limits(&log, 10, 1).unwrap();
    file.write_line("new").unwrap();
    assert_eq!(read(&old_copy_path(&log)), "an old session, too long\n");
    assert_eq!(read(&log), "new\n");
    // Under the limit, it is appended to.
    drop(file);
    let mut file = RotatingLog::open_with_limits(&log, 10, 1).unwrap();
    file.write_line("more").unwrap();
    assert_eq!(read(&log), "new\nmore\n");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fr21_a_log_that_grows_past_the_limit_while_written_moves_aside_and_starts_again() {
    let dir = scratch("grow");
    let log = dir.join("dsa-hostd.log");
    let mut file = RotatingLog::open_with_limits(&log, 20, 1).unwrap();
    for line in ["first line", "second line", "third line"] {
        file.write_line(line).unwrap();
    }
    // The second line took the file past 20 bytes, so it moved before the third.
    assert_eq!(read(&old_copy_path(&log)), "first line\nsecond line\n");
    assert_eq!(read(&log), "third line\n");
    for line in ["fourth line", "fifth line"] {
        file.write_line(line).unwrap();
    }
    // One older copy only: it replaces the copy before.
    assert_eq!(read(&old_copy_path(&log)), "third line\nfourth line\n");
    assert_eq!(read(&log), "fifth line\n");
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fr21_a_writer_whose_file_another_process_moved_follows_to_the_new_file() {
    // Every app window is a process of its own, and they share one log.
    let dir = scratch("shared");
    let log = dir.join("dsa-app-window.log");
    let mut one = RotatingLog::open_with_limits(&log, 20, 1).unwrap();
    let mut two = RotatingLog::open_with_limits(&log, 1000, 1).unwrap();
    one.write_line("one: a long line").unwrap();
    one.write_line("one: moves it").unwrap();
    // `one` moved the file aside; `two` notices at its next write and goes on in the new file.
    two.write_line("two: written").unwrap();
    two.write_line("two: again").unwrap();
    assert!(read(&log).ends_with("two: again\n"), "{}", read(&log));
    assert!(!read(&old_copy_path(&log)).contains("two: again"));
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fr21_a_log_in_a_folder_that_does_not_exist_yet_is_made() {
    let dir = scratch("folder");
    let log = dir.join("logs").join("dsa-wm.log");
    let mut file = RotatingLog::open(&log).unwrap();
    file.write_line("started").unwrap();
    assert_eq!(read(&log), "started\n");
    assert_eq!(file.path(), log.as_path());
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fr21_the_diagnostics_carry_each_host_logs_older_copy_under_its_own_name() {
    use dsa_host_core::log_file::old_copy_name;
    assert_eq!(old_copy_name("host/dsa-wm.log"), "host/dsa-wm.1.log");
    assert_eq!(old_copy_name("host/dsa-hostd.log"), "host/dsa-hostd.1.log");
    assert_eq!(old_copy_name("host/notes"), "host/notes.1");
}
