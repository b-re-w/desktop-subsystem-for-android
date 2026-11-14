//! Whether any task window is shown, which is what lets the guest sleep.
//!
//! A task window counts as shown while it is open and not minimized; a window covered by
//! other windows still counts, since the user can see it again at any moment without
//! telling the window manager. [`ShownWindows`] follows the windows the window manager has
//! and the windows their hosts report minimized, and says when the answer to "is any window
//! shown" changes, as the [`TaskRequest::WindowsShown`] that carries it to the guest.
//!
//! Which tasks the user can see is a second answer, [`TaskRequest::TasksShown`]: there a
//! window wholly covered by other windows, or on another virtual desktop, counts as hidden as
//! a minimized one does, where the window manager can tell ([`ShownWindows::covered`]). The
//! guest keeps less memory for the app of a task the user cannot see and trims it, which costs
//! nothing when the window shows again; it never sleeps for a covered window.

use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

use crate::tasks::TaskRequest;

/// A rectangle on the screen: left, top, right, bottom, in pixels.
pub type ScreenRect = (i32, i32, i32, i32);

/// Whether the windows `above` (each rectangle one window above it) together leave nothing of
/// `target` to see. An empty target is wholly covered.
pub fn wholly_covered(target: ScreenRect, above: &[ScreenRect]) -> bool {
    // What is still visible of the target, as rectangles that do not overlap; each window
    // above cuts each of them into at most four.
    let mut visible = vec![target];
    for &(left, top, right, bottom) in above {
        if left >= right || top >= bottom {
            continue;
        }
        let mut rest = Vec::with_capacity(visible.len());
        for (l, t, r, b) in visible {
            if l >= r || t >= b {
                continue;
            }
            if right <= l || r <= left || bottom <= t || b <= top {
                rest.push((l, t, r, b));
                continue;
            }
            if t < top {
                rest.push((l, t, r, top));
            }
            if bottom < b {
                rest.push((l, bottom, r, b));
            }
            let (middle_top, middle_bottom) = (t.max(top), b.min(bottom));
            if l < left {
                rest.push((l, middle_top, left, middle_bottom));
            }
            if right < r {
                rest.push((right, middle_top, r, middle_bottom));
            }
        }
        visible = rest;
        if visible.is_empty() {
            return true;
        }
    }
    visible.iter().all(|&(l, t, r, b)| l >= r || t >= b)
}

/// How long a window must stay covered before it counts as hidden: past switching between
/// windows with Alt+Tab or a click, so a window covered for a moment is not trimmed.
pub const COVERED_AFTER: Duration = Duration::from_secs(2);

/// Follows which task windows are covered, and says which have been covered for
/// [`COVERED_AFTER`] at least. A window shown again counts as uncovered at once.
#[derive(Debug, Default)]
pub struct CoverWatch {
    since: BTreeMap<i32, Instant>,
}

impl CoverWatch {
    /// Takes the windows covered `now`, and gives those covered long enough to count.
    pub fn seen(&mut self, covered: &BTreeSet<i32>, now: Instant) -> BTreeSet<i32> {
        self.since.retain(|task_id, _| covered.contains(task_id));
        for task_id in covered {
            self.since.entry(*task_id).or_insert(now);
        }
        self.since
            .iter()
            .filter(|(_, since)| now.duration_since(**since) >= COVERED_AFTER)
            .map(|(task_id, _)| *task_id)
            .collect()
    }
}

/// The minimized and the covered task windows, and the last words the guest was given.
#[derive(Debug, Default)]
pub struct ShownWindows {
    minimized: BTreeSet<i32>,
    covered: BTreeSet<i32>,
    told: Option<bool>,
    told_tasks: Option<Vec<i32>>,
}

impl ShownWindows {
    pub fn new() -> ShownWindows {
        ShownWindows::default()
    }

    /// The window host reports a task's window minimized (`shown` false) or shown again.
    pub fn reported(&mut self, task_id: i32, shown: bool) {
        if shown {
            self.minimized.remove(&task_id);
        } else {
            self.minimized.insert(task_id);
        }
    }

    /// The task windows the user cannot see although they are open and not minimized: wholly
    /// covered by other windows, or on another virtual desktop. Each call gives the whole set.
    pub fn covered(&mut self, task_ids: BTreeSet<i32>) {
        self.covered = task_ids;
    }

