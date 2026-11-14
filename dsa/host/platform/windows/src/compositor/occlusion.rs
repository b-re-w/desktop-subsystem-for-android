//! Which task windows the user cannot see although they are open and not minimized: on
//! another virtual desktop (cloaked by the desktop window manager), or wholly covered by the
//! windows above them.
//!
//! The window manager reads it from the task windows' own marks ([`window_marks`]), which any
//! process of the user may read, and the z-order of the desktop's top-level windows. A window
//! counts as covering only when it is visible, not minimized, not cloaked, and opaque to the
//! eye: a layered or click-through window (a translucent overlay, a notification, a tooltip)
//! may let the task show through, so it never counts. What a window covers is its frame as the
//! desktop window manager draws it, without the invisible resize borders `GetWindowRect`
//! includes, and only the part of a task window that is on a monitor counts as seen.
//!
//! [`window_marks`]: super::window_marks

use std::collections::BTreeSet;
use std::ffi::c_void;

use dsa_host_core::compositor::shown_windows::{wholly_covered, ScreenRect};
use windows_sys::Win32::Foundation::{HWND, RECT};
use windows_sys::Win32::Graphics::Dwm::{
    DwmGetWindowAttribute, DWMWA_CLOAKED, DWMWA_EXTENDED_FRAME_BOUNDS,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, GetWindow, GetWindowLongPtrW, GetWindowRect, IsIconic, IsWindowVisible,
    GWL_EXSTYLE, GW_HWNDPREV, SM_CXVIRTUALSCREEN, SM_CYVIRTUALSCREEN, SM_XVIRTUALSCREEN,
    SM_YVIRTUALSCREEN, WS_EX_LAYERED, WS_EX_TRANSPARENT,
};

use super::window_marks;

/// The most windows above a task window that are looked at; a desktop has far fewer.
const MAX_ABOVE: usize = 4096;

/// Whether the desktop window manager cloaks `hwnd`: on another virtual desktop, or hidden
/// by the shell.
unsafe fn cloaked(hwnd: HWND) -> bool {
    let mut value: u32 = 0;
    DwmGetWindowAttribute(
        hwnd,
        DWMWA_CLOAKED as u32,
        &mut value as *mut u32 as *mut c_void,
        std::mem::size_of::<u32>() as u32,
    ) == 0
        && value != 0
}

/// The frame of `hwnd` as the desktop window manager draws it, or its window rectangle when
/// it does not say.
unsafe fn frame(hwnd: HWND) -> Option<ScreenRect> {
    let mut rect: RECT = std::mem::zeroed();
    let drawn = DwmGetWindowAttribute(
        hwnd,
        DWMWA_EXTENDED_FRAME_BOUNDS as u32,
        &mut rect as *mut RECT as *mut c_void,
        std::mem::size_of::<RECT>() as u32,
    ) == 0;
    if !drawn && GetWindowRect(hwnd, &mut rect) == 0 {
        return None;
    }
    Some((rect.left, rect.top, rect.right, rect.bottom))
}

/// Whether `hwnd` hides what is under it wherever it is.
unsafe fn covers(hwnd: HWND) -> bool {
    if IsWindowVisible(hwnd) == 0 || IsIconic(hwnd) != 0 || cloaked(hwnd) {
        return false;
    }
    let style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32;
    style & (WS_EX_LAYERED | WS_EX_TRANSPARENT) == 0
}

/// The part of the screen any monitor shows.
unsafe fn desktop() -> ScreenRect {
    let left = GetSystemMetrics(SM_XVIRTUALSCREEN);
    let top = GetSystemMetrics(SM_YVIRTUALSCREEN);
    (
        left,
        top,
        left + GetSystemMetrics(SM_CXVIRTUALSCREEN),
        top + GetSystemMetrics(SM_CYVIRTUALSCREEN),
    )
}

/// The task windows (by the task each shows, as the window marks it) that are open and not
/// minimized but that the user cannot see now: cloaked, or wholly covered by the windows above.
pub fn hidden_task_windows() -> BTreeSet<i32> {
    let mut hidden = BTreeSet::new();
    // SAFETY: reads of other windows' state, which fail harmlessly for a window that has gone
    // meanwhile; nothing is written.
    unsafe {
        let screen = desktop();
        for (hwnd, task_id) in window_marks::task_windows() {
            let hwnd = hwnd as HWND;
            if IsWindowVisible(hwnd) == 0 || IsIconic(hwnd) != 0 {
                // Not shown at all; a minimized window is the window host's to report.
                continue;
            }
            if cloaked(hwnd) {
                hidden.insert(task_id);
                continue;
            }
            let Some((left, top, right, bottom)) = frame(hwnd) else {
                continue;
            };
            let target = (
                left.max(screen.0),
                top.max(screen.1),
                right.min(screen.2),
                bottom.min(screen.3),
            );
            let mut above = Vec::new();
            let mut next = GetWindow(hwnd, GW_HWNDPREV);
            while !next.is_null() && above.len() < MAX_ABOVE {
                if covers(next) {
                    if let Some(rect) = frame(next) {
                        above.push(rect);
                    }
                }
                next = GetWindow(next, GW_HWNDPREV);
            }
            if wholly_covered(target, &above) {
                hidden.insert(task_id);
            }
        }
    }
    hidden
}
