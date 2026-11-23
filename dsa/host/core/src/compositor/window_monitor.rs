//! Which monitor an app's window opens on, and where on it.
//!
//! The window manager and the app's window host both read the answer before the window opens:
//! the window manager to have the task draw at that monitor's density and size first, the
//! window host to put the window there. A window placed only after it opened showed the app at
//! the main monitor's density and size, then shrank on a monitor of a lower scale while the app
//! redrew. Each side asks [`opening_monitor`] with the monitors the desktop has now, so both
//! come to the same one.

/// A monitor of the desktop, as the platform describes it: an ID that names the same monitor
/// whatever order the desktop lists the monitors in, its DPI (96 is 100%), whether it is the
/// main one, and its work area (left, top, right, bottom), the part a window is placed in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesktopMonitor {
    pub id: String,
    pub dpi: u32,
    pub main: bool,
    pub work: Area,
}

/// A rectangle on the desktop: left, top, right, bottom.
pub type Area = (i32, i32, i32, i32);

/// The monitor an app's window opens on: the one its last window closed on (`last`, by ID)
/// while that one is connected, else the main monitor. `None` only when the desktop has no
/// monitor.
pub fn opening_monitor<'a>(
    last: Option<&str>,
    monitors: &'a [DesktopMonitor],
) -> Option<&'a DesktopMonitor> {
    last.filter(|id| !id.is_empty())
        .and_then(|id| monitors.iter().find(|monitor| monitor.id == id))
        .or_else(|| monitors.iter().find(|monitor| monitor.main))
        .or_else(|| monitors.first())
}

/// Where a window goes on the monitor whose work area is `to`, when the desktop put it at the
/// top left corner `at` on the monitor whose work area is `from`, with an outer size of `size`:
/// as far in from `to`'s top left as it was from `from`'s, moved back in where it would reach
/// past `to`'s right or bottom edge, and at `to`'s top left when it is larger than `to`.
pub fn placed_on(at: (i32, i32), size: (i32, i32), from: Area, to: Area) -> (i32, i32) {
    let along = |at: i32, from_start: i32, to_start: i32, to_end: i32, length: i32| {
        let wanted = to_start + (at - from_start).max(0);
        wanted.min(to_end - length).max(to_start)
    };
    (
        along(at.0, from.0, to.0, to.2, size.0),
        along(at.1, from.1, to.1, to.3, size.1),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn monitor(id: &str, dpi: u32, main: bool, work: Area) -> DesktopMonitor {
        DesktopMonitor {
            id: id.into(),
            dpi,
            main,
            work,
        }
    }

    /// The owner's monitors: a 200% main one and two 125% ones, which have the same size and
    /// scale and differ only by their IDs.
    fn owners() -> Vec<DesktopMonitor> {
        vec![
            monitor("main", 192, true, (0, 0, 2880, 1752)),
            monitor("left", 120, false, (-2560, 0, 0, 1392)),
            monitor("right", 120, false, (2880, 0, 5440, 1392)),
        ]
    }

    #[test]
    fn fr4_a_window_opens_on_the_main_monitor_when_nothing_else_is_known() {
        let monitors = owners();
        assert_eq!(opening_monitor(None, &monitors).unwrap().id, "main");
        assert_eq!(opening_monitor(Some(""), &monitors).unwrap().id, "main");
        // A desktop that names no main monitor opens on its first.
        let mut unnamed = owners();
        unnamed[0].main = false;
        assert_eq!(opening_monitor(None, &unnamed).unwrap().id, "main");
        assert_eq!(opening_monitor(None, &[]), None);
    }

    #[test]
    fn fr4_a_window_moved_onto_another_monitor_keeps_its_place_in_it() {
        // Where the desktop put it on the main monitor, at the same offset on the right one.
        assert_eq!(
            placed_on(
                (100, 80),
                (800, 600),
                (0, 0, 2880, 1752),
                (2880, 0, 5440, 1392)
            ),
            (2980, 80)
        );
        // Kept inside, and at the top left when it is larger than the monitor.
        assert_eq!(
            placed_on(
                (2400, 1200),
                (800, 600),
                (0, 0, 2880, 1752),
                (2880, 0, 5440, 1392)
            ),
            (5440 - 800, 1392 - 600)
        );
        assert_eq!(
            placed_on(
                (10, 10),
                (3000, 2000),
                (0, 0, 2880, 1752),
                (-2560, 0, 0, 1392)
            ),
            (-2560, 0)
        );
    }
}