    /// Of the tasks that have a window now, those whose window is shown (not minimized).
    pub fn shown_among(&self, windows: &[i32]) -> Vec<i32> {
        windows
            .iter()
            .copied()
            .filter(|task_id| !self.minimized.contains(task_id))
            .collect()
    }

    /// Forgets what the guest was told, so the next [`update`](Self::update) says it again,
    /// as when the service may have restarted and lost it.
    pub fn retell(&mut self) {
        self.told = None;
        self.told_tasks = None;
    }

    /// Given the tasks that have a window now, the request to send when which of them the
    /// user can see changed since the guest was last told, or it was never told: the guest
    /// keeps less memory for the app of a task that is minimized or covered, and trims it. Sent
    /// before [`update`](Self::update)'s, so a guest that wakes for a shown window already
    /// knows which one it is.
    pub fn update_tasks(&mut self, windows: &[i32]) -> Option<TaskRequest> {
        self.minimized.retain(|task_id| windows.contains(task_id));
        self.covered.retain(|task_id| windows.contains(task_id));
        let mut task_ids: Vec<i32> = self
            .shown_among(windows)
            .into_iter()
            .filter(|task_id| !self.covered.contains(task_id))
            .collect();
        task_ids.sort_unstable();
        task_ids.dedup();
        if self.told_tasks.as_ref() == Some(&task_ids) {
            return None;
        }
        self.told_tasks = Some(task_ids.clone());
        Some(TaskRequest::TasksShown { task_ids })
    }

