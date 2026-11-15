//! Where an app's overlay windows go on the desktop.
//!
//! An app overlay window (floating lyrics, a chat head) is a window an app adds over other
//! apps in Android. The guest reports it like a task ([`crate::tasks::OverlayWindow`]) and the
//! host shows it in a borderless window of its own that stays above the others and has no
//! close button: its app decides when it goes. This module holds the rules of such a window
//! that name no window system; the platform's window host carries them out.
//!
//! **One plane per app.** All of an app's overlays are placed with one translation of the
//! guest's display onto the desktop ([`OverlayPlane`]), at the scale their window shows them
//! at. So two overlays of one app keep the places the app gave them relative to each other: a
//! floating button and the panel it opens beside itself stay side by side wherever the user
//! put the button.
//!
//! **Who moves it.** The app moves an overlay by giving it a new place on the guest's display
//! (many overlays are dragged by touch inside Android); the window then moves by the same
//! distance at its scale. The user moves it with the handle the window shows while the pointer
//! is over it ([`handle_place`]); that moves the plane, so a later move by the app starts from
//! where the user left the window. Neither undoes the other, because each is taken as a
//! distance and never as a place to go back to.
//!
//! **Where it first goes.** On the monitor of its app's window, or the pointer's when the app
//! has none, where it lies on the guest's display: an overlay at the display's right edge is at
//! the work area's right edge ([`first_place`]).
//!
//! **Remembered per app.** The plane is kept for the app ([`remember`], [`remembered`]), so an
//! overlay that its app shows again at the same place on the guest's display comes back where
//! the user left it.
//!
//! **Its size** is what the app gave it, at the density of the monitor it is on, and never
//! more than that monitor's work area: a larger one is shown smaller, in proportion
//! ([`shown_size`]).
//!
//! **The handle** lies outside the overlay, so it covers none of the app's picture and takes
//! none of its input: every press inside the overlay is the app's. A drag with a modifier key
//! held was the alternative; nobody finds it without being told, and a finger has no modifier.

use std::path::{Path, PathBuf};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use super::task_windows::fit_within;
use super::window_hosts::is_valid_package;
use super::window_sizing::{dip_to_px, to_window_position, Scale};

/// A rectangle on the desktop, in its pixels: left, top, right, bottom.
pub type Area = (i32, i32, i32, i32);

/// One point of an app's plane: the place on the guest's display (`guest`, in its pixels) that
/// lies at `host` on the desktop (in desktop pixels).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Anchor {
    pub guest: (i32, i32),
    pub host: (i32, i32),
}

/// How the overlays of one app lie on the desktop: the guest's display moved so that
/// [`Anchor::guest`] lies at [`Anchor::host`], at each window's scale.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OverlayPlane {
    anchor: Option<Anchor>,
}

impl OverlayPlane {
    /// A plane that starts from what was remembered for the app, if anything.
    pub fn new(remembered: Option<Anchor>) -> OverlayPlane {
        OverlayPlane { anchor: remembered }
    }

    /// The plane's anchor, once an overlay has been placed or something was remembered.
    pub fn anchor(&self) -> Option<Anchor> {
        self.anchor
    }

    /// Where the window of an overlay goes: the top left corner of its client area on the
    /// desktop. `guest` is the overlay's top left corner on the guest's display and `scale`
    /// how large the window shows each guest pixel. The first overlay placed on a plane with
    /// nothing remembered puts the plane where `first` says (see [`first_place`]).
    pub fn place(
        &mut self,
        guest: (i32, i32),
        scale: Scale,
        first: impl FnOnce() -> (i32, i32),
    ) -> (i32, i32) {
        let anchor = *self.anchor.get_or_insert_with(|| Anchor {
            guest,
            host: first(),
        });
        let along = |guest: i32, from: i32, host: i32| -> i32 {
            let moved = to_window_position(f64::from(guest) - f64::from(from), scale);
            (f64::from(host) + moved).round() as i32
        };
        (
            along(guest.0, anchor.guest.0, anchor.host.0),
            along(guest.1, anchor.guest.1, anchor.host.1),
        )
    }

    /// The window of the overlay at `guest` on the guest's display is at `host` now: the user
    /// moved it with its handle, or it was brought back onto a monitor. The plane moves with
    /// it, and so does every other overlay of the app. Returns the anchor to remember.
    pub fn moved_to(&mut self, guest: (i32, i32), host: (i32, i32)) -> Anchor {
        let anchor = Anchor { guest, host };
        self.anchor = Some(anchor);
        anchor
    }
}

