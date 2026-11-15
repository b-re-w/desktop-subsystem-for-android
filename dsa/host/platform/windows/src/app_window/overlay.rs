//! The windows of an app's overlay windows: floating lyrics, a chat head, anything an app adds
//! over other apps in Android with the "display over other apps" permission.
//!
//! Each overlay is a window of its own on the windows' thread, beside the app's task windows,
//! and nothing like them: it has no border, no title bar and no close button (its app decides
//! when it goes), stays above every other window, is in neither the taskbar nor Alt+Tab, and
//! shows the desktop through whatever its app left clear, since its frames keep their own
//! transparency ([`WindowPresenter::new_see_through`]). The rules it follows name no window
//! system and live in [`dsa_host_core::compositor::overlay_windows`]; this file carries them
//! out.
//!
//! - **Where it is.** All of the app's overlays share one [`OverlayPlane`]. The window manager
//!   says where the app has each overlay on Android's display (`ToHost::Overlay`), and the
//!   plane says where that is on the desktop. A window is first shown once it knows.
//! - **Its size** is the overlay's at the scale of its monitor, as the window manager works it
//!   out from the DPI the window reports, and never more than the monitor's work area.
//! - **Input.** Every press, move and wheel turn inside the window is the app's, relative to
//!   the overlay. A click never takes the focus from the window the user works in, unless the
//!   overlay may take the focus in Android; then the click activates it, and keys and text go
//!   to the overlay. An overlay that takes no input lets the pointer through to what is under
//!   it.
//! - **The handle.** While the pointer is over the overlay (the window looks ten times a
//!   second, since an overlay that takes no input hears of no pointer), or a little after a
//!   touch on it, a slim handle shows just outside its top edge. Dragging the handle moves the
//!   overlay, and with it the app's plane, which is remembered for the app in the user's DSA
//!   folder (`apps\<package>.overlay.json`). The handle is a window of its own, so it covers
//!   no pixel of the overlay and takes none of its input.
//!
//! Such a window is an ordinary window of the user's desktop: Windows shows none of them on
//! the lock screen or the secure desktop, and it takes no input outside its own rectangle.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use dsa_host_core::compositor::overlay_windows::{
    click_activates, first_place, handle_place, handle_shown, kept_on, remember, remembered,
    shown_size, Area, OverlayPlane,
};
use dsa_host_core::compositor::window_hosts::{FromHost, ToHost};
use dsa_host_core::compositor::window_sizing::to_window_size;
use dsa_host_core::tasks::OVERLAY_ID_BASE;
use dsa_host_core::ui_language::UiLanguage;
use windows_sys::Win32::Foundation::{
    GetLastError, ERROR_CLASS_ALREADY_EXISTS, HWND, LPARAM, LRESULT, POINT, RECT, WPARAM,
};
use windows_sys::Win32::Graphics::Gdi::{
    BeginPaint, CreateRoundRectRgn, CreateSolidBrush, DeleteObject, EndPaint, FillRect,
    GetMonitorInfoW, MonitorFromPoint, MonitorFromRect, MonitorFromWindow, SetWindowRgn,
    MONITORINFO, MONITOR_DEFAULTTONEAREST, PAINTSTRUCT,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::HiDpi::GetDpiForWindow;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetCapture, ReleaseCapture, SetCapture};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, GetClientRect, GetCursorPos, GetWindowLongPtrW,
    GetWindowRect, IsWindowVisible, KillTimer, LoadCursorW, RegisterClassExW,
    SetLayeredWindowAttributes, SetTimer, SetWindowLongPtrW, SetWindowPos, SetWindowTextW,
    ShowWindow, CREATESTRUCTW, GWLP_USERDATA, GWL_EXSTYLE, HTCLIENT, HWND_TOPMOST, IDC_ARROW,
    IDC_SIZEALL, LWA_ALPHA, MA_ACTIVATE, MA_NOACTIVATE, SWP_FRAMECHANGED, SWP_NOACTIVATE,
    SWP_NOMOVE, SWP_NOSIZE, SWP_NOZORDER, SW_HIDE, SW_SHOWNOACTIVATE, WM_CAPTURECHANGED, WM_CLOSE,
    WM_DPICHANGED, WM_ERASEBKGND, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEACTIVATE, WM_MOUSEMOVE,
    WM_NCCREATE, WM_NCDESTROY, WM_NCHITTEST, WM_PAINT, WM_POINTERDOWN, WM_SIZE, WM_TIMER,
    WNDCLASSEXW, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_NOREDIRECTIONBITMAP, WS_EX_TOOLWINDOW,
    WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_POPUP,
};

