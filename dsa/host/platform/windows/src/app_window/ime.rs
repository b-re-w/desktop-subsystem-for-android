//! Text input in a task window: the system input method's composition and the characters the
//! keys type, as the platform-neutral text input the guest's editor receives.
//!
//! The window takes the input method through the text services framework ([`super::tsf`]):
//! it is a text services client with a document of its own, the text of the guest's focused
//! editor around its selection, which the agent describes ([`super::text_store`]). The input
//! method composes and commits in that document, and each edit reaches the guest as the
//! composition, the commit, the selection or the reconverted range it made. The Microsoft
//! Korean IME composes there, the touch keyboard takes its layout from the editor's input
//! scope, and dictation (Win+H), handwriting and the emoji panel type into it. The window
//! gives its document the framework's focus each time it takes the keyboard focus, rather than
//! trusting the framework to notice.
//!
//! An input method that composes outside the document all the same (the framework had another
//! document focused, or an input method of the older kind) does it through IMM32: the window
//! then gets `WM_IME_COMPOSITION`, reads the composition and what was committed from the
//! window's input context, and puts them into the document as the framework's edits would be,
//! so the guest's editor shows that composition as well. The input method's own composition
//! window is never shown, and its result never comes back a second time as `WM_CHAR`.
//!
//! The guest's editor shows the composition itself, underlined where the text is, so no
//! composition window is drawn over it. The candidate list (Hanja, say) is the input
//! method's, placed beside the cursor of the guest's editor. The guest reports it in the
//! task's pixels, and the window manager gives it in the window's client pixels: the task's
//! at the scale the window shows the task at, told again when that scale changes.
//!
//! A key that types a character without the input method (any keyboard layout, with no input
//! method composing) arrives as `WM_CHAR`, and goes as text alone: `KeyWindow` gives the guest
//! no key event for it (see [`super::keys`]). A control character (Enter, Tab, Backspace, Ctrl
//! with a letter) went as a key event, and the store drops it here.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc::Sender;
use std::sync::Arc;

use dsa_host_core::ime::{ImeCursor, ImeEditor};
use dsa_host_core::input::InputEvent;
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, POINT, RECT, WPARAM};
use windows_sys::Win32::UI::Input::Ime::{
    ImmGetCompositionStringW, ImmGetContext, ImmNotifyIME, ImmReleaseContext,
    ImmSetCandidateWindow, CANDIDATEFORM, CFS_EXCLUDE, CPS_CANCEL, CPS_COMPLETE, GCS_COMPSTR,
    GCS_CURSORPOS, GCS_RESULTSTR, HIMC, ISC_SHOWUICOMPOSITIONWINDOW, NI_COMPOSITIONSTR,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, WM_CHAR, WM_IME_COMPOSITION, WM_IME_ENDCOMPOSITION, WM_IME_SETCONTEXT,
    WM_IME_STARTCOMPOSITION, WM_KILLFOCUS, WM_SETFOCUS,
};

use super::tsf::{Document, Shared};
use crate::compositor::log::CompositorLogger;

/// What one `WM_IME_COMPOSITION` says, read from the window's input context: the text the
/// input method committed, and the composition it holds now with its cursor. A message that
/// says neither is a composition the input method dropped, which leaves none.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImmChange {
    pub result: Option<Vec<u16>>,
    pub composition: Option<(Vec<u16>, u32)>,
}

impl ImmChange {
    /// What a `WM_IME_COMPOSITION` whose `lparam` is `flags` says, through `read`, which
    /// gives the input method's string of one kind (`GCS_RESULTSTR`, `GCS_COMPSTR`), and
    /// `cursor`, which gives the cursor in the composition.
    pub fn read(
        flags: u32,
        read: impl Fn(u32) -> Option<Vec<u16>>,
        cursor: impl Fn() -> Option<u32>,
    ) -> ImmChange {
        let result = (flags & GCS_RESULTSTR != 0)
            .then(|| read(GCS_RESULTSTR))
            .flatten()
            .filter(|text| !text.is_empty());
        let composition = if flags & GCS_COMPSTR != 0 {
            read(GCS_COMPSTR).map(|text| {
                let end = text.len() as u32;
                let at = if flags & GCS_CURSORPOS != 0 {
                    cursor().unwrap_or(end).min(end)
                } else {
                    end
                };
                (text, at)
            })
        } else if result.is_none() {
            Some((Vec::new(), 0))
        } else {
            None
        };
        ImmChange {
            result,
            composition,
        }
    }
}

/// The input context of a window, released when dropped.
struct Context {
    hwnd: HWND,
    himc: HIMC,
}

impl Context {
    /// # Safety
    ///
    /// Called on the window's own thread.
    unsafe fn of(hwnd: HWND) -> Option<Context> {
        let himc = ImmGetContext(hwnd);
        (!himc.is_null()).then_some(Context { hwnd, himc })
    }

