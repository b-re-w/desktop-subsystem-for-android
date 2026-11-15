//! What an app window host asks of the window manager's GPU, and how the answers come back.
//!
//! An app window host owns its task windows and their DirectComposition visuals, but no
//! Direct3D device and no DXGI factory: a device costs each app process tens of MiB of driver
//! memory. The window manager, which has every task's frame texture on its own device, owns
//! each window's swapchains, binds each to a DirectComposition surface handle, and places the
//! handle in the window host's process. The window host shows the surface from that handle
//! and sends a `SurfaceCall` for each thing to do on the GPU (copy and present a frame, draw
//! the caption, read a frame back); the window manager answers with one `SurfaceAnswer`, or
//! several for a frame read back, which is cut into parts that each fit in one frame of the
//! connection.
//!
//! This module holds the parts both ends share and that name no OS API: the operations, the
//! wire form of a band sample, how a frame read back is cut and put together again, and the
//! calls a window host waits on ([`PendingCalls`]).

use std::collections::HashMap;
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

use dsa_protocol::window_host::types::SurfaceAnswer;

use super::caption::BandSample;

/// Copy a frame from its texture into the frame swapchain, and present it unless kept.
pub const SURFACE_OP_PRESENT_FRAME: u32 = 1;
/// Present the frame kept in the back buffer.
pub const SURFACE_OP_PRESENT_KEPT: u32 = 2;
/// Draw the caption buttons.
pub const SURFACE_OP_CAPTION: u32 = 3;
/// Draw the splash.
pub const SURFACE_OP_SPLASH: u32 = 4;
/// Fill the one-pixel fill surface with a colour.
pub const SURFACE_OP_FILL: u32 = 5;
/// Let the frame swapchain go: the window was minimized.
pub const SURFACE_OP_RELEASE_FRAME: u32 = 6;
/// Let every swapchain of the window go: the window closed.
pub const SURFACE_OP_RELEASE_ALL: u32 = 7;
/// Read the newest frame back.
pub const SURFACE_OP_READ_FRAME: u32 = 8;
/// Read what waits to be read and recover from a GPU reset.
pub const SURFACE_OP_TICK: u32 = 9;
/// Let the splash swapchain go: the first frame is shown.
pub const SURFACE_OP_RELEASE_SPLASH: u32 = 10;

/// Whether `op` is one the window manager carries out. Anything else is answered with an
/// error and changes nothing, so a window host of another build cannot make the window
/// manager act on what it does not know.
pub fn known_op(op: u32) -> bool {
    (SURFACE_OP_PRESENT_FRAME..=SURFACE_OP_RELEASE_SPLASH).contains(&op)
}

/// The largest part of a frame read back that one answer carries, in bytes: well under the
/// connection's largest frame, with room for the rest of the answer.
pub const READ_PART_BYTES: usize = 8 * 1024 * 1024;

/// Writes `sample` into `answer`, as a band that changed.
pub fn band_to_answer(sample: BandSample, answer: &mut SurfaceAnswer) {
    answer.band_changed = true;
    answer.has_band_fill = sample.fill.is_some();
    answer.band_fill = sample.fill.unwrap_or(0);
    answer.has_band_colour = sample.band.is_some();
    answer.band_colour = sample.band.unwrap_or(0);
    answer.has_band_luminance = sample.luminance.is_some();
    answer.band_luminance = sample.luminance.map_or(0, u32::from);
    answer.has_band_range = sample.range.is_some();
    let (darkest, lightest) = sample.range.unwrap_or((0, 0));
    answer.band_darkest = u32::from(darkest);
    answer.band_lightest = u32::from(lightest);
}

/// The band sample `answer` carries, if the band changed.
pub fn band_from_answer(answer: &SurfaceAnswer) -> Option<BandSample> {
    if !answer.band_changed {
        return None;
    }
    let thousandths = |value: u32| value.min(u32::from(u16::MAX)) as u16;
    Some(BandSample {
        fill: answer.has_band_fill.then_some(answer.band_fill),
        band: answer.has_band_colour.then_some(answer.band_colour),
        luminance: answer
            .has_band_luminance
            .then(|| thousandths(answer.band_luminance)),
        range: answer.has_band_range.then(|| {
            (
                thousandths(answer.band_darkest),
                thousandths(answer.band_lightest),
            )
        }),
    })
}

/// How a `width` x `height` RGBA frame is cut into parts of whole rows of at most
/// `part_bytes` bytes each: the byte ranges, in order. One part, possibly empty, at least, so
/// every read is answered.
pub fn read_parts(width: u32, height: u32, part_bytes: usize) -> Vec<std::ops::Range<usize>> {
    let row = width as usize * 4;
    let total = row * height as usize;
    if row == 0 || total == 0 {
        return std::iter::once(0..0).collect();
    }
    let rows_per_part = (part_bytes / row).max(1);
    let step = rows_per_part * row;
    (0..total)
        .step_by(step)
        .map(|start| start..(start + step).min(total))
        .collect()
}