use crate::compositor::log::note;
use crate::compositor::present::WindowPresenter;
use crate::ui_language::fill;

use super::ime::ImeWindow;
use super::keys::KeyWindow;
use super::pointer::PointerWindow;
use super::window::{Report, UiSetup};

const OVERLAY_CLASS: &str = "DsaOverlayWindow";
const HANDLE_CLASS: &str = "DsaOverlayHandle";

/// The overlay window's timer, on which it looks where the pointer is.
const HOVER_TIMER: usize = 1;
const HOVER_MILLIS: u32 = 100;

/// The handle's colors, as 0x00BBGGRR: a dark bar with a light grip, readable over anything.
const HANDLE_FILL: u32 = 0x0030_3030;
const HANDLE_GRIP: u32 = 0x00D0_D0D0;
/// How opaque the handle is, of 255.
const HANDLE_ALPHA: u8 = 235;

/// Whether `task_id` names an app overlay window rather than a task.
pub(super) fn is_overlay(task_id: i32) -> bool {
    task_id >= OVERLAY_ID_BASE
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16()
        .filter(|unit| *unit != 0)
        .chain(std::iter::once(0))
        .collect()
}

/// What the handle of an overlay is called, for a screen reader: `{name}` is the overlay's
/// title, which is its app's name unless the app gave the window one.
pub fn handle_name(language: UiLanguage) -> &'static str {
    match language {
        UiLanguage::English => "Move {name}",
        UiLanguage::Korean => "{name} 이동",
        UiLanguage::Japanese => "{name} を移動",
        UiLanguage::ChineseSimplified => "移动 {name}",
        UiLanguage::ChineseTraditional => "移動 {name}",
        UiLanguage::Spanish => "Mover {name}",
        UiLanguage::French => "Déplacer {name}",
        UiLanguage::German => "{name} verschieben",
        UiLanguage::PortugueseBrazil => "Mover {name}",
        UiLanguage::Russian => "Переместить {name}",
        UiLanguage::Italian => "Sposta {name}",
        UiLanguage::Vietnamese => "Di chuyển {name}",
        UiLanguage::Indonesian => "Pindahkan {name}",
        UiLanguage::Turkish => "{name} öğesini taşı",
    }
}

/// Kept in each overlay window's GWLP_USERDATA.
struct OverlayData {
    task_id: i32,
    report: Report,
    presenter: RefCell<Option<Arc<WindowPresenter>>>,
    pointer: PointerWindow,
    keys: KeyWindow,
    ime: ImeWindow,
    /// The overlay's title, which the handle is named after.
    title: RefCell<String>,
    /// The overlay's top left corner on Android's display; `None` until the window manager
    /// said, and the window is shown only then.
    guest: Cell<Option<(i32, i32)>>,
    /// The size of Android's display.
    display: Cell<(u32, u32)>,
    /// The overlay's size in Android's pixels, and how large the window shows each of them.
    guest_size: Cell<(u32, u32)>,
    scale: Cell<(u32, u32)>,
    focusable: Cell<bool>,
    touchable: Cell<bool>,
    /// The handle's window, once it has been shown; null before.
    handle: Cell<HWND>,
    /// When the pointer was last over the overlay or its handle, or a finger last touched it.
    pointed: Cell<Option<Instant>>,
    /// While the user drags the handle: where the pointer and the window were when it began.
    drag: Cell<Option<DragStart>>,
}

/// Where a drag of the handle began: the pointer's place on the desktop, and the window's.
type DragStart = ((i32, i32), (i32, i32));

/// The overlays of this window host's app.
#[derive(Default)]
struct App {
    plane: OverlayPlane,
    /// Whether what was remembered for the app has been read.
    read: bool,
    windows: HashMap<i32, isize>,
    package: String,
    places: Option<PathBuf>,
}

thread_local! {
    static APP: RefCell<App> = RefCell::new(App::default());
}

