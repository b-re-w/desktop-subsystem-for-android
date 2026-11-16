//! How an app window host outlives its window manager.
//!
//! The window manager can crash or be restarted while apps are open. The app window hosts keep
//! their windows (each window's DirectComposition visuals still hold the last frame, and the
//! taskbar keeps its entry), dial the new window manager again, and say which app they show
//! once more. The new window manager learns the tasks from Android, finds the window host by
//! its app and shows it the windows of the app's tasks; a window host has such a window open
//! already and keeps it.
//!
//! This module holds the decisions, which name no OS API: how long a window host waits for a
//! window manager ([`Outage`]), how it dials again ([`reconnect_window_host`]), and which of
//! its windows are still wanted once the new window manager has spoken ([`KeptWindows`]).

use std::collections::BTreeSet;
use std::time::{Duration, Instant};

use dsa_protocol::window_host::types::Attach;
use dsa_transport::{Connection, Connector, FrameSender, TransportError};

use super::window_hosts::connect_window_host;

/// How long a window host waits for a new window manager before it gives up and closes its
/// windows. The service starts a window manager that is gone within 10 seconds, and waits
/// longer between starts of one that keeps exiting, up to a minute.
pub const RECONNECT_WITHIN: Duration = Duration::from_secs(90);

/// How long a window host keeps trying when the window manager answers but turns it away
/// (it does not know the app's tasks yet, or Android has none left for it). A window manager
/// that has just started learns the tasks from Android within seconds.
pub const REFUSED_WITHIN: Duration = Duration::from_secs(20);

/// How often a window host dials while no window manager answers.
pub const RECONNECT_POLL: Duration = Duration::from_millis(250);

/// How long one dial may take.
pub const RECONNECT_DIAL: Duration = Duration::from_secs(1);

/// How long the handshake with a new window manager may take.
pub const RECONNECT_HANDSHAKE: Duration = Duration::from_secs(5);

/// How long after it attached again the window manager is given to name the windows of the
/// app's tasks. It sends them all at once, so a window it has not named by then is of a task
/// that Android no longer has.
pub const REATTACH_SETTLE: Duration = Duration::from_secs(3);

/// A window host's time without a window manager.
#[derive(Debug, Clone)]
pub struct Outage {
    lost_at: Instant,
    first_refused: Option<Instant>,
    within: Duration,
    refused_within: Duration,
    poll: Duration,
}

impl Outage {
    /// The window manager was lost at `now`.
    pub fn new(now: Instant) -> Outage {
        Outage::with_limits(now, RECONNECT_WITHIN, REFUSED_WITHIN, RECONNECT_POLL)
    }

    /// An outage with other limits, for tests.
    pub fn with_limits(
        now: Instant,
        within: Duration,
        refused_within: Duration,
        poll: Duration,
    ) -> Outage {
        Outage {
            lost_at: now,
            first_refused: None,
            within,
            refused_within,
            poll,
        }
    }

    /// A window manager took the window host on and then ended the connection without naming
    /// any of its windows.
    pub fn refused(&mut self, now: Instant) {
        self.first_refused.get_or_insert(now);
    }

    /// Whether the window host may still try at `now`.
    pub fn may_try(&self, now: Instant) -> bool {
        if now.duration_since(self.lost_at) >= self.within {
            return false;
        }
        match self.first_refused {
            Some(first) => now.duration_since(first) < self.refused_within,
            None => true,
        }
    }

    /// How long to wait between two dials.
    pub fn poll(&self) -> Duration {
        self.poll
    }

    /// Why the window host gives up at `now`, in words for the log.
    pub fn why_given_up(&self, now: Instant) -> String {
        match self.first_refused {
            Some(first) if now.duration_since(first) >= self.refused_within => format!(
                "the window manager came back but does not show the app's windows (Android has \
                 no task for them), after {} s",
                self.refused_within.as_secs()
            ),
            _ => format!(
                "no window manager came back within {} s",
                self.within.as_secs()
            ),
        }
    }
}

/// Why a window host stops dialling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GaveUp {
    /// The outage's limits passed; the text says which.
    TimedOut(String),
    /// The caller asked to stop (the windows' thread ended, say).
    Stopped,
}

/// What a window host that dialled again is given: the connection, and the GPU the window
/// manager's frames are on.
pub type Redialled = (Box<dyn Connection>, Option<[u8; 8]>);