/// A frame read back, put together from its answers: `width` x `height` pixels as RGBA rows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadFrame {
    pub width: u32,
    pub height: u32,
    pub rgba: Vec<u8>,
}

/// Puts a frame read back together from the answers of one call. `Ok(None)` when the window
/// manager had no frame to read; an error when an answer says one, or the parts do not add up
/// to the frame's size.
pub fn assemble_read(answers: &[SurfaceAnswer]) -> Result<Option<ReadFrame>, String> {
    if let Some(failed) = answers.iter().find(|answer| !answer.error.is_empty()) {
        return Err(failed.error.clone());
    }
    let Some(first) = answers.first() else {
        return Err("the window manager did not answer the read".into());
    };
    if !first.has_frame {
        return Ok(None);
    }
    let (width, height) = (first.frame_width, first.frame_height);
    let expected = width as usize * height as usize * 4;
    let mut rgba = Vec::with_capacity(expected);
    for answer in answers {
        rgba.extend_from_slice(&answer.rgba);
    }
    if rgba.len() != expected {
        return Err(format!(
            "the frame read back has {} bytes, not the {expected} of {width}x{height} pixels",
            rgba.len()
        ));
    }
    Ok(Some(ReadFrame {
        width,
        height,
        rgba,
    }))
}

/// Why a call got no answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallError {
    /// The connection to the window manager is gone.
    Closed,
    /// No answer came in time.
    TimedOut,
}

impl std::fmt::Display for CallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CallError::Closed => write!(f, "the connection to the window manager is gone"),
            CallError::TimedOut => write!(f, "the window manager did not answer in time"),
        }
    }
}

#[derive(Default)]
struct Calls {
    next: u64,
    /// The answers that came for each call still waited on, and whether the last part came.
    waiting: HashMap<u64, (Vec<SurfaceAnswer>, bool)>,
    closed: bool,
}

/// The calls a window host has sent and waits on. Any thread may start a call and wait for
/// it; the thread that reads the connection hands each answer in ([`PendingCalls::deliver`]).
/// When the connection ends, every wait ends with [`CallError::Closed`], so no window waits
/// for a window manager that is gone.
#[derive(Default)]
pub struct PendingCalls {
    calls: Mutex<Calls>,
    answered: Condvar,
}

impl PendingCalls {
    pub fn new() -> PendingCalls {
        PendingCalls::default()
    }

    /// Starts a call: its ID, to send with it and wait on. An error once closed.
    pub fn begin(&self) -> Result<u64, CallError> {
        let mut calls = self.calls.lock().unwrap_or_else(|e| e.into_inner());
        if calls.closed {
            return Err(CallError::Closed);
        }
        calls.next += 1;
        let id = calls.next;
        calls.waiting.insert(id, (Vec::new(), false));
        Ok(id)
    }

    /// The call `id` could not be sent: nothing waits for it.
    pub fn abandon(&self, id: u64) {
        let mut calls = self.calls.lock().unwrap_or_else(|e| e.into_inner());
        calls.waiting.remove(&id);
    }

    /// An answer came. Returns whether a call waited for it; one for no call (late, after its
    /// wait timed out, or never asked) is dropped.
    pub fn deliver(&self, answer: SurfaceAnswer) -> bool {
        let mut calls = self.calls.lock().unwrap_or_else(|e| e.into_inner());
        let Some((answers, done)) = calls.waiting.get_mut(&answer.call_id) else {
            return false;
        };
        if *done {
            return false;
        }
        *done = !answer.more;
        answers.push(answer);
        drop(calls);
        self.answered.notify_all();
        true
    }