/// Where an overlay first goes on a monitor's work area `work`: where it lies on the guest's
/// display, edge to edge. `guest` is its top left corner there, `guest_size` its size and
/// `display` the display's size, all in guest pixels; `window` is the size of its window in
/// desktop pixels. An overlay at the display's left edge is at the work area's left edge, one
/// at its right edge at the right edge, and one in between in proportion. With no display
/// size, or an overlay as large as the display, it is centred.
pub fn first_place(
    guest: (i32, i32),
    guest_size: (u32, u32),
    display: (u32, u32),
    window: (u32, u32),
    work: Area,
) -> (i32, i32) {
    let along = |at: i32, size: u32, display: u32, window: u32, start: i32, end: i32| -> i32 {
        let room = i64::from(end) - i64::from(start) - i64::from(window);
        let travel = i64::from(display) - i64::from(size);
        if room <= 0 {
            return start;
        }
        if display == 0 || travel <= 0 {
            return start + (room / 2) as i32;
        }
        let at = i64::from(at).clamp(0, travel);
        start + ((at * room + travel / 2) / travel) as i32
    };
    (
        along(guest.0, guest_size.0, display.0, window.0, work.0, work.2),
        along(guest.1, guest_size.1, display.1, window.1, work.1, work.3),
    )
}

/// The size an overlay's window has on a monitor: `scaled`, the overlay's size at the
/// monitor's density, or the largest size of that shape the work area holds when it is larger.
/// An app sizes an overlay for Android's display, which is as long as the user's longest
/// monitor, so on a smaller monitor a display-wide overlay would reach past the screen.
pub fn shown_size(scaled: (u32, u32), work: Area) -> (u32, u32) {
    let room = (
        (i64::from(work.2) - i64::from(work.0)).max(1) as u32,
        (i64::from(work.3) - i64::from(work.1)).max(1) as u32,
    );
    fit_within(scaled, room)
}

/// `host` moved as little as it takes to keep a window of `size` on the work area `work`:
/// whole when it fits, else with its top left corner on it. An overlay remembered on a
/// monitor that is gone, or that its app moved off the edge of Android's display, is still in
/// reach.
pub fn kept_on(host: (i32, i32), size: (u32, u32), work: Area) -> (i32, i32) {
    let along = |at: i32, size: u32, start: i32, end: i32| -> i32 {
        let last = (i64::from(end) - i64::from(size)).max(i64::from(start));
        i64::from(at).clamp(i64::from(start), last) as i32
    };
    (
        along(host.0, size.0, work.0, work.2),
        along(host.1, size.1, work.1, work.3),
    )
}

/// The handle's size at 100%: a slim bar, wide enough for a finger.
pub const HANDLE_DIP: (u32, u32) = (72, 20);

/// The gap between the overlay and its handle at 100%.
pub const HANDLE_GAP_DIP: u32 = 4;

/// How long the handle stays after the pointer left the overlay and the handle, so the
/// pointer can cross the gap between them, and after a touch on the overlay.
pub const HANDLE_LINGER: Duration = Duration::from_millis(1200);

/// Where the handle of an overlay's window goes, as left, top, width, height in desktop
/// pixels, for a window at `window` (left, top, width, height) on a monitor of `dpi` whose
/// work area is `work`: centred above the window's top edge, outside the overlay, so it covers
/// nothing the app drew and takes none of its input. With no room above (the overlay is at the
/// top of the screen) it goes below the bottom edge, and with no room there either, beside the
/// window's top right corner or, at last, its top left.
pub fn handle_place(window: (i32, i32, u32, u32), dpi: u32, work: Area) -> (i32, i32, u32, u32) {
    let (left, top, width, height) = window;
    let (handle_width, handle_height) = (
        dip_to_px(HANDLE_DIP.0, dpi).max(1),
        dip_to_px(HANDLE_DIP.1, dpi).max(1),
    );
    let gap = dip_to_px(HANDLE_GAP_DIP, dpi) as i32;
    let (w, h) = (handle_width as i32, handle_height as i32);
    let (right, bottom) = (left + width as i32, top + height as i32);
    let centred = (left + (width as i32 - w) / 2).min(work.2 - w).max(work.0);
    let (x, y) = if top - gap - h >= work.1 {
        (centred, top - gap - h)
    } else if bottom + gap + h <= work.3 {
        (centred, bottom + gap)
    } else if right + gap + w <= work.2 {
        (right + gap, top.max(work.1))
    } else {
        ((left - gap - w).max(work.0), top.max(work.1))
    };
    (x, y, handle_width, handle_height)
}