unsafe fn data_of<'a>(hwnd: HWND) -> Option<&'a OverlayData> {
    (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const OverlayData).as_ref()
}

unsafe fn register(
    name: &str,
    procedure: unsafe extern "system" fn(HWND, u32, WPARAM, LPARAM) -> LRESULT,
    cursor: *const u16,
) -> Result<(), String> {
    let name = wide(name);
    let class = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: 0,
        lpfnWndProc: Some(procedure),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: GetModuleHandleW(std::ptr::null()),
        hIcon: std::ptr::null_mut(),
        hCursor: LoadCursorW(std::ptr::null_mut(), cursor),
        hbrBackground: std::ptr::null_mut(),
        lpszMenuName: std::ptr::null(),
        lpszClassName: name.as_ptr(),
        hIconSm: std::ptr::null_mut(),
    };
    if RegisterClassExW(&class) == 0 && GetLastError() != ERROR_CLASS_ALREADY_EXISTS {
        return Err(format!(
            "could not register an overlay window class: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

fn area(rect: &RECT) -> Area {
    (rect.left, rect.top, rect.right, rect.bottom)
}

unsafe fn work_of(monitor: *mut core::ffi::c_void) -> Option<RECT> {
    let mut info: MONITORINFO = std::mem::zeroed();
    info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
    (!monitor.is_null() && GetMonitorInfoW(monitor, &mut info) != 0).then_some(info.rcWork)
}

/// The work area of the monitor an app's first overlay goes to: the one of the app's window
/// `beside`, or the pointer's when the app has no window.
unsafe fn first_work_area(beside: Option<HWND>) -> Option<RECT> {
    if let Some(window) = beside {
        return work_of(MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST));
    }
    let mut pointer = POINT { x: 0, y: 0 };
    if GetCursorPos(&mut pointer) == 0 {
        return None;
    }
    work_of(MonitorFromPoint(pointer, MONITOR_DEFAULTTONEAREST))
}

/// The styles of an overlay's window: never in the taskbar, always on top, drawn only by its
/// visual; not activated by a click unless the overlay may take the focus; and let through by
/// the pointer when the overlay takes no input.
fn ex_style(focusable: bool, touchable: bool) -> u32 {
    let mut style = WS_EX_NOREDIRECTIONBITMAP | WS_EX_TOPMOST | WS_EX_TOOLWINDOW;
    if !click_activates(focusable) {
        style |= WS_EX_NOACTIVATE;
    }
    if !touchable {
        style |= WS_EX_LAYERED | WS_EX_TRANSPARENT;
    }
    style
}

/// Carries out what the window manager says about an overlay's window. `beside` is one of the
/// app's task windows, if it has any.
///
/// # Safety
///
/// Called on the windows' thread.
pub(super) unsafe fn command(setup: &UiSetup, beside: Option<HWND>, command: ToHost) {
    let window = |task_id: i32| -> Option<HWND> {
        APP.with(|app| app.borrow().windows.get(&task_id).map(|hwnd| *hwnd as HWND))
    };
    match command {
        ToHost::Show {
            task_id,
            title,
            width,
            height,
            ..
        } => open(setup, beside, task_id, &title, (width, height)),
        ToHost::Retitle { task_id, title } => {
            if let Some(hwnd) = window(task_id) {
                SetWindowTextW(hwnd, wide(&title).as_ptr());
                if let Some(data) = data_of(hwnd) {
                    data.title.replace(title);
                    name_handle(data);
                }
            }
        }
        // The window's size is its overlay's at its scale; both come with TaskSize too.
        ToHost::Resize { task_id, .. } => {
            if let Some(hwnd) = window(task_id) {
                place(hwnd);
            }
        }
        ToHost::TaskSize {
            task_id,
            width,
            height,
            scale,
            ..
        } => {
            if let Some((hwnd, data)) = window(task_id).and_then(|h| Some((h, data_of(h)?))) {
                data.guest_size.set((width.max(1), height.max(1)));
                data.scale.set(scale);
                data.pointer.task_resized((width, height));
                place(hwnd);
            }
        }
        ToHost::Overlay {
            task_id,
            left,
            top,
            display,
            focusable,
            touchable,
        } => {
            if let Some((hwnd, data)) = window(task_id).and_then(|h| Some((h, data_of(h)?))) {
                let first = data.guest.replace(Some((left, top))).is_none();
                data.display.set(display);
                let flags = (
                    data.focusable.replace(focusable),
                    data.touchable.replace(touchable),
                );
                if first || flags != (focusable, touchable) {
                    restyle(hwnd, focusable, touchable);
                }
                place(hwnd);
                if first {
                    // Never active when it appears: only the user's click may activate it.
                    ShowWindow(hwnd, SW_SHOWNOACTIVATE);
                    SetTimer(hwnd, HOVER_TIMER, HOVER_MILLIS, None);
                }
            }
        }
        ToHost::Close { task_id } => close(setup, task_id),
        // An overlay's window has one mode, no title bar and no editor of its own to describe.
        _ => {}
    }
}

/// Closes every overlay window, as when the windows' thread ends.
pub(super) fn close_all(setup: &UiSetup) {
    let open: Vec<i32> = APP.with(|app| app.borrow().windows.keys().copied().collect());
    for task_id in open {
        close(setup, task_id);
    }
}

fn close(setup: &UiSetup, task_id: i32) {
    let presenter = setup
        .presenters
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .remove(&task_id);
    let hwnd = APP.with(|app| app.borrow_mut().windows.remove(&task_id));
    if let Some(presenter) = &presenter {
        presenter.close();
    }
    if let Some(hwnd) = hwnd {
        // SAFETY: a window of this thread.
        unsafe { DestroyWindow(hwnd as HWND) };
    }
    if let Some(presenter) = presenter {
        presenter.release();
    }
}

unsafe fn open(setup: &UiSetup, beside: Option<HWND>, task_id: i32, title: &str, size: (u32, u32)) {
    if APP.with(|app| app.borrow().windows.contains_key(&task_id)) {
        return;
    }
    if let Err(why) = register(OVERLAY_CLASS, overlay_proc, IDC_ARROW)
        .and_then(|()| register(HANDLE_CLASS, handle_proc, IDC_SIZEALL))
    {
        note(setup.logger.as_deref(), &why);
        return;
    }
    APP.with(|app| {
        let mut app = app.borrow_mut();
        if !app.read {
            // What the user chose for this app's overlays the last time.
            app.read = true;
            app.package = setup.package.clone();
            app.places = setup.overlay_places.clone();
            let kept = app
                .places
                .as_deref()
                .and_then(|dir| remembered(dir, &app.package));
            app.plane = OverlayPlane::new(kept);
        }
    });
    let size = (size.0.max(1), size.1.max(1));
    let data = Box::into_raw(Box::new(OverlayData {
        task_id,
        report: Arc::clone(&setup.report),
        presenter: RefCell::new(None),
        pointer: PointerWindow::new(task_id, size, setup.input.clone(), None),
        keys: KeyWindow::new(task_id, setup.input.clone()),
        ime: ImeWindow::new(task_id, setup.input.clone()),
        title: RefCell::new(title.to_string()),
        guest: Cell::new(None),
        display: Cell::new((0, 0)),
        guest_size: Cell::new(size),
        scale: Cell::new((1, 1)),
        focusable: Cell::new(false),
        touchable: Cell::new(true),
        handle: Cell::new(std::ptr::null_mut()),
        pointed: Cell::new(None),
        drag: Cell::new(None),
    }));
    // On the monitor it will first be placed on, so the DPI it reports is that monitor's.
    let work = first_work_area(beside);
    let (x, y) = work.map_or((0, 0), |work| (work.left, work.top));
    let hwnd = CreateWindowExW(
        ex_style(false, true),
        wide(OVERLAY_CLASS).as_ptr(),
        wide(title).as_ptr(),
        WS_POPUP,
        x,
        y,
        size.0 as i32,
        size.1 as i32,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        GetModuleHandleW(std::ptr::null()),
        data as *const std::ffi::c_void,
    );
    if hwnd.is_null() {
        drop(Box::from_raw(data));
        note(
            setup.logger.as_deref(),
            &format!(
                "could not open the window of overlay {task_id}: {}",
                std::io::Error::last_os_error()
            ),
        );
        return;
    }
    super::pointer::deliver_contacts_at_once(hwnd);
    let Some(data) = data_of(hwnd) else {
        return;
    };
    if let Err(why) = data.ime.attach(hwnd) {
        note(
            setup.logger.as_deref(),
            &format!("overlay {task_id}'s window takes no input method: {why}"),
        );
    }
    // Its frames are shown in it as they come, at its scale, see-through where the app drew
    // nothing; the window itself takes the size the window manager says.
    data.pointer.set_held(true);
    if let Some(gpu) = &setup.gpu {
        match WindowPresenter::new_see_through(
            task_id,
            Arc::clone(gpu),
            hwnd as isize,
            size,
            setup.logger.clone(),
        ) {
            Ok(presenter) => {
                presenter.set_held(true);
                *data.presenter.borrow_mut() = Some(Arc::clone(&presenter));
                setup
                    .presenters
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .insert(task_id, presenter);
            }
            Err(why) => note(
                setup.logger.as_deref(),
                &format!("overlay {task_id}'s window cannot show frames: {why}"),
            ),
        }
    }
    APP.with(|app| app.borrow_mut().windows.insert(task_id, hwnd as isize));
    // The scale of its monitor, which the window manager answers with the overlay's size there.
    (setup.report)(FromHost::Dpi {
        task_id,
        dpi: GetDpiForWindow(hwnd).max(96),
    });
    note(
        setup.logger.as_deref(),
        &format!(
            "opened the window of app overlay window {task_id} ({}x{})",
            size.0, size.1
        ),
    );
}

/// Gives the window the styles of an overlay that may or may not take the focus and input.
unsafe fn restyle(hwnd: HWND, focusable: bool, touchable: bool) {
    let style = ex_style(focusable, touchable);
    SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style as isize);
    if style & WS_EX_LAYERED != 0 {
        // A layered window shows nothing until it is given its opacity.
        SetLayeredWindowAttributes(hwnd, 0, 255, LWA_ALPHA);
    }
    SetWindowPos(
        hwnd,
        std::ptr::null_mut(),
        0,
        0,
        0,
        0,
        SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE | SWP_FRAMECHANGED,
    );
}

/// Puts the window where its overlay is on the app's plane, at the overlay's size on its
/// monitor. Nothing until the window manager has said where the overlay is.
unsafe fn place(hwnd: HWND) {
    let Some(data) = data_of(hwnd) else {
        return;
    };
    let Some(guest) = data.guest.get() else {
        return;
    };
    let Some(work) = work_of(MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST)) else {
        return;
    };
    let scale = data.scale.get();
    let guest_size = data.guest_size.get();
    let scaled = to_window_size(guest_size, scale);
    let size = shown_size(scaled, area(&work));
    // A window shown smaller than its overlay's size there shows the frames, and measures the
    // pointer, at the scale it has.
    let shown_scale = if size == scaled {
        scale
    } else {
        (size.0.max(1), guest_size.0.max(1))
    };
    data.pointer.set_scale(shown_scale);
    if let Some(presenter) = data.presenter.borrow().as_deref() {
        presenter.set_scale(shown_scale);
    }
    let display = data.display.get();
    let wanted = APP.with(|app| {
        app.borrow_mut().plane.place(guest, scale, || {
            first_place(guest, guest_size, display, size, area(&work))
        })
    });
    // Kept in reach on the monitor that place is on.
    let rect = RECT {
        left: wanted.0,
        top: wanted.1,
        right: wanted.0 + size.0 as i32,
        bottom: wanted.1 + size.1 as i32,
    };
    let on = work_of(MonitorFromRect(&rect, MONITOR_DEFAULTTONEAREST)).unwrap_or(work);
    let host = kept_on(wanted, size, area(&on));
    if host != wanted {
        APP.with(|app| app.borrow_mut().plane.moved_to(guest, host));
    }
    SetWindowPos(
        hwnd,
        HWND_TOPMOST,
        host.0,
        host.1,
        size.0 as i32,
        size.1 as i32,
        SWP_NOACTIVATE,
    );
    place_handle(hwnd, data);
}