    /// Waits at most `timeout` for every part of call `id`'s answer.
    pub fn wait(&self, id: u64, timeout: Duration) -> Result<Vec<SurfaceAnswer>, CallError> {
        let deadline = Instant::now() + timeout;
        let mut calls = self.calls.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if calls.waiting.get(&id).is_some_and(|(_, done)| *done) {
                let (answers, _) = calls.waiting.remove(&id).unwrap_or_default();
                return Ok(answers);
            }
            if calls.closed {
                calls.waiting.remove(&id);
                return Err(CallError::Closed);
            }
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                calls.waiting.remove(&id);
                return Err(CallError::TimedOut);
            }
            calls = self
                .answered
                .wait_timeout(calls, left)
                .unwrap_or_else(|e| e.into_inner())
                .0;
        }
    }

    /// The connection ended: every wait ends, and no call starts again.
    pub fn close(&self) {
        let mut calls = self.calls.lock().unwrap_or_else(|e| e.into_inner());
        calls.closed = true;
        drop(calls);
        self.answered.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn answer(call_id: u64, more: bool) -> SurfaceAnswer {
        SurfaceAnswer {
            call_id,
            more,
            ..Default::default()
        }
    }

    #[test]
    fn nfr2_band_sample_crosses_the_window_manager_connection_unchanged() {
        let samples = [
            BandSample::default(),
            BandSample {
                fill: Some(0xFF20_3040),
                band: None,
                luminance: Some(812),
                range: Some((0, 1000)),
            },
            BandSample {
                fill: None,
                band: Some(0xFF00_0000),
                luminance: None,
                range: None,
            },
        ];
        for sample in samples {
            let mut wire = SurfaceAnswer::default();
            band_to_answer(sample, &mut wire);
            assert_eq!(band_from_answer(&wire), Some(sample));
        }
        assert_eq!(band_from_answer(&SurfaceAnswer::default()), None);
    }

    #[test]
    fn nfr2_a_frame_read_back_is_cut_into_parts_that_fit_the_connection() {
        // A 4K frame is larger than one frame of the connection.
        let parts = read_parts(3840, 2160, READ_PART_BYTES);
        assert!(parts.len() > 1);
        assert_eq!(parts.first().map(|part| part.start), Some(0));
        assert_eq!(parts.last().map(|part| part.end), Some(3840 * 2160 * 4));
        for pair in parts.windows(2) {
            assert_eq!(pair[0].end, pair[1].start);
        }
        for part in &parts {
            assert!(part.len() <= READ_PART_BYTES);
            assert_eq!(part.len() % (3840 * 4), 0, "parts hold whole rows");
        }
        // An empty frame is still answered once.
        assert_eq!(read_parts(0, 0, READ_PART_BYTES), vec![0..0]);
    }

    #[test]
    fn nfr2_a_frame_read_back_is_put_together_from_its_parts() {
        let (width, height) = (3u32, 5u32);
        let rgba: Vec<u8> = (0..width * height * 4).map(|b| b as u8).collect();
        let answers: Vec<SurfaceAnswer> = read_parts(width, height, 24)
            .into_iter()
            .enumerate()
            .map(|(index, range)| SurfaceAnswer {
                call_id: 7,
                has_frame: true,
                frame_width: width,
                frame_height: height,
                more: range.end < rgba.len(),
                rgba: rgba[range].to_vec(),
                ..answer(7, index == 0)
            })
            .collect();
        let frame = assemble_read(&answers).unwrap().unwrap();
        assert_eq!((frame.width, frame.height), (width, height));
        assert_eq!(frame.rgba, rgba);
        // Parts lost on the way are an error, not a torn screenshot.
        assert!(assemble_read(&answers[..1]).is_err());
        // No frame yet is no screenshot, not an error.
        assert_eq!(assemble_read(&[answer(7, false)]), Ok(None));
    }

    #[test]
    fn nfr5_a_window_waiting_on_the_window_manager_is_freed_when_it_goes() {
        let calls = Arc::new(PendingCalls::new());
        let id = calls.begin().unwrap();
        let waiter = {
            let calls = Arc::clone(&calls);
            std::thread::spawn(move || calls.wait(id, Duration::from_secs(30)))
        };
        std::thread::sleep(Duration::from_millis(20));
        calls.close();
        assert_eq!(waiter.join().unwrap(), Err(CallError::Closed));
        // And nothing waits on it again.
        assert_eq!(calls.begin(), Err(CallError::Closed));
    }

    #[test]
    fn nfr5_an_answer_to_no_call_is_dropped() {
        let calls = PendingCalls::new();
        assert!(!calls.deliver(answer(99, false)));
        let id = calls.begin().unwrap();
        assert_eq!(
            calls.wait(id, Duration::from_millis(1)),
            Err(CallError::TimedOut)
        );
        // Late, after its wait gave up.
        assert!(!calls.deliver(answer(id, false)));
    }

    #[test]
    fn nfr2_every_part_of_an_answer_reaches_its_call() {
        let calls = PendingCalls::new();
        let id = calls.begin().unwrap();
        assert!(calls.deliver(answer(id, true)));
        assert!(calls.deliver(answer(id, false)));
        // A part after the last one belongs to nothing.
        assert!(!calls.deliver(answer(id, false)));
        assert_eq!(calls.wait(id, Duration::from_secs(1)).unwrap().len(), 2);
    }

    #[test]
    fn nfr5_an_unknown_surface_operation_is_refused() {
        for op in SURFACE_OP_PRESENT_FRAME..=SURFACE_OP_RELEASE_SPLASH {
            assert!(known_op(op));
        }
        assert!(!known_op(0));
        assert!(!known_op(SURFACE_OP_RELEASE_SPLASH + 1));
    }
}