/// Dials the window manager through `connector` until one answers and takes `attach`, the
/// `outage` ends, or `stop` says so. `sleep` waits between dials.
pub fn reconnect_window_host(
    connector: &mut dyn Connector,
    build: &str,
    attach: &Attach,
    outage: &Outage,
    stop: &dyn Fn() -> bool,
    sleep: &dyn Fn(Duration),
) -> Result<Redialled, GaveUp> {
    loop {
        if stop() {
            return Err(GaveUp::Stopped);
        }
        let now = Instant::now();
        if !outage.may_try(now) {
            return Err(GaveUp::TimedOut(outage.why_given_up(now)));
        }
        if let Ok(mut connection) = connector.connect(RECONNECT_DIAL) {
            match connect_window_host(&mut *connection, build, attach.clone(), RECONNECT_HANDSHAKE)
            {
                Ok(luid) => return Ok((connection, luid)),
                // A window manager that is still starting may drop the connection; the next
                // dial finds it ready.
                Err(_) => connection.close(),
            }
        }
        sleep(outage.poll());
    }
}

/// The sender a window host keeps while it has no window manager: every send fails at once,
/// so nothing waits on a connection that is gone.
pub struct ClosedSender;

impl FrameSender for ClosedSender {
    fn send(&mut self, _: &[u8]) -> Result<(), TransportError> {
        Err(TransportError::Closed)
    }
}

/// The task windows a window host has open, and which of them a new window manager still
/// wants.
#[derive(Debug, Default)]
pub struct KeptWindows {
    open: BTreeSet<i32>,
    /// The windows the window manager on this connection named since it attached.
    named: BTreeSet<i32>,
    /// Since when the window manager is given to name the windows; `None` once it has had its
    /// time, and on the first connection, which names every window itself.
    waiting_since: Option<Instant>,
}

impl KeptWindows {
    pub fn new() -> KeptWindows {
        KeptWindows::default()
    }

    /// The window manager asked for task `task_id`'s window.
    pub fn shown(&mut self, task_id: i32) {
        self.open.insert(task_id);
        self.named.insert(task_id);
    }

    /// Task `task_id`'s window is closed.
    pub fn closed(&mut self, task_id: i32) {
        self.open.remove(&task_id);
        self.named.remove(&task_id);
    }

    /// Whether no window is open.
    pub fn is_empty(&self) -> bool {
        self.open.is_empty()
    }

    /// The tasks whose windows are open, in task ID order.
    pub fn tasks(&self) -> Vec<i32> {
        self.open.iter().copied().collect()
    }

    /// Whether the window manager on this connection named a window since it attached.
    pub fn any_named(&self) -> bool {
        !self.named.is_empty()
    }

    /// A new window manager attached at `now`: it is given [`REATTACH_SETTLE`] to name the
    /// windows that are open.
    pub fn reattached(&mut self, now: Instant) {
        self.named.clear();
        self.waiting_since = Some(now);
    }

