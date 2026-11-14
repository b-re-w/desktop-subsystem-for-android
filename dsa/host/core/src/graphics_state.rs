//! What a user's graphics renderer runs on, as their window manager reports it.
//!
//! The renderer runs the guest's Vulkan calls on the host. Where the host has no Vulkan the
//! renderer can use, Android would boot and then show nothing, with nothing in its windows
//! that says why. So the window manager tells the service what its renderer found each time
//! it starts, and the service keeps that per user: it reports it with the settings, and while
//! the renderer has nothing to draw on it does not start that user's Android, and answers the
//! start with what to install instead.

/// What the user's window manager says its graphics renderer runs on.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GraphicsState {
    /// The window manager has not said yet, or the service does not report it.
    #[default]
    Unknown,
    /// A native Vulkan driver of a GPU; the detail names it.
    Native(String),
    /// A Vulkan layered on the host's own graphics API (Dozen, Vulkan on Direct3D 12) on a GPU.
    Layered(String),
    /// The same on a software device: the CPU draws everything, so games are slow.
    LayeredOnCpu(String),
    /// No Vulkan the renderer can use; the detail says what to install.
    Unavailable(String),
}

impl GraphicsState {
    /// The state's name on the wire: empty, "native", "dozen", "dozen-cpu" or "unavailable".
    pub fn as_str(&self) -> &'static str {
        match self {
            GraphicsState::Unknown => "",
            GraphicsState::Native(_) => "native",
            GraphicsState::Layered(_) => "dozen",
            GraphicsState::LayeredOnCpu(_) => "dozen-cpu",
            GraphicsState::Unavailable(_) => "unavailable",
        }
    }

    /// The window manager's words for it.
    pub fn detail(&self) -> &str {
        match self {
            GraphicsState::Unknown => "",
            GraphicsState::Native(detail)
            | GraphicsState::Layered(detail)
            | GraphicsState::LayeredOnCpu(detail)
            | GraphicsState::Unavailable(detail) => detail,
        }
    }

    /// The state a name and a detail on the wire stand for. A name this build does not know
    /// reads as unknown, so it never stops an Android from starting.
    pub fn from_wire(state: &str, detail: &str) -> GraphicsState {
        let detail = detail.to_string();
        match state {
            "native" => GraphicsState::Native(detail),
            "dozen" => GraphicsState::Layered(detail),
            "dozen-cpu" => GraphicsState::LayeredOnCpu(detail),
            "unavailable" => GraphicsState::Unavailable(if detail.is_empty() {
                "the graphics renderer found no Vulkan it can use".to_string()
            } else {
                detail
            }),
            _ => GraphicsState::Unknown,
        }
    }

    /// Why the user's Android must not start now, if it must not.
    pub fn refuses_start(&self) -> Option<String> {
        match self {
            GraphicsState::Unavailable(detail) => Some(format!(
                "Android is not started because it would have nothing to draw on: {detail}"
            )),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pr3_each_graphics_state_crosses_the_wire_unchanged() {
        for state in [
            GraphicsState::Unknown,
            GraphicsState::Native("Intel(R) Arc(TM) Graphics".into()),
            GraphicsState::Layered("Intel(R) HD Graphics 5500 through Dozen".into()),
            GraphicsState::LayeredOnCpu("Microsoft Basic Render Driver".into()),
            GraphicsState::Unavailable("install the Compatibility Pack".into()),
        ] {
            assert_eq!(
                GraphicsState::from_wire(state.as_str(), state.detail()),
                state
            );
        }
        assert_eq!(
            GraphicsState::from_wire("something-new", "x"),
            GraphicsState::Unknown
        );
        assert!(!GraphicsState::from_wire("unavailable", "")
            .detail()
            .is_empty());
    }

    #[test]
    fn pr3_only_a_renderer_without_vulkan_keeps_android_from_starting() {
        assert!(GraphicsState::Unknown.refuses_start().is_none());
        assert!(GraphicsState::Native(String::new())
            .refuses_start()
            .is_none());
        assert!(GraphicsState::Layered(String::new())
            .refuses_start()
            .is_none());
        assert!(GraphicsState::LayeredOnCpu(String::new())
            .refuses_start()
            .is_none());
        let refused = GraphicsState::Unavailable("install the pack".into())
            .refuses_start()
            .unwrap();
        assert!(refused.contains("install the pack"), "{refused}");
    }
}