    /// The input method's string of one kind, in UTF-16 code units.
    unsafe fn string(&self, kind: u32) -> Option<Vec<u16>> {
        let bytes = ImmGetCompositionStringW(self.himc, kind, std::ptr::null_mut(), 0);
        if bytes < 0 {
            return None;
        }
        let mut units = vec![0u16; bytes as usize / 2];
        if !units.is_empty() {
            let read =
                ImmGetCompositionStringW(self.himc, kind, units.as_mut_ptr().cast(), bytes as u32);
            if read < 0 {
                return None;
            }
            units.truncate(read as usize / 2);
        }
        Some(units)
    }

    /// The cursor in the composition, in UTF-16 code units.
    unsafe fn cursor(&self) -> Option<u32> {
        let at = ImmGetCompositionStringW(self.himc, GCS_CURSORPOS, std::ptr::null_mut(), 0);
        (at >= 0).then_some(at as u32)
    }

    /// Puts the candidate list beside `cursor`, in the window's client pixels, never over it.
    unsafe fn place(&self, cursor: ImeCursor) {
        let form = CANDIDATEFORM {
            dwIndex: 0,
            dwStyle: CFS_EXCLUDE,
            ptCurrentPos: POINT {
                x: cursor.left,
                y: cursor.bottom,
            },
            rcArea: RECT {
                left: cursor.left,
                top: cursor.top,
                right: cursor.right.max(cursor.left + 1),
                bottom: cursor.bottom,
            },
        };
        ImmSetCandidateWindow(self.himc, &form);
    }

    /// Ends the input method's composition: typed as it stands, or dropped.
    unsafe fn end(&self, keep: bool) {
        let how = if keep { CPS_COMPLETE } else { CPS_CANCEL };
        ImmNotifyIME(self.himc, NI_COMPOSITIONSTR, how, 0);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // SAFETY: the context this window was given on its own thread.
        unsafe { ImmReleaseContext(self.hwnd, self.himc) };
    }
}

/// One task window's text input.
pub struct ImeWindow {
    shared: Rc<Shared>,
    /// The framework's document for the window, once the window exists; none when the window
    /// thread has no framework, and the keys then still type.
    document: RefCell<Option<Document>>,
    hwnd: Cell<isize>,
    /// Whether the input method composes through IMM32 now.
    imm_composing: Cell<bool>,
    /// Whether the log was told the input method composes through IMM32 in this window.
    imm_noted: Cell<bool>,
}

impl ImeWindow {
    pub fn new(
        task_id: i32,
        events: Sender<InputEvent>,
        logger: Option<Arc<CompositorLogger>>,
    ) -> Self {
        ImeWindow {
            shared: Shared::new(task_id, events, logger),
            document: RefCell::new(None),
            hwnd: Cell::new(0),
            imm_composing: Cell::new(false),
            imm_noted: Cell::new(false),
        }
    }

    /// Gives the window `hwnd` its document, so the input method composes in it. Called once
    /// the window exists, on its thread. The reason it could not, for the log.
    pub fn attach(&self, hwnd: HWND) -> Result<(), String> {
        self.hwnd.set(hwnd as isize);
        let document = Document::attach(hwnd as isize, &self.shared)?;
        if let Ok(mut slot) = self.document.try_borrow_mut() {
            *slot = Some(document);
        }
        // A window that has the keyboard focus already never sees it arrive.
        // SAFETY: plain Win32 on the window's own thread.
        if unsafe { windows_sys::Win32::UI::Input::KeyboardAndMouse::GetFocus() } == hwnd {
            self.focus_document();
        }
        Ok(())
    }

    /// Lets the document go, as the window closes.
    pub fn detach(&self) {
        let document = self
            .document
            .try_borrow_mut()
            .ok()
            .and_then(|mut slot| slot.take());
        drop(document);
    }

    /// Where the guest last said the cursor of the task's focused editor is.
    pub fn cursor(&self) -> Option<ImeCursor> {
        self.shared.cursor()
    }

    /// What the guest last said of the task's focused editor.
    pub fn editor(&self) -> Option<ImeEditor> {
        self.shared.described()
    }

    /// The guest says the cursor of the task's focused editor is at `cursor`: the input
    /// method's windows go beside it from now on.
    pub fn cursor_moved(&self, cursor: ImeCursor) {
        self.shared.cursor_moved(cursor);
        if self.imm_composing.get() {
            // SAFETY: on the window's own thread, which the window manager's messages reach.
            unsafe {
                if let Some(context) = Context::of(self.hwnd.get() as HWND) {
                    context.place(cursor);
                }
            }
        }
    }

    /// The guest described the task's focused editor: the document becomes its text, and the
    /// input scope follows its kind.
    pub fn editor_changed(&self, editor: ImeEditor) {
        let document = self.document.try_borrow().ok();
        let ended = match document.as_ref().and_then(|document| document.as_ref()) {
            Some(document) => document.described(editor),
            // No framework: the description is kept for what it says of the editor.
            None => {
                self.shared.described_without_framework(editor);
                false
            }
        };
        if ended && self.imm_composing.get() {
            // The guest's editor kept the composition's text where it was; the input method
            // drops its own, so the next key composes anew where the cursor is now.
            // SAFETY: on the window's own thread.
            unsafe {
                if let Some(context) = Context::of(self.hwnd.get() as HWND) {
                    context.end(false);
                }
            }
        }
    }

