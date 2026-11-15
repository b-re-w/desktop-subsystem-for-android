//! Picture-in-picture windows: small, above every other window, in a corner of the screen.
//!
//! A Wayland toplevel can neither place itself nor stay above the others, so a task in
//! picture-in-picture leaves its toplevel (which is unmapped) for a surface that can:
//!
//! 1. a layer surface (wlr-layer-shell, which KDE Plasma and the wlroots desktops have) on the
//!    overlay layer, anchored to the bottom right corner;
//! 2. else an X11 window through XWayland (GNOME), kept above with `_NET_WM_STATE_ABOVE` and
//!    placed in the corner, which the X11 window managers honor;
//! 3. else the toplevel itself, at the small size, wherever the desktop puts it.
//!
//! Leaving picture-in-picture maps the toplevel again.

use wayland_client::protocol::wl_surface::WlSurface;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::wp::viewporter::client::wp_viewport::WpViewport;
use wayland_protocols_wlr::layer_shell::v1::client::{
    zwlr_layer_shell_v1::{Layer, ZwlrLayerShellV1},
    zwlr_layer_surface_v1::{self, Anchor, KeyboardInteractivity, ZwlrLayerSurfaceV1},
};

use super::State;
use dsa_host_core::compositor::overlay_windows::{first_place, kept_on, shown_size};
use dsa_host_core::compositor::window_hosts::FromHost;

/// The margin from the screen's corner, in logical units.
pub const MARGIN: i32 = 16;

/// Where a task in picture-in-picture is shown instead of its toplevel.
pub enum Pip {
    /// A layer surface on the overlay layer.
    Layer {
        surface: WlSurface,
        layer: ZwlrLayerSurfaceV1,
        viewport: Option<WpViewport>,
        configured: bool,
    },
    /// An X11 window, through XWayland.
    X11,
}

impl Pip {
    /// The Wayland surface the frames go to, when there is one.
    pub fn surface(&self) -> Option<&WlSurface> {
        match self {
            Pip::Layer { surface, .. } => Some(surface),
            Pip::X11 => None,
        }
    }

    pub fn is_configured(&self) -> bool {
        match self {
            Pip::Layer { configured, .. } => *configured,
            Pip::X11 => true,
        }
    }

    pub fn viewport(&mut self) -> Option<&mut Option<WpViewport>> {
        match self {
            Pip::Layer { viewport, .. } => Some(viewport),
            Pip::X11 => None,
        }
    }

    fn destroy(self) {
        if let Pip::Layer {
            surface,
            layer,
            viewport,
            ..
        } = self
        {
            if let Some(viewport) = viewport {
                viewport.destroy();
            }
            layer.destroy();
            surface.destroy();
        }
    }
}

impl State {
    /// The task's window goes into picture-in-picture at `size` pixels. Returns whether it
    /// left its toplevel for a surface that stays above the others.
    pub(crate) fn enter_pip(&mut self, task_id: i32, size: (u32, u32)) -> bool {
        if self.windows.get(&task_id).is_some_and(|w| w.pip.is_some()) {
            return true;
        }
        let Some(window) = self.windows.get(&task_id) else {
            return false;
        };
        let (width, height) = window.model.to_logical(size);
        let pip = if let Some(shell) = &self.layer_shell {
            let surface = self.compositor.create_surface(&self.qh, task_id);
            let layer = shell.get_layer_surface(
                &surface,
                None,
                Layer::Overlay,
                "dsa-picture-in-picture".into(),
                &self.qh,
                task_id,
            );
            layer.set_size(width.max(1) as u32, height.max(1) as u32);
            layer.set_anchor(Anchor::Bottom | Anchor::Right);
            layer.set_margin(0, MARGIN, MARGIN, 0);
            layer.set_keyboard_interactivity(KeyboardInteractivity::None);
            surface.commit();
            Pip::Layer {
                surface,
                layer,
                viewport: None,
                configured: false,
            }
        } else if let Some(x11) = &mut self.x11 {
            let title = window.model.title.clone();
            if !x11.open(task_id, size, &title) {
                return false;
            }
            Pip::X11
        } else {
            return false;
        };
        if let Some(window) = self.windows.get_mut(&task_id) {
            window.unmap();
            window.pip = Some(pip);
        }
        true
    }