/// Puts every overlay window of the app where the plane has it now.
unsafe fn place_all() {
    let open: Vec<isize> = APP.with(|app| app.borrow().windows.values().copied().collect());
    for hwnd in open {
        place(hwnd as HWND);
    }
}

/// The handle's rectangle for the overlay window `hwnd`, as it is placed now.
unsafe fn handle_rect(hwnd: HWND) -> Option<(i32, i32, u32, u32)> {
    let mut window: RECT = std::mem::zeroed();
    if GetWindowRect(hwnd, &mut window) == 0 {
        return None;
    }
    let work = work_of(MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST))?;
    Some(handle_place(
        (
            window.left,
            window.top,
            (window.right - window.left).max(1) as u32,
            (window.bottom - window.top).max(1) as u32,
        ),
        GetDpiForWindow(hwnd).max(96),
        area(&work),
    ))
}

/// Moves the handle with its overlay's window, if it has been made.
unsafe fn place_handle(hwnd: HWND, data: &OverlayData) {
    let handle = data.handle.get();
    if handle.is_null() {
        return;
    }
    let Some((x, y, width, height)) = handle_rect(hwnd) else {
        return;
    };
    SetWindowPos(
        handle,
        HWND_TOPMOST,
        x,
        y,
        width as i32,
        height as i32,
        SWP_NOACTIVATE,
    );
    // A bar with round ends.
    let round = height as i32;
    let region = CreateRoundRectRgn(0, 0, width as i32 + 1, height as i32 + 1, round, round);
    // The window owns the region from here on.
    SetWindowRgn(handle, region, 1);
}

