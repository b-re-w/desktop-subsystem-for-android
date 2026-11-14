//! What the user is told about the Vulkan the graphics renderer runs on.
//!
//! Two cases need the user: no Vulkan at all (no GPU has a Vulkan driver and the Compatibility
//! Pack is not installed), where Android cannot draw and is kept stopped until they install
//! the pack; and Dozen on the software Direct3D 12 device, where everything works but the CPU
//! draws, so games are slow. The first is said each time the window manager finds it, since
//! nothing works until it is fixed. The second is said once: a file in the user's DSA folder
//! remembers it, so the user is not told again at every sign-in, and is told again only after
//! the renderer ran on a GPU in between.
//!
//! The notices are DSA's own toasts under the recovery prefix, so a click on one asks nothing
//! of Android.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use dsa_host_core::compositor::frame_stall::NOTICE_PREFIX;
use dsa_host_core::graphics_state::GraphicsState;
use dsa_host_core::notifications::Notification;

use crate::vulkan_driver::{PACK_WEB_LINK, PACK_WINGET_ID};

/// The keys of the two notices.
pub fn unavailable_key() -> String {
    format!("{NOTICE_PREFIX}vulkan:unavailable")
}

pub fn cpu_key() -> String {
    format!("{NOTICE_PREFIX}vulkan:cpu")
}

/// The title and text of the notice for `state`, in Korean when `korean`; `None` for a state
/// the user need not hear of.
pub fn notice_text(state: &GraphicsState, korean: bool) -> Option<(String, String)> {
    match (state, korean) {
        (GraphicsState::Unavailable(_), false) => Some((
            "Android needs Vulkan to draw".into(),
            format!(
                "No GPU of this PC has a Vulkan driver DSA can use, so Android is not \
                 started. Install Microsoft's \"OpenCL, OpenGL, and Vulkan Compatibility \
                 Pack\" from the Microsoft Store ({PACK_WEB_LINK}) or with \"winget install \
                 {PACK_WINGET_ID}\", then sign out and in again. \"dsa gpus\" shows what DSA \
                 finds."
            ),
        )),
        (GraphicsState::Unavailable(_), true) => Some((
            "Android를 그리려면 Vulkan이 필요합니다".into(),
            format!(
                "이 PC의 GPU에는 DSA가 쓸 수 있는 Vulkan 드라이버가 없어 Android를 시작하지 \
                 않습니다. Microsoft Store({PACK_WEB_LINK})나 \"winget install \
                 {PACK_WINGET_ID}\"로 Microsoft의 \"OpenCL, OpenGL, and Vulkan \
                 Compatibility Pack\"을 설치한 뒤 로그아웃했다가 다시 로그인하십시오. \
                 \"dsa gpus\"로 DSA가 찾은 것을 볼 수 있습니다."
            ),
        )),
        (GraphicsState::LayeredOnCpu(_), false) => Some((
            "Android draws on the CPU".into(),
            "This PC's GPU has no Direct3D 12 driver, so DSA renders Android through Dozen on \
             Windows' software renderer. Apps work, but games and animations will be slow."
                .into(),
        )),
        (GraphicsState::LayeredOnCpu(_), true) => Some((
            "Android를 CPU로 그립니다".into(),
            "이 PC의 GPU에는 Direct3D 12 드라이버가 없어 DSA가 Windows의 소프트웨어 렌더러 위의 \
             Dozen으로 Android를 그립니다. 앱은 동작하지만 게임과 애니메이션은 느립니다."
                .into(),
        )),
        _ => None,
    }
}

/// The notice for `state` as one of DSA's own notifications.
pub fn notification(state: &GraphicsState, korean: bool) -> Option<Notification> {
    let (title, text) = notice_text(state, korean)?;
    let key = match state {
        GraphicsState::Unavailable(_) => unavailable_key(),
        _ => cpu_key(),
    };
    Some(Notification {
        key,
        package_name: String::new(),
        app_label: dsa_host_core::update::alerts::SHOWN_AS.into(),
        title,
        text,
        icon_png: Vec::new(),
        importance: 4,
        high_priority: false,
        clearable: true,
        when_millis: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|since| since.as_millis() as i64)
            .unwrap_or(0),
    })
}