    /// Given the tasks that have a window now, the request to send when whether any window
    /// is shown changed since the guest was last told, or it was never told.
    pub fn update(&mut self, windows: &[i32]) -> Option<TaskRequest> {
        // A window that closed is no longer minimized; one that opens again opens shown.
        self.minimized.retain(|task_id| windows.contains(task_id));
        let shown = windows
            .iter()
            .any(|task_id| !self.minimized.contains(task_id));
        if self.told == Some(shown) {
            return None;
        }
        self.told = Some(shown);
        Some(TaskRequest::WindowsShown { shown })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shown(shown: bool) -> Option<TaskRequest> {
        Some(TaskRequest::WindowsShown { shown })
    }

    #[test]
    fn fr10_the_guest_is_told_at_once_whether_any_window_is_shown() {
        let mut windows = ShownWindows::new();
        assert_eq!(windows.update(&[]), shown(false));
        assert_eq!(windows.update(&[]), None);
        assert_eq!(windows.update(&[3]), shown(true));
        assert_eq!(windows.update(&[3]), None);
    }

    #[test]
    fn fr10_closing_or_minimizing_every_window_says_none_is_shown() {
        let mut windows = ShownWindows::new();
        assert_eq!(windows.update(&[1, 2]), shown(true));
        windows.reported(1, false);
        assert_eq!(windows.update(&[1, 2]), None, "window 2 is still shown");
        // Window 2 closes: only a minimized window is left.
        assert_eq!(windows.update(&[1]), shown(false));
        // Restoring it shows it again.
        windows.reported(1, true);
        assert_eq!(windows.update(&[1]), shown(true));
    }

    #[test]
    fn nfr1_a_covered_window_counts_as_hidden_but_does_not_let_the_guest_sleep() {
        let mut windows = ShownWindows::new();
        assert_eq!(windows.update_tasks(&[1, 2]), tasks(&[1, 2]));
        assert_eq!(windows.update(&[1, 2]), shown(true));
        // Window 2 covers window 1 wholly: the guest hears task 1 is hidden, and stays awake.
        windows.covered(BTreeSet::from([1]));
        assert_eq!(windows.update_tasks(&[1, 2]), tasks(&[2]));
        assert_eq!(windows.update(&[1, 2]), None);
        // Window 2 is minimized too: nothing is seen, yet window 1 may show at any moment.
        windows.reported(2, false);
        assert_eq!(windows.update_tasks(&[1, 2]), tasks(&[]));
        assert_eq!(windows.update(&[1, 2]), None);
        // Uncovered, window 1 is seen again.
        windows.covered(BTreeSet::new());
        assert_eq!(windows.update_tasks(&[1, 2]), tasks(&[1]));
        // A covered window that closes and opens again opens seen.
        windows.covered(BTreeSet::from([1]));
        assert_eq!(windows.update_tasks(&[2]), tasks(&[]));
        assert_eq!(windows.update_tasks(&[1, 2]), tasks(&[1]));
    }

    #[test]
    fn nfr1_a_window_is_hidden_only_when_the_windows_above_leave_nothing_of_it() {
        let window = (100, 100, 500, 400);
        assert!(!wholly_covered(window, &[]));
        // One window over all of it, or larger.
        assert!(wholly_covered(window, &[(0, 0, 800, 600)]));
        // Two windows side by side, each over half.
        assert!(wholly_covered(
            window,
            &[(0, 0, 300, 600), (300, 0, 800, 600)]
        ));
        // A strip of one pixel left between them.
        assert!(!wholly_covered(
            window,
            &[(0, 0, 300, 600), (301, 0, 800, 600)]
        ));
        // A window over the middle leaves the edges.
        assert!(!wholly_covered(window, &[(150, 150, 450, 350)]));
        // Windows elsewhere, and empty ones, cover nothing.
        assert!(!wholly_covered(
            window,
            &[(600, 0, 900, 600), (100, 100, 100, 400)]
        ));
        // Four windows around a hole, then one over the hole.
        let frame = [
            (0, 0, 800, 200),
            (0, 300, 800, 600),
            (0, 0, 200, 600),
            (400, 0, 800, 600),
        ];
        assert!(!wholly_covered(window, &frame));
        let mut closed = frame.to_vec();
        closed.push((200, 200, 400, 300));
        assert!(wholly_covered(window, &closed));
    }

    #[test]
    fn nfr1_a_window_covered_for_a_moment_does_not_count_as_hidden() {
        let mut watch = CoverWatch::default();
        let start = Instant::now();
        let one = BTreeSet::from([1]);
        assert!(watch.seen(&one, start).is_empty());
        assert!(watch.seen(&one, start + Duration::from_secs(1)).is_empty());
        assert_eq!(watch.seen(&one, start + COVERED_AFTER), one);
        // Uncovered, it counts as seen at once, and covered again it waits again.
        assert!(watch
            .seen(&BTreeSet::new(), start + Duration::from_secs(3))
            .is_empty());
        assert!(watch.seen(&one, start + Duration::from_secs(4)).is_empty());
    }

    #[test]
    fn fr10_a_window_that_closes_while_minimized_opens_again_shown() {
        let mut windows = ShownWindows::new();
        assert_eq!(windows.update(&[1]), shown(true));
        windows.reported(1, false);
        assert_eq!(windows.update(&[1]), shown(false));
        assert_eq!(windows.update(&[]), None);
        assert_eq!(windows.update(&[1]), shown(true));
    }

    #[test]
    fn fr10_the_word_is_said_again_after_the_service_comes_back() {
        let mut windows = ShownWindows::new();
        assert_eq!(windows.update(&[1]), shown(true));
        windows.retell();
        assert_eq!(windows.update(&[1]), shown(true));
    }

    fn tasks(task_ids: &[i32]) -> Option<TaskRequest> {
        Some(TaskRequest::TasksShown {
            task_ids: task_ids.to_vec(),
        })
    }

    #[test]
    fn nfr2_the_guest_is_told_which_task_windows_are_shown() {
        let mut windows = ShownWindows::new();
        assert_eq!(windows.update_tasks(&[2, 1]), tasks(&[1, 2]));
        assert_eq!(windows.update_tasks(&[1, 2]), None);
        // Minimizing one of two windows changes no word on whether any is shown, but it does
        // change which are.
        windows.reported(1, false);
        assert_eq!(windows.update_tasks(&[1, 2]), tasks(&[2]));
        assert_eq!(windows.update(&[1, 2]), shown(true));
        windows.reported(1, true);
        assert_eq!(windows.update_tasks(&[1, 2]), tasks(&[1, 2]));
        // A closed window is no longer shown.
        assert_eq!(windows.update_tasks(&[2]), tasks(&[2]));
        // And the word is said again after the service comes back.
        windows.retell();
        assert_eq!(windows.update_tasks(&[2]), tasks(&[2]));
    }
}