/// Whether a click activates an overlay's window, giving it the keyboard focus: only for an
/// overlay that may take the focus in Android (its app left `FLAG_NOT_FOCUSABLE` off), and
/// then only by the user's click, never when the window opens or its app changes it. A click
/// on any other overlay goes to the app and leaves the user's own window active.
pub fn click_activates(focusable: bool) -> bool {
    focusable
}

/// Whether the handle shows, given how long ago the pointer was last over the overlay or the
/// handle (or a finger last touched the overlay); `None` when it never was. It always shows
/// while the user drags it.
pub fn handle_shown(since_pointer: Option<Duration>, dragging: bool) -> bool {
    dragging || since_pointer.is_some_and(|since| since <= HANDLE_LINGER)
}

/// The file an app's plane is remembered in, in `dir`; `None` for a name that is no package.
fn place_file(dir: &Path, package: &str) -> Option<PathBuf> {
    is_valid_package(package).then(|| dir.join(format!("{package}.overlay.json")))
}

/// The plane remembered for `package` in `dir`, if one was and it can be read.
pub fn remembered(dir: &Path, package: &str) -> Option<Anchor> {
    let path = place_file(dir, package)?;
    crate::settings::read_json::<Anchor>(&path).ok().flatten()
}

/// Remembers `anchor` as the plane of `package`, in `dir`.
pub fn remember(dir: &Path, package: &str, anchor: Anchor) -> Result<(), String> {
    let path = place_file(dir, package).ok_or_else(|| {
        format!("{package} is not a package name; its overlays are not remembered")
    })?;
    crate::settings::write_json(&path, &anchor)
}

#[cfg(test)]
mod tests {
    use super::*;

    const WORK: Area = (0, 0, 2560, 1400);
    const DISPLAY: (u32, u32) = (3840, 3840);

    #[test]
    fn fr41_an_overlay_first_opens_where_it_lies_on_androids_display() {
        // Floating lyrics at (158, 82) of a 3840 pixel display, shown 318 by 86 on the monitor.
        let at = first_place((158, 82), (508, 138), DISPLAY, (318, 86), WORK);
        assert_eq!(at, (106, 29));
        // Edge to edge: flush left stays flush left, flush right and bottom stay flush.
        assert_eq!(
            first_place((0, 0), (508, 138), DISPLAY, (318, 86), WORK),
            (0, 0)
        );
        assert_eq!(
            first_place((3332, 3702), (508, 138), DISPLAY, (318, 86), WORK),
            (2560 - 318, 1400 - 86)
        );
        // On a monitor that does not start at the desktop's origin.
        let second = (2560, -200, 4480, 880);
        assert_eq!(
            first_place((3332, 0), (508, 138), DISPLAY, (318, 86), second),
            (4480 - 318, -200)
        );
        // A place off the display is taken as the nearest edge.
        assert_eq!(
            first_place((-50, 9000), (508, 138), DISPLAY, (318, 86), WORK),
            (0, 1314)
        );
        // Without the display's size, in the middle.
        assert_eq!(
            first_place((158, 82), (508, 138), (0, 0), (318, 86), WORK),
            (1121, 657)
        );
    }

    #[test]
    fn fr41_a_move_by_the_app_moves_the_window_the_same_distance_at_its_scale() {
        let mut plane = OverlayPlane::new(None);
        let first = plane.place((158, 82), (1, 1), || (106, 29));
        assert_eq!(first, (106, 29));
        // The app drags it 100 right and 40 down: so does the window, pixel for pixel.
        assert_eq!(
            plane.place((258, 122), (1, 1), || unreachable!()),
            (206, 69)
        );
        // On a monitor at half the density, half the distance.
        let mut plane = OverlayPlane::new(None);
        plane.place((158, 82), (1, 2), || (106, 29));
        assert_eq!(
            plane.place((258, 122), (1, 2), || unreachable!()),
            (156, 49)
        );
    }

    #[test]
    fn fr41_a_user_move_and_an_app_move_add_up() {
        let mut plane = OverlayPlane::new(None);
        plane.place((158, 82), (1, 1), || (106, 29));
        // The user drags the handle: the window is at (900, 600) now.
        plane.moved_to((158, 82), (900, 600));
        assert_eq!(
            plane.place((158, 82), (1, 1), || unreachable!()),
            (900, 600)
        );
        // The app then moves it 30 left: from where the user left it, not from where it was.
        assert_eq!(
            plane.place((128, 82), (1, 1), || unreachable!()),
            (870, 600)
        );
        // The user moves it again while the app has it there, and the app moves it back to its
        // first place: 30 right of where the user dropped it.
        plane.moved_to((128, 82), (300, 300));
        assert_eq!(
            plane.place((158, 82), (1, 1), || unreachable!()),
            (330, 300)
        );
    }