/// Names the handle after its overlay, for a screen reader.
unsafe fn name_handle(data: &OverlayData) {
    let handle = data.handle.get();
    if handle.is_null() {
        return;
    }
    let name = fill(
        handle_name(crate::ui_language::user_language()),
        &[("name", data.title.borrow().as_str())],
    );
    SetWindowTextW(handle, wide(&name).as_ptr());
}

/// Shows or hides the handle of the overlay window `hwnd`, making it the first time.
unsafe fn show_handle(hwnd: HWND, data: &OverlayData, shown: bool) {
    let mut handle = data.handle.get();
    if handle.is_null() {
        if !shown {
            return;
        }
        handle = CreateWindowExW(
            WS_EX_TOPMOST | WS_EX_TOOLWINDOW | WS_EX_NOACTIVATE | WS_EX_LAYERED,
            wide(HANDLE_CLASS).as_ptr(),
            std::ptr::null(),
            WS_POPUP,
            0,
            0,
            1,
            1,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            GetModuleHandleW(std::ptr::null()),
            // The handle knows its overlay's window.
            hwnd as *const std::ffi::c_void,
        );
        if handle.is_null() {
            return;
        }
        SetLayeredWindowAttributes(handle, 0, HANDLE_ALPHA, LWA_ALPHA);
        data.handle.set(handle);
        name_handle(data);
    }
    let visible = IsWindowVisible(handle) != 0;
    if shown && !visible {
        place_handle(hwnd, data);
        ShowWindow(handle, SW_SHOWNOACTIVATE);
    } else if !shown && visible {
        ShowWindow(handle, SW_HIDE);
    }
}

