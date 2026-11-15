//! Whether a screen reader runs in a user's desktop session, which gives the administrator
//! prompt for an Android app's root request longer: hearing its words and buttons read out
//! takes longer than reading them.
//!
//! The desktop says it on the session bus: `org.a11y.Bus` at `/org/a11y/bus`, whose
//! `org.a11y.Status` property `ScreenReaderEnabled` is true while Orca (or another screen
//! reader) runs. GNOME keeps it in step with its `screen-reader-enabled` setting, and Orca sets
//! it when it starts. The service runs as root and the bus accepts only its own user, so the
//! question runs in a short helper started as the user (`dsa-linux-hostd
//! screen-reader-helper`, with the user's IDs and their bus's address), which answers with its
//! exit code. It is asked as each prompt is about to open, since a screen reader can be started
//! or stopped at any time.

use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};
use std::time::{Duration, Instant};

use zbus::blocking::Connection;
use zbus::zvariant::OwnedValue;

use crate::secret_service::user_bus;
use crate::users::Account;

/// The hidden subcommand of `dsa-linux-hostd` that runs the helper.
pub const HELPER_ARG: &str = "screen-reader-helper";

/// The helper's exit code when a screen reader runs.
pub const RUNNING: u8 = 10;

/// The helper's exit code when none runs, or the session cannot say.
pub const NOT_RUNNING: u8 = 11;

/// How long the service waits for the helper. Reading one property takes milliseconds; a
/// helper that hangs past this is ended, and the prompt opens with its default time rather
/// than not at all.
const HELPER_WAIT: Duration = Duration::from_secs(5);

/// How often the service looks whether the helper has ended.
const POLL: Duration = Duration::from_millis(20);

const A11Y_BUS: &str = "org.a11y.Bus";
const A11Y_BUS_PATH: &str = "/org/a11y/bus";
const STATUS: &str = "org.a11y.Status";
const PROPERTIES: &str = "org.freedesktop.DBus.Properties";

/// Whether the helper's exit `code` says a screen reader runs. Any other code (the helper
/// failed, was ended, or is older than the question) says no.
pub fn running_of_exit(code: Option<i32>) -> bool {
    code == Some(i32::from(RUNNING))
}

/// `dsa-linux-hostd screen-reader-helper`: reads `ScreenReaderEnabled` on the session bus
/// `DBUS_SESSION_BUS_ADDRESS` names and answers with [`RUNNING`] or [`NOT_RUNNING`].
pub fn helper_main() -> ExitCode {
    let running = Connection::session()
        .ok()
        .and_then(|session| {
            session
                .call_method(
                    Some(A11Y_BUS),
                    A11Y_BUS_PATH,
                    Some(PROPERTIES),
                    "Get",
                    &(STATUS, "ScreenReaderEnabled"),
                )
                .ok()
        })
        .and_then(|reply| reply.body().deserialize::<OwnedValue>().ok())
        .and_then(|value| bool::try_from(value).ok())
        .unwrap_or(false);
    ExitCode::from(if running { RUNNING } else { NOT_RUNNING })
}

/// Asks whether a screen reader runs in `account`'s desktop session, through the helper
/// started as them. No when they have no session bus or the helper cannot tell, which leaves
/// the prompt its default time.
pub fn running_for(account: &Account) -> bool {
    let (address, socket) = user_bus(account.uid);
    if !socket.exists() {
        return false;
    }
    let mut command = Command::new(PathBuf::from("/proc/self/exe"));
    command.arg(HELPER_ARG);
    account.run_as(&mut command, None);
    command
        .env("DBUS_SESSION_BUS_ADDRESS", address)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            eprintln!(
                "root: cannot tell whether a screen reader runs for {}, so the prompt has its \
                 usual time: {error}",
                account.name
            );
            return false;
        }
    };
    let until = Instant::now() + HELPER_WAIT;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return running_of_exit(status.code()),
            Ok(None) if Instant::now() < until => std::thread::sleep(POLL),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                eprintln!(
                    "root: the screen reader check for {} did not answer in time, so the \
                     prompt has its usual time",
                    account.name
                );
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fr18_the_helpers_exit_code_says_whether_a_screen_reader_runs() {
        assert!(running_of_exit(Some(i32::from(RUNNING))));
        assert!(!running_of_exit(Some(i32::from(NOT_RUNNING))));
        // A helper that failed, was ended by a signal, or does not know the question says no.
        assert!(!running_of_exit(Some(0)));
        assert!(!running_of_exit(Some(1)));
        assert!(!running_of_exit(Some(2)));
        assert!(!running_of_exit(None));
        assert_ne!(RUNNING, NOT_RUNNING);
    }
}
