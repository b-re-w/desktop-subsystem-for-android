//! Moving and resizing a widget window with the keyboard, and the keys that open its menu.
//!
//! The widget's menu offers Move and Size, as a window's system menu does. Choosing one starts
//! a [`KeyboardEdit`]: the arrow keys move the widget, or resize it from its bottom right
//! corner, a step at a time; Enter finishes and the placement is kept, as the end of a drag
//! keeps it; Esc puts the widget back where it was. A user who does not use a pointer can so
//! place a widget as precisely as one who drags it.
//!
//! A step of a move is [`MOVE_STEP_DIP`] device-independent pixels, or one with Ctrl held
//! ([`FINE_STEP_DIP`]), and a moved widget stays on its monitor's work area. A step of a resize
//! is a cell of [`SIZE_CELL_DP`] of Android's dp, or one dp with Ctrl held, and the widget
//! keeps to the sizes it allows ([`clamp_size`]) along the sides it may be resized along,
//! and to its work area.
//!
//! The menu opens from the keyboard with the menu key, Shift+F10, Enter or Space while the
//! window has the keyboard ([`opens_menu`]).

use super::geometry::{dp_of_px, px_of_dip, px_of_dp, ScreenRect};
use crate::widgets::{clamp_size, WidgetProvider, RESIZE_HORIZONTAL, RESIZE_VERTICAL};

/// How far one arrow key moves a widget, in device-independent pixels.
pub const MOVE_STEP_DIP: u32 = 8;

/// How far one arrow key moves a widget with Ctrl held, in device-independent pixels.
pub const FINE_STEP_DIP: u32 = 1;

/// How much one arrow key resizes a widget, in Android's dp: a cell of a grid of 16 dp, so a
/// few presses take a widget from one launcher size to the next.
pub const SIZE_CELL_DP: u32 = 16;

/// How much one arrow key resizes a widget with Ctrl held, in Android's dp.
pub const FINE_SIZE_DP: u32 = 1;

/// What a keyboard edit does with the arrow keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardMode {
    /// The arrows move the widget.
    Move,
    /// The arrows resize the widget from its bottom right corner.
    Size,
}

/// A key a widget window answers, whichever host it came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Enter,
    Space,
    Escape,
    /// The menu key (the application key of a PC keyboard).
    Menu,
    F10,
    Other,
}

/// Whether `key`, with Shift held when `shift`, opens the widget's menu under it: the menu key,
/// Shift+F10, Enter or Space, as a focused control's context menu opens.
pub fn opens_menu(key: Key, shift: bool) -> bool {
    match key {
        Key::Menu | Key::Enter | Key::Space => true,
        Key::F10 => shift,
        _ => false,
    }
}

/// What a key did to a keyboard edit, for the host to carry out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyStep {
    /// The key does nothing here.
    Nothing,
    /// Show the widget at this rectangle on the screen now. Nothing is kept yet: a move is
    /// kept at Enter, and a resize passes its size on as a drag of an edge does when it pauses.
    Show(ScreenRect),
    /// Enter: the edit is over and the widget stays at this rectangle, which is kept as the
    /// end of a drag keeps it.
    Finish(ScreenRect),
    /// Esc: the edit is over and the widget goes back to this rectangle, where it was when the
    /// edit began. `resized` says its size was changed meanwhile, so its size is passed on
    /// again and Android draws it at its old size.
    Restore { to: ScreenRect, resized: bool },
}

/// A widget being moved or resized with the keyboard.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardEdit {
    pub mode: KeyboardMode,
    /// The widget's rectangle on the screen when the edit began.
    start: ScreenRect,
    /// Where it is now.
    now: ScreenRect,
    /// Its size now in dp, which a resize steps through, so that rounding to whole pixels
    /// never keeps a small step from changing it.
    size_dp: (u32, u32),
    dpi: u32,
    resize_mode: u32,
    provider: WidgetProvider,
    /// The work area of the widget's monitor, which it stays on.
    area: Option<ScreenRect>,
    /// Whether the size was ever changed.
    resized: bool,
}