fn inside(point: POINT, rect: (i32, i32, i32, i32)) -> bool {
    point.x >= rect.0 && point.y >= rect.1 && point.x < rect.2 && point.y < rect.3
}

/// Looks where the pointer is: the handle shows while it is over the overlay or the handle,
/// and for a moment after.
unsafe fn follow_pointer(hwnd: HWND, data: &OverlayData) {
    let mut pointer = POINT { x: 0, y: 0 };
    let mut window: RECT = std::mem::zeroed();
    if GetCursorPos(&mut pointer) != 0 && GetWindowRect(hwnd, &mut window) != 0 {
        let over_handle = handle_rect(hwnd).is_some_and(|(x, y, width, height)| {
            inside(pointer, (x, y, x + width as i32, y + height as i32))
        });
        if inside(pointer, area(&window)) || (over_handle && !data.handle.get().is_null()) {
            data.pointed.set(Some(Instant::now()));
        }
    }
    let since = data.pointed.get().map(|at| at.elapsed());
    show_handle(hwnd, data, handle_shown(since, data.drag.get().is_some()));
}

unsafe extern "system" fn overlay_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_NCCREATE {
        let create = &*(lparam as *const CREATESTRUCTW);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, create.lpCreateParams as isize);
        return DefWindowProcW(hwnd, message, wparam, lparam);
    }
    let Some(data) = data_of(hwnd) else {
        return DefWindowProcW(hwnd, message, wparam, lparam);
    };
    match message {
        // Only an overlay that may take the focus is activated, and only by a click.
        WM_MOUSEACTIVATE => {
            return if click_activates(data.focusable.get()) {
                MA_ACTIVATE as LRESULT
            } else {
                MA_NOACTIVATE as LRESULT
            };
        }
        // All of it is the app's: no border to resize by, no caption to drag by.
        WM_NCHITTEST => return HTCLIENT as LRESULT,
        // No close button, and Alt+F4 closes nothing: the app removes its overlay.
        WM_CLOSE => return 0,
        WM_ERASEBKGND => return 1,
        WM_PAINT => {
            let mut paint: PAINTSTRUCT = std::mem::zeroed();
            BeginPaint(hwnd, &mut paint);
            EndPaint(hwnd, &paint);
            return 0;
        }
        WM_SIZE => {
            let view = ((lparam & 0xffff) as u32, ((lparam >> 16) & 0xffff) as u32);
            if view.0 > 0 && view.1 > 0 {
                if let Some(presenter) = data.presenter.borrow().clone() {
                    presenter.view_resized(view);
                }
            }
            return 0;
        }
        WM_DPICHANGED => {
            // The window manager answers with the overlay's size on this monitor; the size
            // Windows suggests is not taken.
            (data.report)(FromHost::Dpi {
                task_id: data.task_id,
                dpi: ((wparam & 0xffff) as u32).max(96),
            });
            return 0;
        }
        WM_TIMER if wparam == HOVER_TIMER => {
            follow_pointer(hwnd, data);
            return 0;
        }
        // A finger has no hover: a touch brings the handle out for a moment.
        WM_POINTERDOWN | WM_LBUTTONDOWN => data.pointed.set(Some(Instant::now())),
        WM_NCDESTROY => {
            KillTimer(hwnd, HOVER_TIMER);
            let handle = data.handle.replace(std::ptr::null_mut());
            if !handle.is_null() {
                DestroyWindow(handle);
            }
            data.ime.detach();
            super::pointer::forget_contacts(hwnd);
            let data = SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) as *mut OverlayData;
            if !data.is_null() {
                drop(Box::from_raw(data));
            }
            return DefWindowProcW(hwnd, message, wparam, lparam);
        }
        _ => {}
    }
    if let Some(result) = data.pointer.handle(hwnd, message, wparam, lparam) {
        return result;
    }
    if let Some(result) = data.keys.handle(hwnd, message, wparam, lparam) {
        return result;
    }
    if let Some(result) = data.ime.handle(hwnd, message, wparam, lparam) {
        return result;
    }
    DefWindowProcW(hwnd, message, wparam, lparam)
}