    /// The windows that are open but that the new window manager did not name in `settle`
    /// from `now`: their tasks are gone. They are no longer counted open. Empty until the
    /// time has passed, and after it has been asked once.
    pub fn unwanted(&mut self, now: Instant, settle: Duration) -> Vec<i32> {
        let Some(since) = self.waiting_since else {
            return Vec::new();
        };
        if now.duration_since(since) < settle {
            return Vec::new();
        }
        self.waiting_since = None;
        let gone: Vec<i32> = self.open.difference(&self.named).copied().collect();
        for task_id in &gone {
            self.open.remove(task_id);
        }
        gone
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compositor::window_hosts::{accept_window_host, answer_attached};
    use dsa_transport::{memory, Listener};
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    const CLOCK: &str = "com.android.deskclock";

    fn attach() -> Attach {
        Attach {
            package_name: CLOCK.into(),
            process_id: 7,
            launch: false,
            shortcut_id: String::new(),
        }
    }

    /// A connector that finds no window manager for its first `refusals` dials.
    struct Later {
        refusals: u32,
        dials: Arc<AtomicU32>,
        inner: memory::MemoryConnector,
    }

    impl Connector for Later {
        fn connect(&mut self, timeout: Duration) -> Result<Box<dyn Connection>, TransportError> {
            let dials = self.dials.fetch_add(1, Ordering::SeqCst);
            if dials < self.refusals {
                return Err(TransportError::Closed);
            }
            self.inner.connect(timeout)
        }
    }

    fn quick(within_ms: u64) -> Outage {
        Outage::with_limits(
            Instant::now(),
            Duration::from_millis(within_ms),
            Duration::from_millis(within_ms),
            Duration::from_millis(1),
        )
    }

    #[test]
    fn nfr5_a_window_host_dials_again_until_the_window_manager_is_back() {
        let (mut listener, connector) = memory::pair();
        let dials = Arc::new(AtomicU32::new(0));
        let mut later = Later {
            refusals: 3,
            dials: Arc::clone(&dials),
            inner: connector,
        };
        let manager = std::thread::spawn(move || {
            let mut end = listener.accept(Duration::from_secs(5)).unwrap();
            let attach = accept_window_host(&mut *end, "wm", Duration::from_secs(5)).unwrap();
            answer_attached(&mut *end, Some([9; 8])).unwrap();
            attach
        });
        let (_connection, luid) = reconnect_window_host(
            &mut later,
            "host",
            &attach(),
            &quick(5_000),
            &|| false,
            &std::thread::sleep,
        )
        .unwrap_or_else(|why| panic!("{why:?}"));
        assert_eq!(luid, Some([9; 8]));
        assert_eq!(dials.load(Ordering::SeqCst), 4);
        // It says which app it shows, and that the user did not start it.
        let said = manager.join().unwrap();
        assert_eq!(said.package_name, CLOCK);
        assert!(!said.launch);
    }

    #[test]
    fn nfr5_a_window_host_gives_up_when_no_window_manager_comes_back() {
        let (_listener, connector) = memory::pair();
        let mut never = Later {
            refusals: u32::MAX,
            dials: Arc::default(),
            inner: connector,
        };
        let outage = quick(30);
        let gave_up = reconnect_window_host(
            &mut never,
            "host",
            &attach(),
            &outage,
            &|| false,
            &std::thread::sleep,
        )
        .err();
        assert!(matches!(gave_up, Some(GaveUp::TimedOut(_))), "{gave_up:?}");
    }

    #[test]
    fn nfr5_a_window_host_stops_dialling_when_asked() {
        let (_listener, connector) = memory::pair();
        let mut never = Later {
            refusals: u32::MAX,
            dials: Arc::default(),
            inner: connector,
        };
        let gave_up = reconnect_window_host(
            &mut never,
            "host",
            &attach(),
            &quick(60_000),
            &|| true,
            &std::thread::sleep,
        )
        .err();
        assert_eq!(gave_up, Some(GaveUp::Stopped));
    }

    #[test]
    fn nfr5_a_window_manager_that_turns_the_window_host_away_ends_the_wait_sooner() {
        let start = Instant::now();
        let mut outage = Outage::with_limits(
            start,
            Duration::from_secs(90),
            Duration::from_secs(20),
            RECONNECT_POLL,
        );
        assert!(outage.may_try(start + Duration::from_secs(60)));
        outage.refused(start + Duration::from_secs(5));
        assert!(outage.may_try(start + Duration::from_secs(24)));
        assert!(!outage.may_try(start + Duration::from_secs(26)));
        assert!(outage
            .why_given_up(start + Duration::from_secs(26))
            .contains("no task"));
        // Without a refusal, the whole time is its own.
        let fresh = Outage::new(start);
        assert!(fresh.may_try(start + Duration::from_secs(89)));
        assert!(!fresh.may_try(start + Duration::from_secs(90)));
    }

    #[test]
    fn nfr5_a_closed_sender_fails_at_once() {
        assert_eq!(ClosedSender.send(b"x"), Err(TransportError::Closed));
    }

    #[test]
    fn nfr5_windows_the_new_window_manager_names_are_kept() {
        let now = Instant::now();
        let mut windows = KeptWindows::new();
        windows.shown(1);
        windows.shown(2);
        windows.reattached(now);
        windows.shown(1);
        windows.shown(2);
        assert!(windows.any_named());
        assert!(windows
            .unwanted(now + REATTACH_SETTLE, REATTACH_SETTLE)
            .is_empty());
        assert_eq!(windows.tasks(), vec![1, 2]);
    }

    #[test]
    fn nfr5_a_window_of_a_task_android_no_longer_has_closes() {
        let now = Instant::now();
        let mut windows = KeptWindows::new();
        windows.shown(1);
        windows.shown(2);
        windows.reattached(now);
        windows.shown(2);
        // Not before the window manager has had its time.
        assert!(windows.unwanted(now, REATTACH_SETTLE).is_empty());
        assert_eq!(
            windows.unwanted(now + REATTACH_SETTLE, REATTACH_SETTLE),
            vec![1]
        );
        assert_eq!(windows.tasks(), vec![2]);
        // Asked once.
        assert!(windows
            .unwanted(now + REATTACH_SETTLE * 2, REATTACH_SETTLE)
            .is_empty());
    }

    #[test]
    fn nfr5_a_window_manager_that_names_nothing_leaves_no_window_wanted() {
        let now = Instant::now();
        let mut windows = KeptWindows::new();
        windows.shown(4);
        windows.reattached(now);
        assert!(!windows.any_named());
        assert_eq!(
            windows.unwanted(now + REATTACH_SETTLE, REATTACH_SETTLE),
            vec![4]
        );
        assert!(windows.is_empty());
    }

    #[test]
    fn nfr5_the_first_connection_never_closes_a_window_for_being_unnamed() {
        let mut windows = KeptWindows::new();
        windows.shown(1);
        let later = Instant::now() + Duration::from_secs(3600);
        assert!(windows.unwanted(later, REATTACH_SETTLE).is_empty());
        windows.closed(1);
        assert!(windows.is_empty());
    }
}