impl KeyboardEdit {
    /// An edit of the widget of `provider`, which is at `widget` on the screen (its own
    /// rectangle, without the room of an edit), on a monitor of `dpi` whose work area is
    /// `area`.
    pub fn new(
        mode: KeyboardMode,
        widget: ScreenRect,
        provider: &WidgetProvider,
        dpi: u32,
        area: Option<ScreenRect>,
    ) -> KeyboardEdit {
        let (l, t, r, b) = widget;
        KeyboardEdit {
            mode,
            start: widget,
            now: widget,
            size_dp: (dp_of_px(r - l, dpi).max(1), dp_of_px(b - t, dpi).max(1)),
            dpi: dpi.max(1),
            resize_mode: provider.resize_mode,
            provider: provider.clone(),
            area,
            resized: false,
        }
    }

    /// The widget's rectangle on the screen now.
    pub fn rect(&self) -> ScreenRect {
        self.now
    }

    /// Where the widget was when the edit began.
    pub fn start(&self) -> ScreenRect {
        self.start
    }

    /// Whether the widget can be resized at all, which the menu's Size needs.
    pub fn can_size(provider: &WidgetProvider) -> bool {
        provider.resize_mode & (RESIZE_HORIZONTAL | RESIZE_VERTICAL) != 0
    }

    /// Takes a key; `fine` when Ctrl is held, for a step of one pixel or one dp.
    pub fn key(&mut self, key: Key, fine: bool) -> KeyStep {
        match key {
            Key::Enter => KeyStep::Finish(self.now),
            Key::Escape => {
                self.now = self.start;
                KeyStep::Restore {
                    to: self.start,
                    resized: self.resized,
                }
            }
            Key::Left | Key::Right | Key::Up | Key::Down => {
                let next = match self.mode {
                    KeyboardMode::Move => self.moved(key, fine),
                    KeyboardMode::Size => self.sized(key, fine),
                };
                match next {
                    Some(rect) if rect != self.now => {
                        self.now = rect;
                        KeyStep::Show(rect)
                    }
                    _ => KeyStep::Nothing,
                }
            }
            _ => KeyStep::Nothing,
        }
    }

    /// The rectangle one step of `key` moves the widget to, kept on the work area.
    fn moved(&self, key: Key, fine: bool) -> Option<ScreenRect> {
        let step = px_of_dip(if fine { FINE_STEP_DIP } else { MOVE_STEP_DIP }, self.dpi).max(1);
        let (dx, dy) = match key {
            Key::Left => (-step, 0),
            Key::Right => (step, 0),
            Key::Up => (0, -step),
            Key::Down => (0, step),
            _ => return None,
        };
        let (l, t, r, b) = self.now;
        let (w, h) = (r - l, b - t);
        let (mut x, mut y) = (l + dx, t + dy);
        if let Some((al, at, ar, ab)) = self.area {
            // A widget larger than its work area keeps its top left corner on it.
            x = x.clamp(al, (ar - w).max(al));
            y = y.clamp(at, (ab - h).max(at));
        }
        Some((x, y, x + w, y + h))
    }