    fn focus_document(&self) {
        if let Ok(document) = self.document.try_borrow() {
            if let Some(document) = document.as_ref() {
                document.focus();
            }
        }
    }

    /// Ends a composition, typed as it stands, as the window loses the focus.
    fn end_composition(&self, hwnd: HWND) {
        if let Ok(document) = self.document.try_borrow() {
            if let Some(document) = document.as_ref() {
                document.end_composition();
            }
        }
        if self.imm_composing.get() {
            // SAFETY: on the window's own thread.
            unsafe {
                if let Some(context) = Context::of(hwnd) {
                    context.end(true);
                }
            }
        }
    }

    /// Handles a typed character, the input method's IMM32 messages, and the keyboard focus
    /// coming and going. `None` for any other message, or one that Windows must also see.
    ///
    /// # Safety
    ///
    /// Called on the window's own thread, from its window procedure.
    pub unsafe fn handle(
        &self,
        hwnd: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> Option<LRESULT> {
        match message {
            WM_CHAR => {
                self.shared.typed(wparam as u16);
                Some(0)
            }
            WM_SETFOCUS => {
                self.focus_document();
                None
            }
            WM_KILLFOCUS => {
                // What is being composed is typed as it stands, as in any other window,
                // rather than left in the guest's editor half made.
                self.end_composition(hwnd);
                None
            }
            // The guest's editor shows the composition, so the input method draws none of
            // its own; its candidate list still shows.
            WM_IME_SETCONTEXT => Some(DefWindowProcW(
                hwnd,
                message,
                wparam,
                lparam & !(ISC_SHOWUICOMPOSITIONWINDOW as LPARAM),
            )),
            // A composition of the framework's own goes through the document; these come
            // only for one outside it. Once the framework has composed in the document, any
            // that still come are its echo, which Windows keeps.
            WM_IME_STARTCOMPOSITION | WM_IME_COMPOSITION | WM_IME_ENDCOMPOSITION
                if self.shared.composes_in_document() =>
            {
                None
            }
            WM_IME_STARTCOMPOSITION => {
                if !self.imm_noted.replace(true) {
                    self.shared.note(
                        "the input method composes through IMM32, outside the window's text \
                         services document; the guest's editor takes that composition",
                    );
                }
                self.imm_composing.set(true);
                if let (Some(context), Some(cursor)) = (Context::of(hwnd), self.shared.cursor()) {
                    context.place(cursor);
                }
                Some(0)
            }
            WM_IME_COMPOSITION => {
                let context = Context::of(hwnd)?;
                let change = ImmChange::read(
                    lparam as u32,
                    |kind| context.string(kind),
                    || context.cursor(),
                );
                drop(context);
                self.imm_composing.set(true);
                self.shared.imm(
                    change.result.as_deref(),
                    change
                        .composition
                        .as_ref()
                        .map(|(text, cursor)| (text.as_slice(), *cursor)),
                );
                // Handled: Windows would type the result again as WM_IME_CHAR and WM_CHAR.
                Some(0)
            }
            WM_IME_ENDCOMPOSITION if self.imm_composing.replace(false) => {
                // A composition the input method ended without a word removes what it showed.
                if self.shared.imm_composing() {
                    let nothing: &[u16] = &[];
                    self.shared.imm(None, Some((nothing, 0)));
                }
                Some(0)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn units(text: &str) -> Vec<u16> {
        text.encode_utf16().collect()
    }

    #[test]
    fn fr6_an_imm32_composition_message_gives_the_commit_before_the_next_syllable() {
        // Korean commits 한 and begins ㄱ in one message.
        let change = ImmChange::read(
            GCS_RESULTSTR | GCS_COMPSTR | GCS_CURSORPOS,
            |kind| match kind {
                GCS_RESULTSTR => Some(units("\u{d55c}")),
                GCS_COMPSTR => Some(units("\u{3131}")),
                _ => None,
            },
            || Some(1),
        );
        assert_eq!(change.result, Some(units("\u{d55c}")));
        assert_eq!(change.composition, Some((units("\u{3131}"), 1)));
        // A composition alone, its cursor at its end when the message gives none.
        let change = ImmChange::read(GCS_COMPSTR, |_| Some(units("\u{d558}")), || None);
        assert_eq!(change.result, None);
        assert_eq!(change.composition, Some((units("\u{d558}"), 1)));
    }

    #[test]
    fn fr6_an_imm32_composition_message_that_says_nothing_drops_the_composition() {
        let change = ImmChange::read(0, |_| None, || None);
        assert_eq!(change.result, None);
        assert_eq!(change.composition, Some((Vec::new(), 0)));
        // A commit alone leaves no composition to drop.
        let change = ImmChange::read(GCS_RESULTSTR, |_| Some(units("a")), || None);
        assert_eq!(change.composition, None);
    }
}