/// Ends a drag of the handle: the app's plane is where the user left the window, the other
/// overlays of the app follow it, and the place is remembered for the app.
unsafe fn end_drag(owner: HWND, data: &OverlayData) {
    if data.drag.take().is_none() {
        return;
    }
    let mut window: RECT = std::mem::zeroed();
    let Some(guest) = data.guest.get() else {
        return;
    };
    if GetWindowRect(owner, &mut window) == 0 {
        return;
    }
    let kept = APP.with(|app| {
        let mut app = app.borrow_mut();
        let anchor = app.plane.moved_to(guest, (window.left, window.top));
        app.places
            .as_deref()
            .map(|dir| remember(dir, &app.package, anchor))
    });
    if let Some(Err(why)) = kept {
        note(None, &why);
    }
    // On a monitor of another scale the window hears its new size from the window manager.
    place_all();
}

unsafe extern "system" fn handle_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_NCCREATE {
        let create = &*(lparam as *const CREATESTRUCTW);
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, create.lpCreateParams as isize);
        return DefWindowProcW(hwnd, message, wparam, lparam);
    }
    let owner = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as HWND;
    let data = if owner.is_null() {
        None
    } else {
        data_of(owner)
    };
    match message {
        // Dragging it never takes the focus from the window the user works in.
        WM_MOUSEACTIVATE => MA_NOACTIVATE as LRESULT,
        WM_ERASEBKGND => 1,
        WM_PAINT => {
            let mut paint: PAINTSTRUCT = std::mem::zeroed();
            let context = BeginPaint(hwnd, &mut paint);
            let mut client: RECT = std::mem::zeroed();
            GetClientRect(hwnd, &mut client);
            let fill = CreateSolidBrush(HANDLE_FILL);
            FillRect(context, &client, fill);
            DeleteObject(fill);
            // The grip: a light bar across the middle.
            let (width, height) = (client.right - client.left, client.bottom - client.top);
            let grip = RECT {
                left: width / 4,
                top: height / 2 - (height / 10).max(1),
                right: width - width / 4,
                bottom: height / 2 + (height / 10).max(1),
            };
            let light = CreateSolidBrush(HANDLE_GRIP);
            FillRect(context, &grip, light);
            DeleteObject(light);
            EndPaint(hwnd, &paint);
            0
        }
        WM_LBUTTONDOWN => {
            if let Some(data) = data {
                let mut pointer = POINT { x: 0, y: 0 };
                let mut window: RECT = std::mem::zeroed();
                if GetCursorPos(&mut pointer) != 0 && GetWindowRect(owner, &mut window) != 0 {
                    data.drag
                        .set(Some(((pointer.x, pointer.y), (window.left, window.top))));
                    SetCapture(hwnd);
                }
            }
            0
        }
        WM_MOUSEMOVE => {
            if let Some((data, (from, origin))) = data.and_then(|d| Some((d, d.drag.get()?))) {
                let mut pointer = POINT { x: 0, y: 0 };
                if GetCapture() == hwnd && GetCursorPos(&mut pointer) != 0 {
                    SetWindowPos(
                        owner,
                        std::ptr::null_mut(),
                        origin.0 + pointer.x - from.0,
                        origin.1 + pointer.y - from.1,
                        0,
                        0,
                        SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE,
                    );
                    place_handle(owner, data);
                    data.pointed.set(Some(Instant::now()));
                }
            }
            0
        }
        WM_LBUTTONUP => {
            if GetCapture() == hwnd {
                // WM_CAPTURECHANGED ends the drag.
                ReleaseCapture();
            }
            0
        }
        WM_CAPTURECHANGED => {
            if let Some(data) = data {
                end_drag(owner, data);
            }
            0
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui_language::assert_translation;

    #[test]
    fn fr41_the_handle_is_named_in_every_language() {
        let english = handle_name(UiLanguage::English);
        for language in UiLanguage::ALL {
            let text = handle_name(language);
            assert_translation(language, "the overlay handle's name", english, text);
            if language != UiLanguage::English {
                assert_ne!(text, english, "{language:?}");
            }
            assert_eq!(fill(text, &[("name", "FLO")]).matches("FLO").count(), 1);
        }
    }

    #[test]
    fn fr41_an_overlay_window_is_borderless_on_top_and_out_of_the_taskbar() {
        for (focusable, touchable) in [(false, true), (true, true), (false, false)] {
            let style = ex_style(focusable, touchable);
            assert_ne!(style & WS_EX_TOPMOST, 0);
            assert_ne!(style & WS_EX_TOOLWINDOW, 0);
            assert_ne!(style & WS_EX_NOREDIRECTIONBITMAP, 0);
        }
    }

    #[test]
    fn fr41_a_click_activates_only_an_overlay_that_may_take_the_focus() {
        assert_ne!(ex_style(false, true) & WS_EX_NOACTIVATE, 0);
        assert_eq!(ex_style(true, true) & WS_EX_NOACTIVATE, 0);
    }

    #[test]
    fn fr41_the_pointer_passes_through_an_overlay_that_takes_no_input() {
        let through = WS_EX_LAYERED | WS_EX_TRANSPARENT;
        assert_eq!(ex_style(false, false) & through, through);
        assert_eq!(ex_style(false, true) & through, 0);
    }

    #[test]
    fn fr41_overlay_ids_are_told_from_task_ids() {
        assert!(is_overlay(OVERLAY_ID_BASE));
        assert!(is_overlay(OVERLAY_ID_BASE + 7));
        assert!(!is_overlay(42));
        assert!(!is_overlay(1_000_042));
    }
}