    /// The rectangle one step of `key` resizes the widget to, from its bottom right corner:
    /// Right and Down make it larger, Left and Up smaller, within what the widget allows along
    /// the side it may be resized along, and within the work area.
    fn sized(&mut self, key: Key, fine: bool) -> Option<ScreenRect> {
        let step = if fine { FINE_SIZE_DP } else { SIZE_CELL_DP };
        let horizontal = self.resize_mode & RESIZE_HORIZONTAL != 0;
        let vertical = self.resize_mode & RESIZE_VERTICAL != 0;
        let (w_dp, h_dp) = self.size_dp;
        let asked = match key {
            Key::Right if horizontal => (w_dp.saturating_add(step), h_dp),
            Key::Left if horizontal => (w_dp.saturating_sub(step), h_dp),
            Key::Down if vertical => (w_dp, h_dp.saturating_add(step)),
            Key::Up if vertical => (w_dp, h_dp.saturating_sub(step)),
            _ => return None,
        };
        let (cw, ch) = clamp_size(&self.provider, asked.0, asked.1);
        // Only the side the key names changes; the other keeps the size it has.
        let size = match key {
            Key::Left | Key::Right => (cw, h_dp),
            _ => (w_dp, ch),
        };
        if size == self.size_dp {
            return None;
        }
        let (l, t, r, b) = self.now;
        let width = if size.0 == w_dp {
            r - l
        } else {
            px_of_dp(size.0, self.dpi).max(1)
        };
        let height = if size.1 == h_dp {
            b - t
        } else {
            px_of_dp(size.1, self.dpi).max(1)
        };
        if let Some((_, _, ar, ab)) = self.area {
            // A widget does not grow past its work area.
            if (width > r - l && l + width > ar) || (height > b - t && t + height > ab) {
                return None;
            }
        }
        self.size_dp = size;
        self.resized = true;
        Some((l, t, l + width, t + height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clock() -> WidgetProvider {
        WidgetProvider {
            provider: "com.example.clock/.ClockWidget".into(),
            label: "Clock".into(),
            min_width_dp: 110,
            min_height_dp: 40,
            default_width_dp: 180,
            default_height_dp: 110,
            max_width_dp: 300,
            max_height_dp: 0,
            resize_mode: 3,
            ..WidgetProvider::default()
        }
    }

    /// The clock at 100, 200 at its default size, on a monitor of `dpi`.
    fn at_default(dpi: u32) -> ScreenRect {
        let (w, h) = (px_of_dp(180, dpi), px_of_dp(110, dpi));
        (100, 200, 100 + w, 200 + h)
    }

    const AREA: ScreenRect = (0, 0, 1920, 1040);

    #[test]
    fn fr18_widget_keyboard_move_steps_and_clamps_to_the_work_area() {
        for dpi in [96, 144, 192] {
            let start = at_default(dpi);
            let mut edit = KeyboardEdit::new(KeyboardMode::Move, start, &clock(), dpi, Some(AREA));
            let step = px_of_dip(MOVE_STEP_DIP, dpi);
            let (w, h) = (start.2 - start.0, start.3 - start.1);
            assert_eq!(
                edit.key(Key::Right, false),
                KeyStep::Show((100 + step, 200, 100 + step + w, 200 + h))
            );
            assert_eq!(
                edit.key(Key::Down, false),
                KeyStep::Show((100 + step, 200 + step, 100 + step + w, 200 + step + h))
            );
            // Ctrl moves one pixel of the monitor's scale.
            let fine = px_of_dip(FINE_STEP_DIP, dpi);
            assert_eq!(
                edit.key(Key::Left, true),
                KeyStep::Show((
                    100 + step - fine,
                    200 + step,
                    100 + step - fine + w,
                    200 + step + h
                ))
            );
            // The size never changes in a move.
            let (l, t, r, b) = edit.rect();
            assert_eq!((r - l, b - t), (w, h));
            // Held against the left and top edges of the work area, it stops there.
            for _ in 0..200 {
                edit.key(Key::Left, false);
                edit.key(Key::Up, false);
            }
            assert_eq!(edit.rect(), (0, 0, w, h));
            assert_eq!(edit.key(Key::Left, false), KeyStep::Nothing);
            // And at the right and bottom edges.
            for _ in 0..1000 {
                edit.key(Key::Right, false);
                edit.key(Key::Down, false);
            }
            assert_eq!(edit.rect(), (1920 - w, 1040 - h, 1920, 1040));
            // Other keys do nothing.
            assert_eq!(edit.key(Key::Other, false), KeyStep::Nothing);
            assert_eq!(edit.key(Key::Space, false), KeyStep::Nothing);
        }
    }

    #[test]
    fn fr18_widget_keyboard_size_respects_min_and_cells() {
        let dpi = 96;
        let start = at_default(dpi);
        let mut edit = KeyboardEdit::new(KeyboardMode::Size, start, &clock(), dpi, Some(AREA));
        // One press is one cell of 16 dp, from the bottom right corner: the top left stays.
        match edit.key(Key::Right, false) {
            KeyStep::Show((l, t, r, b)) => {
                assert_eq!((l, t), (100, 200));
                assert_eq!(r - l, px_of_dp(180 + SIZE_CELL_DP, dpi));
                assert_eq!(b - t, start.3 - start.1);
            }
            other => panic!("expected a new size, got {other:?}"),
        }
        match edit.key(Key::Down, true) {
            KeyStep::Show((_, t, _, b)) => assert_eq!(b - t, px_of_dp(110 + 1, dpi)),
            other => panic!("expected a new size, got {other:?}"),
        }
        // Smaller, until the widget's minimum, where it stops.
        for _ in 0..40 {
            edit.key(Key::Left, false);
            edit.key(Key::Up, false);
        }
        let (l, t, r, b) = edit.rect();
        assert_eq!(r - l, px_of_dp(110, dpi));
        assert_eq!(b - t, px_of_dp(crate::widgets::MIN_SIDE_DP, dpi));
        assert_eq!(edit.key(Key::Left, false), KeyStep::Nothing);
        // Larger, until the widget's maximum width.
        for _ in 0..40 {
            edit.key(Key::Right, false);
        }
        let (l, _, r, _) = edit.rect();
        assert_eq!(r - l, px_of_dp(300, dpi));
        // A widget resized only along its width ignores Up and Down.
        let wide_only = WidgetProvider {
            resize_mode: RESIZE_HORIZONTAL,
            ..clock()
        };
        let mut edit = KeyboardEdit::new(KeyboardMode::Size, start, &wide_only, dpi, Some(AREA));
        assert_eq!(edit.key(Key::Down, false), KeyStep::Nothing);
        assert!(matches!(edit.key(Key::Right, false), KeyStep::Show(_)));
        // A widget that cannot be resized offers no Size.
        let fixed = WidgetProvider {
            resize_mode: 0,
            ..clock()
        };
        assert!(!KeyboardEdit::can_size(&fixed));
        assert!(KeyboardEdit::can_size(&clock()));
        // It never grows past its work area.
        let small_area = (0, 0, start.2 + 4, 1040);
        let mut edit =
            KeyboardEdit::new(KeyboardMode::Size, start, &clock(), dpi, Some(small_area));
        assert_eq!(edit.key(Key::Right, false), KeyStep::Nothing);
    }

    #[test]
    fn fr18_escape_restores_the_placement() {
        let start = at_default(144);
        let mut edit = KeyboardEdit::new(KeyboardMode::Move, start, &clock(), 144, Some(AREA));
        edit.key(Key::Right, false);
        edit.key(Key::Down, false);
        assert_ne!(edit.rect(), start);
        assert_eq!(
            edit.key(Key::Escape, false),
            KeyStep::Restore {
                to: start,
                resized: false
            }
        );
        assert_eq!(edit.rect(), start);
        // After a resize, Esc says so, so Android is told the old size again.
        let mut edit = KeyboardEdit::new(KeyboardMode::Size, start, &clock(), 144, Some(AREA));
        edit.key(Key::Right, false);
        assert_eq!(
            edit.key(Key::Escape, false),
            KeyStep::Restore {
                to: start,
                resized: true
            }
        );
    }

    #[test]
    fn fr18_enter_saves_the_placement() {
        let start = at_default(96);
        let mut edit = KeyboardEdit::new(KeyboardMode::Move, start, &clock(), 96, Some(AREA));
        edit.key(Key::Left, false);
        let moved = edit.rect();
        assert_eq!(edit.key(Key::Enter, false), KeyStep::Finish(moved));
        assert_eq!(edit.start(), start);
    }

    #[test]
    fn fr18_shift_f10_and_menu_key_open_the_widget_menu() {
        assert!(opens_menu(Key::Menu, false));
        assert!(opens_menu(Key::F10, true));
        assert!(!opens_menu(Key::F10, false));
        assert!(opens_menu(Key::Enter, false));
        assert!(opens_menu(Key::Space, false));
        for key in [
            Key::Left,
            Key::Right,
            Key::Up,
            Key::Down,
            Key::Escape,
            Key::Other,
        ] {
            assert!(!opens_menu(key, true), "{key:?}");
        }
    }
}