    #[test]
    fn fr41_overlays_of_one_app_keep_their_places_relative_to_each_other() {
        let mut plane = OverlayPlane::new(None);
        // A floating button, which the user moves.
        plane.place((3600, 1800), (1, 1), || (2400, 656));
        plane.moved_to((3600, 1800), (400, 300));
        // The panel the app opens 200 pixels left of the button opens 200 left of it.
        assert_eq!(
            plane.place((3400, 1800), (1, 1), || unreachable!()),
            (200, 300)
        );
    }

    #[test]
    fn fr41_the_place_the_user_chose_is_remembered_per_app() {
        let dir = std::env::temp_dir().join(format!(
            "dsa-overlay-places-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(remembered(&dir, "skplanet.musicmate"), None);

        let mut plane = OverlayPlane::new(None);
        plane.place((158, 82), (1, 1), || (106, 29));
        let anchor = plane.moved_to((158, 82), (3000, 900));
        remember(&dir, "skplanet.musicmate", anchor).unwrap();

        // The app shows its overlay again, at the same place on Android's display.
        let mut again = OverlayPlane::new(remembered(&dir, "skplanet.musicmate"));
        assert_eq!(
            again.place((158, 82), (1, 1), || unreachable!()),
            (3000, 900)
        );
        // Another app's overlays are placed afresh.
        assert_eq!(remembered(&dir, "com.example.chat"), None);
        // A name that is no package names no file.
        assert!(remember(&dir, "..\\elsewhere", anchor).is_err());
        assert_eq!(remembered(&dir, "..\\elsewhere"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn fr41_an_overlay_is_never_larger_than_its_monitors_work_area() {
        // What fits keeps its size.
        assert_eq!(shown_size((318, 86), WORK), (318, 86));
        // A display-wide bar on a narrower monitor is shown smaller, in proportion.
        assert_eq!(shown_size((3840, 200), WORK), (2560, 133));
        // A display-high panel too.
        assert_eq!(shown_size((300, 3840), WORK), (109, 1400));
    }

    #[test]
    fn fr41_a_window_off_every_monitor_is_brought_back() {
        assert_eq!(kept_on((900, 600), (318, 86), WORK), (900, 600));
        assert_eq!(kept_on((5000, -300), (318, 86), WORK), (2560 - 318, 0));
        // One larger than the work area keeps its top left corner in reach.
        assert_eq!(kept_on((-900, 50), (4000, 86), WORK), (0, 50));
    }

    #[test]
    fn fr41_the_handle_lies_outside_the_overlay() {
        let window = (900, 600, 318, 86);
        let (x, y, width, height) = handle_place(window, 96, WORK);
        assert_eq!((width, height), HANDLE_DIP);
        // Centred above the top edge, a gap away: it covers no pixel of the overlay.
        assert_eq!((x, y), (900 + (318 - 72) / 2, 600 - 4 - 20));
        assert!(y + height as i32 <= 600);
        // At 200% it is twice as large.
        assert_eq!(handle_place(window, 192, WORK).2, 144);
        // At the top of the screen it goes below the overlay.
        let (_, y, _, _) = handle_place((900, 0, 318, 86), 96, WORK);
        assert_eq!(y, 86 + 4);
        // An overlay as high as the work area has it beside its top right corner.
        let (x, y, _, _) = handle_place((900, 0, 318, 1400), 96, WORK);
        assert_eq!((x, y), (900 + 318 + 4, 0));
        // And one as large as the work area keeps it on the screen.
        let (x, y, _, _) = handle_place((0, 0, 2560, 1400), 96, WORK);
        assert_eq!((x, y), (0, 0));
    }

    #[test]
    fn fr41_the_handle_shows_only_around_the_pointer_or_a_touch() {
        assert!(!handle_shown(None, false));
        assert!(handle_shown(Some(Duration::ZERO), false));
        assert!(handle_shown(Some(HANDLE_LINGER), false));
        assert!(!handle_shown(
            Some(HANDLE_LINGER + Duration::from_millis(1)),
            false
        ));
        // It never goes away under the user's hand.
        assert!(handle_shown(None, true));
    }

    #[test]
    fn fr41_only_a_focusable_overlay_takes_the_focus_and_only_by_a_click() {
        assert!(!click_activates(false));
        assert!(click_activates(true));
    }
}