/// Decides which renderer starts the user hears of.
#[derive(Debug, Default)]
pub struct GraphicsNotices {
    /// The state this process last told the user of, as its wire name.
    told: Option<&'static str>,
    /// The file that remembers the CPU notice was shown; `None` keeps it in memory only.
    marker: Option<PathBuf>,
}

impl GraphicsNotices {
    /// Remembers the CPU notice in `marker`.
    pub fn new(marker: Option<PathBuf>) -> GraphicsNotices {
        GraphicsNotices { told: None, marker }
    }

    /// The marker beside the window manager's settings: `<DSA folder>\graphics-cpu-notice`.
    pub fn marker_in(dir: &Path) -> PathBuf {
        dir.join("graphics-cpu-notice")
    }

    /// Whether the renderer's new `state` is one to tell the user of now, and records it.
    pub fn should_tell(&mut self, state: &GraphicsState) -> bool {
        let name = state.as_str();
        let again = self.told == Some(name);
        match state {
            GraphicsState::Unavailable(_) => {
                self.told = Some(name);
                !again
            }
            GraphicsState::LayeredOnCpu(_) => {
                self.told = Some(name);
                let shown_before = self.marker.as_ref().is_some_and(|marker| marker.is_file());
                if again || shown_before {
                    return false;
                }
                if let Some(marker) = &self.marker {
                    if let Some(dir) = marker.parent() {
                        let _ = std::fs::create_dir_all(dir);
                    }
                    let _ = std::fs::write(marker, b"shown\n");
                }
                true
            }
            GraphicsState::Native(_) | GraphicsState::Layered(_) => {
                // On a GPU again: the CPU notice may be said again if it comes back.
                self.told = Some(name);
                if let Some(marker) = &self.marker {
                    let _ = std::fs::remove_file(marker);
                }
                false
            }
            GraphicsState::Unknown => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dsa_host_core::compositor::frame_stall::is_recovery_key;

    fn scratch(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("dsa-graphics-notice-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn pr3_no_vulkan_is_said_with_the_store_link_and_the_winget_id() {
        for korean in [false, true] {
            let state = GraphicsState::Unavailable("why".into());
            let notice = notification(&state, korean).unwrap();
            assert!(notice.text.contains(PACK_WEB_LINK), "{}", notice.text);
            assert!(notice.text.contains(PACK_WINGET_ID), "{}", notice.text);
            assert!(notice.text.contains("dsa gpus"), "{}", notice.text);
            assert!(
                is_recovery_key(&notice.key),
                "a click asks nothing of Android"
            );
        }
        assert!(notification(&GraphicsState::Native("x".into()), false).is_none());
        assert!(notification(&GraphicsState::Layered("x".into()), true).is_none());
        let cpu = notification(&GraphicsState::LayeredOnCpu("x".into()), false).unwrap();
        assert!(cpu.text.contains("slow"), "{}", cpu.text);
        assert_ne!(cpu.key, unavailable_key());
    }

    #[test]
    fn pr3_the_cpu_notice_is_said_once_and_again_only_after_a_gpu() {
        let dir = scratch("once");
        let marker = GraphicsNotices::marker_in(&dir);
        let cpu = GraphicsState::LayeredOnCpu("warp".into());
        let mut first = GraphicsNotices::new(Some(marker.clone()));
        assert!(first.should_tell(&cpu));
        assert!(
            !first.should_tell(&cpu),
            "a restarted renderer says nothing new"
        );
        // The next sign-in remembers it.
        let mut next = GraphicsNotices::new(Some(marker.clone()));
        assert!(!next.should_tell(&cpu));
        // A GPU in between, and the CPU again: said again.
        assert!(!next.should_tell(&GraphicsState::Native("arc".into())));
        assert!(next.should_tell(&cpu));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn pr3_no_vulkan_is_said_at_each_start_of_the_window_manager() {
        let mut notices = GraphicsNotices::new(None);
        let none = GraphicsState::Unavailable("why".into());
        assert!(notices.should_tell(&none));
        assert!(
            !notices.should_tell(&none),
            "not again for a restarted renderer"
        );
        assert!(GraphicsNotices::new(None).should_tell(&none));
        assert!(!notices.should_tell(&GraphicsState::Unknown));
    }
}