    /// The window shows an app overlay window (floating lyrics, a chat head): it leaves its
    /// toplevel for a surface that stays above the others and takes no keyboard focus, as a
    /// picture-in-picture window does, placed where its app has the overlay
    /// ([`Self::place_overlay`]) rather than in a corner. Without a layer shell it is an X11
    /// window kept above, where the desktop puts it; without either it stays a toplevel.
    /// Moving it by a handle and remembering its place are not here yet; on a desktop with a
    /// layer shell it follows its app's moves.
    pub(crate) fn enter_overlay(&mut self, task_id: i32) -> bool {
        if self.windows.get(&task_id).is_some_and(|w| w.pip.is_some()) {
            return true;
        }
        let Some(window) = self.windows.get(&task_id) else {
            return false;
        };
        let size = window.model.shown;
        let (width, height) = window.model.to_logical(size);
        let pip = if let Some(shell) = &self.layer_shell {
            let surface = self.compositor.create_surface(&self.qh, task_id);
            let layer = shell.get_layer_surface(
                &surface,
                None,
                Layer::Overlay,
                "dsa-overlay".into(),
                &self.qh,
                task_id,
            );
            layer.set_size(width.max(1) as u32, height.max(1) as u32);
            layer.set_anchor(Anchor::Top | Anchor::Left);
            // A click in it never takes the keyboard from the window the user works in.
            layer.set_keyboard_interactivity(KeyboardInteractivity::None);
            surface.commit();
            Pip::Layer {
                surface,
                layer,
                viewport: None,
                configured: false,
            }
        } else if let Some(x11) = &mut self.x11 {
            let title = window.model.title.clone();
            if !x11.open(task_id, size, &title) {
                return false;
            }
            Pip::X11
        } else {
            return false;
        };
        if let Some(window) = self.windows.get_mut(&task_id) {
            window.unmap();
            window.pip = Some(pip);
        }
        true
    }

    /// Puts an overlay's surface where its app has the overlay: `left`, `top` is the overlay's
    /// top left corner on the guest's display, `display` the display's size. All of an app's
    /// overlays share one plane ([`OverlayPlane`]), first put where the overlay lies on the
    /// guest's display, and a move by the app moves the surface the same distance at its
    /// scale. Only a layer surface can be placed.
    pub(crate) fn place_overlay(&mut self, task_id: i32, left: i32, top: i32, display: (u32, u32)) {
        let work = self.work_area();
        let Some(window) = self.windows.get_mut(&task_id) else {
            return;
        };
        let Some(Pip::Layer { surface, layer, .. }) = &window.pip else {
            return;
        };
        // In the output's pixels, which the window's size and scale are in.
        let output = window.model.scale.max(1) as i32;
        let area = (0, 0, work.0 * output, work.1 * output);
        let size = shown_size(window.model.shown, area);
        let scale = window.model.frame_scale;
        let task = window.model.task;
        let plane = self
            .overlay_planes
            .entry(window.model.package.clone())
            .or_default();
        let wanted = plane.place((left, top), scale, || {
            first_place((left, top), task, display, size, area)
        });
        let at = kept_on(wanted, size, area);
        if at != wanted {
            plane.moved_to((left, top), at);
        }
        let (width, height) = window.model.to_logical(size);
        layer.set_size(width.max(1) as u32, height.max(1) as u32);
        layer.set_margin(at.1 / output, 0, 0, at.0 / output);
        surface.commit();
    }

    /// The task's window leaves picture-in-picture: its toplevel comes back.
    pub(crate) fn leave_pip(&mut self, task_id: i32) {
        let Some(window) = self.windows.get_mut(&task_id) else {
            return;
        };
        let Some(pip) = window.pip.take() else {
            return;
        };
        if matches!(pip, Pip::X11) {
            if let Some(x11) = &mut self.x11 {
                x11.close(task_id);
            }
        }
        pip.destroy();
        window.remap();
    }
}

impl Dispatch<ZwlrLayerSurfaceV1, i32> for State {
    fn event(
        state: &mut Self,
        layer: &ZwlrLayerSurfaceV1,
        event: zwlr_layer_surface_v1::Event,
        task_id: &i32,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        let task_id = *task_id;
        match event {
            zwlr_layer_surface_v1::Event::Configure { serial, .. } => {
                layer.ack_configure(serial);
                if let Some(Some(Pip::Layer { configured, .. })) =
                    state.windows.get_mut(&task_id).map(|w| w.pip.as_mut())
                {
                    *configured = true;
                }
                state.redraw(task_id);
            }
            zwlr_layer_surface_v1::Event::Closed => {
                state.report(FromHost::Closed { task_id });
            }
            _ => {}
        }
    }
}

wayland_client::delegate_noop!(State: ignore ZwlrLayerShellV1);
