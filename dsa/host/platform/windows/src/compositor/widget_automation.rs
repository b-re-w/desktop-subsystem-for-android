//! The UI Automation tree of a widget window: a pane named after the widget, which takes the
//! keyboard, so Narrator can reach the widget's menu, and whose children are the widget's own
//! Android accessibility nodes (its text, buttons and lists), as a task window's are. Focusing
//! the pane gives the widget window the keyboard, which then opens the menu under the widget
//! with Enter, Space, the menu key or Shift+F10, as a right click opens it at the pointer.
//! Nothing is drawn over the widget for it: the widget's own buttons stay free in every corner.
//!
//! The nodes come the way a task window's do. A client that reads the window sends it
//! `WM_GETOBJECT`; that first request starts a thread that watches the widget's tree through
//! the service's local API (`HostApiClient::watch_widget_accessibility`): the service asks the
//! agent in Android to read the views of the widget, which the agent hosts, and their tree and
//! its changes come back here. Until a client asks, nothing runs and Android reads nothing;
//! once no client has asked for a while and none listens for events, the thread ends and
//! Android stops reading the widget ([`ClientPresence`]).
//!
//! Each node gets the control type of its role, its name and its bounds on the screen: the
//! widget's image is shown unscaled from the widget's top left, so a node is where the image
//! has it. Invoke, Toggle, SelectionItem, Value and Scroll become the node's accessibility
//! action in Android ([`crate::ui_automation`]): an Invoke on a button presses that button in
//! the widget, and what it opens shows as an app's window.
//!
//! The providers read a [`WidgetView`] the window writes whenever it moves or draws and the
//! watch writes as trees come; a call that needs the window is posted to it and done on its
//! thread, and so are the events of a tree's changes.

// UI Automation's constants keep their Windows names, which are matched on below.
#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use dsa_host_core::accessibility::{
    from_screen, to_screen, ActionRequest, Changes, ClientPresence, GuestAccessibility, Node,
    NodeAction, PresenceChange, Tree, TreeError,
};
use dsa_host_core::host_api::HostApiClient;
use dsa_host_core::input::WindowGeometry;
use dsa_transport::Connector;
use windows::core::{
    implement, Error, IUnknown, IUnknownImpl, Interface, BOOL, BSTR, HRESULT, PCWSTR,
};
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::System::Com::SAFEARRAY;
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Accessibility::*;
use windows_sys::Win32::UI::WindowsAndMessaging::{PostMessageW, WM_APP};

use crate::app_window::accessibility::{
    none, not_available, safe_array, variant_bool, variant_empty, variant_i4, variant_str,
};
use crate::compositor::log::{note, CompositorLogger};
use crate::ui_automation::{
    control_type, control_type_of, is_content_element, is_control_element, patterns_of,
    scroll_action, Pattern,
};

/// Posted to a widget window to take the keyboard.
pub const WM_WIDGET_FOCUS: u32 = WM_APP + 0x51;

/// Posted to a widget window to raise the events of the tree's changes that came.
pub const WM_WIDGET_TREE: u32 = WM_APP + 0x52;

/// Dials the service's local API for a widget's tree, one connection at a time: a watch holds
/// it only while it connects.
pub type TreeConnect = Arc<Mutex<Box<dyn Connector>>>;

/// The build a widget window names itself by when it watches a tree.
const BUILD: &str = "dsa-wm widget accessibility";

/// How long the watch waits for the service, and between two tries after it failed.
const CONNECT: Duration = Duration::from_secs(5);
const RETRY: Duration = Duration::from_secs(1);

/// How long the watch thread waits for news before it looks at its actions again.
const POLL: Duration = Duration::from_millis(20);

/// The most property change events one batch of changes raises; past it the window says its
/// whole tree changed instead, which costs a client one read.
const MAX_PROPERTY_EVENTS: usize = 64;

/// What a widget window's providers read.
#[derive(Debug, Default)]
pub struct WidgetView {
    /// False once the window closed.
    pub alive: bool,
    pub hwnd: isize,
    /// The widget's name.
    pub name: String,
    /// Whether the window has the keyboard.
    pub focused: bool,
    /// The room around the widget in its window while it is edited, in pixels.
    pub margin: i32,
    /// The widget's tree, as the watch last applied it.
    pub tree: Tree,
    /// Where actions go while the watch runs.
    actions: Option<Sender<ActionRequest>>,
    next_request: u32,
    /// Changes waiting for the window's thread to raise their events, each with the tree's
    /// names from before it.
    pending: Vec<(Changes, Vec<(u32, String)>)>,
}

fn lock(view: &Mutex<WidgetView>) -> MutexGuard<'_, WidgetView> {
    view.lock().unwrap_or_else(|e| e.into_inner())
}

impl WidgetView {
    /// The widget's own rectangle on the screen: its window's, less the room of an edit, as
    /// its origin and size.
    fn widget_on_screen(&self) -> ((i32, i32), (u32, u32)) {
        use windows_sys::Win32::Foundation::RECT;
        use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowRect;
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        // SAFETY: a plain read of a window's place; a stale handle only fails.
        unsafe { GetWindowRect(self.hwnd as _, &mut rect) };
        let m = self.margin;
        (
            (rect.left + m, rect.top + m),
            (
                (rect.right - rect.left - 2 * m).max(0) as u32,
                (rect.bottom - rect.top - 2 * m).max(0) as u32,
            ),
        )
    }

    /// Asks Android to do `action` on node `id` of `epoch`. An error when nothing watches
    /// the tree, or the node is gone.
    fn perform(
        &mut self,
        epoch: u32,
        id: u32,
        action: NodeAction,
        text: String,
    ) -> windows::core::Result<()> {
        if self.tree.epoch() != Some(epoch) || self.tree.node(id).is_none() {
            return Err(not_available());
        }
        self.next_request = self.next_request.wrapping_add(1);
        let request = ActionRequest {
            epoch,
            node_id: id,
            action,
            text,
            request_id: self.next_request,
            selection: (0, 0),
            progress: 0.0,
            position: (0, 0),
        };
        match &self.actions {
            Some(actions) if actions.send(request).is_ok() => Ok(()),
            _ => Err(not_available()),
        }
    }
}

/// The geometry a widget's nodes are shown with: its image, `size` pixels, unscaled in a
/// window of the same size.
pub fn widget_geometry(size: (u32, u32)) -> WindowGeometry {
    WindowGeometry::one_to_one(size, size)
}

fn invalid_operation() -> Error {
    Error::from_hresult(HRESULT(UIA_E_INVALIDOPERATION as i32))
}

/// A node's runtime ID: unique in the window while its epoch lasts.
fn runtime_id(epoch: u32, id: u32) -> [i32; 3] {
    [UiaAppendRuntimeId as i32, epoch as i32, id as i32]
}

/// Frees a VARIANT this module made.
fn free_variant(mut variant: VARIANT) {
    // SAFETY: a VARIANT made by one of the constructors this module uses, cleared once.
    unsafe {
        let _ = windows::Win32::System::Variant::VariantClear(&mut variant);
    }
}

/// The widget window: a Pane that takes the keyboard, whose children are the widget's nodes.
#[implement(
    IRawElementProviderSimple,
    IRawElementProviderFragment,
    IRawElementProviderFragmentRoot
)]
struct PaneProvider {
    view: Arc<Mutex<WidgetView>>,
}

/// One node of the widget.
#[implement(
    IRawElementProviderSimple,
    IRawElementProviderFragment,
    IInvokeProvider,
    IValueProvider,
    IToggleProvider,
    ISelectionItemProvider,
    IScrollProvider
)]
struct NodeProvider {
    view: Arc<Mutex<WidgetView>>,
    epoch: u32,
    id: u32,
}

fn pane_of(view: &Arc<Mutex<WidgetView>>) -> IRawElementProviderFragmentRoot {
    PaneProvider {
        view: Arc::clone(view),
    }
    .into()
}

fn node_of(view: &Arc<Mutex<WidgetView>>, epoch: u32, id: u32) -> IRawElementProviderFragment {
    NodeProvider {
        view: Arc::clone(view),
        epoch,
        id,
    }
    .into()
}

/// The provider a widget window answers `WM_GETOBJECT` with.
pub fn provider(view: &Arc<Mutex<WidgetView>>) -> IRawElementProviderSimple {
    PaneProvider {
        view: Arc::clone(view),
    }
    .into()
}

/// Whether `WM_GETOBJECT` with `lparam` asks for UI Automation's tree.
pub fn asks_for_tree(lparam: isize) -> bool {
    lparam as i32 == UiaRootObjectId
}

/// Answers `WM_GETOBJECT` for UI Automation's root object; None for anything else.
pub fn answer(
    hwnd: isize,
    wparam: usize,
    lparam: isize,
    provider: &IRawElementProviderSimple,
) -> Option<isize> {
    if !asks_for_tree(lparam) {
        return None;
    }
    // SAFETY: the documented answer to WM_GETOBJECT, for the window that got it.
    let result = unsafe {
        UiaReturnRawElementProvider(
            HWND(hwnd as *mut _),
            WPARAM(wparam),
            LPARAM(lparam),
            provider,
        )
    };
    Some(result.0)
}

/// The window is closing: its tree's watch stops, and UI Automation lets go of its provider.
pub fn release(
    hwnd: isize,
    view: &Arc<Mutex<WidgetView>>,
    provider: Option<IRawElementProviderSimple>,
    tree: &mut WidgetTree,
) {
    tree.close(view);
    lock(view).alive = false;
    // SAFETY: the documented way for a window that answered WM_GETOBJECT to say it is going.
    unsafe {
        UiaReturnRawElementProvider(
            HWND(hwnd as *mut _),
            WPARAM(0),
            LPARAM(0),
            None::<&IRawElementProviderSimple>,
        );
        if let Some(provider) = provider {
            let _ = UiaDisconnectProvider(&provider);
        }
    }
}

/// Says that the widget has the keyboard now.
pub fn raise_focus(view: &Arc<Mutex<WidgetView>>) {
    // SAFETY: a plain query.
    if !unsafe { UiaClientsAreListening() }.as_bool() {
        return;
    }
    let provider = provider(view);
    // SAFETY: an event of the window's own provider.
    let _ = unsafe { UiaRaiseAutomationEvent(&provider, UIA_AutomationFocusChangedEventId) };
}

/// Posts `message` to the widget window, which does it on its own thread.
fn post(view: &Mutex<WidgetView>, message: u32) -> windows::core::Result<()> {
    let view = lock(view);
    if !view.alive {
        return Err(not_available());
    }
    // SAFETY: posts to the widget's window, which may have gone; then it fails.
    unsafe { PostMessageW(view.hwnd as _, message, 0, 0) };
    Ok(())
}

impl IRawElementProviderSimple_Impl for PaneProvider_Impl {
    fn ProviderOptions(&self) -> windows::core::Result<ProviderOptions> {
        Ok(ProviderOptions_ServerSideProvider)
    }

    fn GetPatternProvider(&self, _: UIA_PATTERN_ID) -> windows::core::Result<IUnknown> {
        none()
    }

    fn GetPropertyValue(&self, property: UIA_PROPERTY_ID) -> windows::core::Result<VARIANT> {
        let view = lock(&self.view);
        Ok(match property {
            UIA_ControlTypePropertyId => variant_i4(control_type::PANE),
            UIA_NamePropertyId => variant_str(&view.name),
            UIA_FrameworkIdPropertyId => variant_str("Android"),
            UIA_IsKeyboardFocusablePropertyId => variant_bool(true),
            UIA_HasKeyboardFocusPropertyId => variant_bool(view.focused),
            _ => variant_empty(),
        })
    }

    fn HostRawElementProvider(&self) -> windows::core::Result<IRawElementProviderSimple> {
        let hwnd = lock(&self.view).hwnd;
        // SAFETY: the widget's own window.
        unsafe { UiaHostProviderFromHwnd(HWND(hwnd as *mut _)) }
    }
}

impl IRawElementProviderFragment_Impl for PaneProvider_Impl {
    fn Navigate(
        &self,
        direction: NavigateDirection,
    ) -> windows::core::Result<IRawElementProviderFragment> {
        // The pane's children are the widget's top-level nodes; its menu opens from the
        // keyboard.
        let child = {
            let view = lock(&self.view);
            let epoch = view.tree.epoch();
            let roots = view.tree.roots();
            let child = match direction {
                NavigateDirection_FirstChild => roots.first().copied(),
                NavigateDirection_LastChild => roots.last().copied(),
                _ => None,
            };
            epoch.zip(child)
        };
        match child {
            Some((epoch, id)) => Ok(node_of(&self.view, epoch, id)),
            None => none(),
        }
    }

    fn GetRuntimeId(&self) -> windows::core::Result<*mut SAFEARRAY> {
        // The window's own provider gives the pane its runtime ID.
        Ok(std::ptr::null_mut())
    }

    fn BoundingRectangle(&self) -> windows::core::Result<UiaRect> {
        // The window's own provider gives the pane its bounds.
        Ok(UiaRect::default())
    }

    fn GetEmbeddedFragmentRoots(&self) -> windows::core::Result<*mut SAFEARRAY> {
        Ok(std::ptr::null_mut())
    }

    fn SetFocus(&self) -> windows::core::Result<()> {
        post(&self.view, WM_WIDGET_FOCUS)
    }

    fn FragmentRoot(&self) -> windows::core::Result<IRawElementProviderFragmentRoot> {
        Ok(self.to_interface())
    }
}

impl IRawElementProviderFragmentRoot_Impl for PaneProvider_Impl {
    fn ElementProviderFromPoint(
        &self,
        x: f64,
        y: f64,
    ) -> windows::core::Result<IRawElementProviderFragment> {
        // A point on a node of the widget is that node; any other point is the pane itself.
        let view = lock(&self.view);
        let Some(epoch) = view.tree.epoch() else {
            return none();
        };
        let (origin, size) = view.widget_on_screen();
        let (tx, ty) = from_screen((x, y), widget_geometry(size), origin);
        match view.tree.hit(tx, ty) {
            Some(id) => {
                drop(view);
                Ok(node_of(&self.view, epoch, id))
            }
            None => none(),
        }
    }

    fn GetFocus(&self) -> windows::core::Result<IRawElementProviderFragment> {
        let view = lock(&self.view);
        match (view.tree.epoch(), view.tree.focused()) {
            (Some(epoch), Some(id)) => {
                drop(view);
                Ok(node_of(&self.view, epoch, id))
            }
            // The pane itself has the keyboard, which its HasKeyboardFocus says.
            _ => none(),
        }
    }
}

impl NodeProvider {
    /// The node, while it is in the tree of this provider's epoch.
    fn with_node<T>(&self, read: impl FnOnce(&Tree, &Node) -> T) -> windows::core::Result<T> {
        let view = lock(&self.view);
        if view.tree.epoch() != Some(self.epoch) {
            return Err(not_available());
        }
        let node = view.tree.node(self.id).ok_or_else(not_available)?;
        Ok(read(&view.tree, node))
    }

    fn act(&self, action: NodeAction, text: String) -> windows::core::Result<()> {
        let supported = self.with_node(|_, node| node.supports(action))?;
        if !supported {
            return Err(invalid_operation());
        }
        lock(&self.view).perform(self.epoch, self.id, action, text)
    }
}

impl IRawElementProviderSimple_Impl for NodeProvider_Impl {
    fn ProviderOptions(&self) -> windows::core::Result<ProviderOptions> {
        Ok(ProviderOptions_ServerSideProvider)
    }

    fn GetPatternProvider(&self, pattern: UIA_PATTERN_ID) -> windows::core::Result<IUnknown> {
        let wanted = Pattern::from_id(pattern.0);
        let supported =
            self.with_node(|_, node| wanted.is_some_and(|p| patterns_of(node).contains(&p)))?;
        if supported {
            Ok(self.to_interface::<IRawElementProviderSimple>().cast()?)
        } else {
            none()
        }
    }

    fn GetPropertyValue(&self, property: UIA_PROPERTY_ID) -> windows::core::Result<VARIANT> {
        self.with_node(|tree, node| {
            let name = tree.name(node.id);
            match property {
                UIA_ControlTypePropertyId => variant_i4(control_type_of(node.role())),
                UIA_NamePropertyId => variant_str(&name),
                UIA_AutomationIdPropertyId if !node.view_id.is_empty() => {
                    variant_str(&node.view_id)
                }
                UIA_ClassNamePropertyId => variant_str(&node.class_name),
                UIA_FrameworkIdPropertyId => variant_str("Android"),
                UIA_HelpTextPropertyId if !node.hint.is_empty() => variant_str(&node.hint),
                UIA_ItemStatusPropertyId if !node.state_description.is_empty() => {
                    variant_str(&node.state_description)
                }
                UIA_IsEnabledPropertyId => variant_bool(node.states.enabled),
                UIA_HasKeyboardFocusPropertyId => variant_bool(node.states.focused),
                UIA_IsKeyboardFocusablePropertyId => variant_bool(node.states.focusable),
                UIA_IsPasswordPropertyId => variant_bool(node.states.password),
                UIA_IsControlElementPropertyId => variant_bool(is_control_element(node, &name)),
                UIA_IsContentElementPropertyId => variant_bool(is_content_element(node, &name)),
                UIA_ValueValuePropertyId if node.states.editable => {
                    variant_str(&node.value().unwrap_or_default())
                }
                UIA_ValueIsReadOnlyPropertyId if node.states.editable => {
                    variant_bool(!node.supports(NodeAction::SetText))
                }
                UIA_ToggleToggleStatePropertyId if node.states.checkable => {
                    variant_i4(if node.states.checked {
                        ToggleState_On.0
                    } else {
                        ToggleState_Off.0
                    })
                }
                UIA_SelectionItemIsSelectedPropertyId => variant_bool(node.states.selected),
                _ => variant_empty(),
            }
        })
    }

    fn HostRawElementProvider(&self) -> windows::core::Result<IRawElementProviderSimple> {
        none()
    }
}

/// Where a navigation from a node goes.
enum Target {
    Pane,
    Node(u32),
}

impl IRawElementProviderFragment_Impl for NodeProvider_Impl {
    fn Navigate(
        &self,
        direction: NavigateDirection,
    ) -> windows::core::Result<IRawElementProviderFragment> {
        let target = self.with_node(|tree, node| match direction {
            NavigateDirection_Parent => {
                Some(tree.parent(node.id).map_or(Target::Pane, Target::Node))
            }
            NavigateDirection_NextSibling => tree.sibling(node.id, true).map(Target::Node),
            NavigateDirection_PreviousSibling => tree.sibling(node.id, false).map(Target::Node),
            NavigateDirection_FirstChild => node.children.first().copied().map(Target::Node),
            NavigateDirection_LastChild => node.children.last().copied().map(Target::Node),
            _ => None,
        })?;
        match target {
            Some(Target::Pane) => Ok(pane_of(&self.view).cast()?),
            Some(Target::Node(id)) => Ok(node_of(&self.view, self.epoch, id)),
            None => none(),
        }
    }

    fn GetRuntimeId(&self) -> windows::core::Result<*mut SAFEARRAY> {
        safe_array(&runtime_id(self.epoch, self.id))
    }

    fn BoundingRectangle(&self) -> windows::core::Result<UiaRect> {
        let (origin, size) = lock(&self.view).widget_on_screen();
        let bounds = self.with_node(|_, node| node.bounds)?;
        let rect = to_screen(bounds, widget_geometry(size), origin);
        Ok(UiaRect {
            left: rect.left,
            top: rect.top,
            width: rect.width,
            height: rect.height,
        })
    }

    fn GetEmbeddedFragmentRoots(&self) -> windows::core::Result<*mut SAFEARRAY> {
        Ok(std::ptr::null_mut())
    }

    fn SetFocus(&self) -> windows::core::Result<()> {
        let focus = self.with_node(|_, node| {
            if node.supports(NodeAction::Focus) {
                NodeAction::Focus
            } else {
                NodeAction::AccessibilityFocus
            }
        })?;
        self.act(focus, String::new())
    }

    fn FragmentRoot(&self) -> windows::core::Result<IRawElementProviderFragmentRoot> {
        Ok(pane_of(&self.view))
    }
}

impl IInvokeProvider_Impl for NodeProvider_Impl {
    fn Invoke(&self) -> windows::core::Result<()> {
        // Presses the button in the widget, as a tap on it does, without touching the screen.
        self.act(NodeAction::Click, String::new())
    }
}

impl IValueProvider_Impl for NodeProvider_Impl {
    fn SetValue(&self, value: &PCWSTR) -> windows::core::Result<()> {
        // SAFETY: UI Automation hands a NUL-terminated string that lives through the call.
        let text = unsafe { value.to_string() }.map_err(|_| invalid_operation())?;
        self.act(NodeAction::SetText, text)
    }

    fn Value(&self) -> windows::core::Result<BSTR> {
        self.with_node(|_, node| BSTR::from(node.value().unwrap_or_default()))
    }

    fn IsReadOnly(&self) -> windows::core::Result<BOOL> {
        self.with_node(|_, node| BOOL::from(!node.supports(NodeAction::SetText)))
    }
}

impl IToggleProvider_Impl for NodeProvider_Impl {
    fn Toggle(&self) -> windows::core::Result<()> {
        self.act(NodeAction::Click, String::new())
    }

    fn ToggleState(&self) -> windows::core::Result<ToggleState> {
        self.with_node(|_, node| {
            if node.states.checked {
                ToggleState_On
            } else {
                ToggleState_Off
            }
        })
    }
}

impl ISelectionItemProvider_Impl for NodeProvider_Impl {
    fn Select(&self) -> windows::core::Result<()> {
        let action = self.with_node(|_, node| {
            if node.supports(NodeAction::Select) {
                NodeAction::Select
            } else {
                NodeAction::Click
            }
        })?;
        self.act(action, String::new())
    }

    fn AddToSelection(&self) -> windows::core::Result<()> {
        self.Select()
    }

    fn RemoveFromSelection(&self) -> windows::core::Result<()> {
        Err(invalid_operation())
    }

    fn IsSelected(&self) -> windows::core::Result<BOOL> {
        self.with_node(|_, node| BOOL::from(node.states.selected))
    }

    fn SelectionContainer(&self) -> windows::core::Result<IRawElementProviderSimple> {
        let parent = self.with_node(|tree, node| tree.parent(node.id))?;
        match parent {
            Some(id) => Ok(node_of(&self.view, self.epoch, id).cast()?),
            None => none(),
        }
    }
}

impl IScrollProvider_Impl for NodeProvider_Impl {
    fn Scroll(
        &self,
        horizontal: ScrollAmount,
        vertical: ScrollAmount,
    ) -> windows::core::Result<()> {
        // Android scrolls a view forward or backward, whichever way it scrolls.
        match scroll_action(vertical.0).or_else(|| scroll_action(horizontal.0)) {
            Some(action) => self.act(action, String::new()),
            None => Ok(()),
        }
    }

    fn SetScrollPercent(&self, _: f64, _: f64) -> windows::core::Result<()> {
        Err(invalid_operation())
    }

    fn HorizontalScrollPercent(&self) -> windows::core::Result<f64> {
        Ok(UIA_ScrollPatternNoScroll)
    }

    fn VerticalScrollPercent(&self) -> windows::core::Result<f64> {
        Ok(UIA_ScrollPatternNoScroll)
    }

    fn HorizontalViewSize(&self) -> windows::core::Result<f64> {
        Ok(100.0)
    }

    fn VerticalViewSize(&self) -> windows::core::Result<f64> {
        Ok(100.0)
    }

    fn HorizontallyScrollable(&self) -> windows::core::Result<BOOL> {
        Ok(BOOL::from(false))
    }

    fn VerticallyScrollable(&self) -> windows::core::Result<BOOL> {
        self.with_node(|_, node| BOOL::from(node.states.scrollable))
    }
}

/// The watch thread of a widget window, while it runs.
struct Watch {
    stop: Arc<AtomicBool>,
    thread: JoinHandle<()>,
}

/// When a widget window reads its widget's tree from Android, kept by the window on its own
/// thread: from the first time a client asks for the window's tree until none has asked for a
/// while and none listens for events.
pub struct WidgetTree {
    widget_id: i32,
    presence: ClientPresence,
    watch: Option<Watch>,
    connect: Option<TreeConnect>,
    logger: Option<Arc<CompositorLogger>>,
}

impl WidgetTree {
    /// The tree of widget `widget_id`'s window, which watches nothing until a client asks for
    /// it. Without `connect` the window answers clients with its pane alone.
    pub fn new(
        widget_id: i32,
        connect: Option<TreeConnect>,
        logger: Option<Arc<CompositorLogger>>,
    ) -> WidgetTree {
        WidgetTree {
            widget_id,
            presence: ClientPresence::default(),
            watch: None,
            connect,
            logger,
        }
    }

    /// A client asked for the window's tree: Android is asked for the widget's nodes, if no
    /// watch runs yet.
    pub fn asked(&mut self, view: &Arc<Mutex<WidgetView>>) {
        if self.presence.asked(Instant::now()) == Some(PresenceChange::Start) {
            self.start(view);
        }
    }

    fn start(&mut self, view: &Arc<Mutex<WidgetView>>) {
        let Some(connect) = self.connect.clone() else {
            return;
        };
        if self.watch.is_some() {
            return;
        }
        let (tx, actions) = mpsc::channel();
        lock(view).actions = Some(tx);
        let stop = Arc::new(AtomicBool::new(false));
        let widget_id = self.widget_id;
        let thread_view = Arc::clone(view);
        let thread_stop = Arc::clone(&stop);
        let logger = self.logger.clone();
        note(
            self.logger.as_deref(),
            &format!(
                "widget {widget_id}: a UI Automation client reads the window; watching its accessibility tree"
            ),
        );
        let spawned = thread::Builder::new()
            .name("dsa-wm-widget-accessibility".into())
            .spawn(move || {
                watch(
                    widget_id,
                    connect,
                    thread_view,
                    actions,
                    thread_stop,
                    logger,
                )
            });
        match spawned {
            Ok(thread) => self.watch = Some(Watch { stop, thread }),
            Err(why) => {
                note(
                    self.logger.as_deref(),
                    &format!("could not watch a widget's accessibility tree: {why}"),
                );
                lock(view).actions = None;
                self.presence.end();
            }
        }
    }

    fn stop(&mut self, view: &Arc<Mutex<WidgetView>>) {
        if let Some(watch) = self.watch.take() {
            watch.stop.store(true, Ordering::Relaxed);
            let _ = watch.thread.join();
            note(
                self.logger.as_deref(),
                &format!(
                    "widget {}: no UI Automation client reads the window; stopped watching its accessibility tree",
                    self.widget_id
                ),
            );
        }
        let mut view = lock(view);
        view.actions = None;
        let names = names_of(&view.tree);
        let changes = view.tree.clear();
        view.pending.push((changes, names));
    }

    /// Checks, every so often on the window's thread, whether clients have gone; Android
    /// stops reading the widget once they have.
    pub fn tick(&mut self, view: &Arc<Mutex<WidgetView>>) {
        // SAFETY: a plain query.
        let listening = unsafe { UiaClientsAreListening() }.as_bool();
        if self.presence.poll(Instant::now(), listening) == Some(PresenceChange::Stop) {
            self.stop(view);
            let _ = post(view, WM_WIDGET_TREE);
        }
        // A watch that ended by itself lets a later request start a new one.
        if self
            .watch
            .as_ref()
            .is_some_and(|watch| watch.thread.is_finished())
        {
            self.stop(view);
            self.presence.end();
        }
    }

    /// The window is closing: the watch stops.
    fn close(&mut self, view: &Arc<Mutex<WidgetView>>) {
        self.stop(view);
        self.presence.end();
    }
}

fn names_of(tree: &Tree) -> Vec<(u32, String)> {
    let mut names = Vec::new();
    let mut stack: Vec<u32> = tree.roots().to_vec();
    while let Some(id) = stack.pop() {
        if let Some(node) = tree.node(id) {
            names.push((id, tree.name(id)));
            stack.extend(node.children.iter().copied());
        }
    }
    names
}

/// Raises the UI Automation events of the changes the watch received, on the window's thread;
/// `root` is the provider the window answered with.
pub fn raise(view: &Arc<Mutex<WidgetView>>, root: &IRawElementProviderSimple) {
    let (pending, epoch, focused_window) = {
        let mut state = lock(view);
        (
            std::mem::take(&mut state.pending),
            state.tree.epoch(),
            state.focused,
        )
    };
    if pending.is_empty() {
        return;
    }
    // SAFETY: a plain query.
    if !unsafe { UiaClientsAreListening() }.as_bool() {
        return;
    }
    for (changes, before) in pending {
        raise_changes(view, root, epoch, &changes, &before, focused_window);
    }
}

fn raise_changes(
    view: &Arc<Mutex<WidgetView>>,
    root: &IRawElementProviderSimple,
    epoch: Option<u32>,
    changes: &Changes,
    before: &[(u32, String)],
    focused_window: bool,
) {
    // SAFETY: UI Automation's event calls, with providers of this window.
    unsafe {
        if changes.replaced || changes.roots_changed {
            let _ = UiaRaiseStructureChangedEvent(
                root,
                StructureChangeType_ChildrenInvalidated,
                std::ptr::null_mut(),
                0,
            );
        }
        let Some(epoch) = epoch else {
            return;
        };
        let simple =
            |id: u32| -> Option<IRawElementProviderSimple> { node_of(view, epoch, id).cast().ok() };
        if !changes.replaced {
            for id in &changes.children_changed {
                if let Some(provider) = simple(*id) {
                    let mut runtime = runtime_id(epoch, *id);
                    let _ = UiaRaiseStructureChangedEvent(
                        &provider,
                        StructureChangeType_ChildrenInvalidated,
                        runtime.as_mut_ptr(),
                        runtime.len() as i32,
                    );
                }
            }
            let mut events: Vec<(u32, UIA_PROPERTY_ID, VARIANT, VARIANT)> = Vec::new();
            {
                let state = lock(view);
                let old_names: HashMap<u32, &String> =
                    before.iter().map(|(id, name)| (*id, name)).collect();
                for old in &changes.changed {
                    let Some(new) = state.tree.node(old.id) else {
                        continue;
                    };
                    if old.states.checked != new.states.checked && new.states.checkable {
                        let state_of = |on: bool| {
                            if on {
                                ToggleState_On.0
                            } else {
                                ToggleState_Off.0
                            }
                        };
                        events.push((
                            new.id,
                            UIA_ToggleToggleStatePropertyId,
                            variant_i4(state_of(old.states.checked)),
                            variant_i4(state_of(new.states.checked)),
                        ));
                    }
                    if old.bounds != new.bounds {
                        events.push((
                            new.id,
                            UIA_BoundingRectanglePropertyId,
                            variant_empty(),
                            variant_empty(),
                        ));
                    }
                    // A clock's or a list row's text is its name, or its parent's.
                    let mut at = Some(new.id);
                    while let Some(id) = at {
                        let name = state.tree.name(id);
                        let was = old_names.get(&id).map(|name| name.as_str()).unwrap_or("");
                        if name != was {
                            events.push((
                                id,
                                UIA_NamePropertyId,
                                variant_str(was),
                                variant_str(&name),
                            ));
                        }
                        at = state.tree.parent(id);
                        if events.len() > MAX_PROPERTY_EVENTS {
                            break;
                        }
                    }
                }
            }
            let many = events.len() > MAX_PROPERTY_EVENTS;
            if many {
                let _ = UiaRaiseStructureChangedEvent(
                    root,
                    StructureChangeType_ChildrenInvalidated,
                    std::ptr::null_mut(),
                    0,
                );
            }
            for (id, property, old, new) in events {
                if !many {
                    if let Some(provider) = simple(id) {
                        let _ =
                            UiaRaiseAutomationPropertyChangedEvent(&provider, property, &old, &new);
                    }
                }
                free_variant(old);
                free_variant(new);
            }
        }
        if let (Some(focus), true) = (changes.focus, focused_window) {
            if let Some(provider) = simple(focus) {
                let _ = UiaRaiseAutomationEvent(&provider, UIA_AutomationFocusChangedEventId);
            }
        }
    }
}

/// The watch thread: watches the widget's tree through the service until told to stop, and
/// starts over when the connection ends or a tree cannot be taken.
fn watch(
    widget_id: i32,
    connect: TreeConnect,
    view: Arc<Mutex<WidgetView>>,
    actions: Receiver<ActionRequest>,
    stop: Arc<AtomicBool>,
    logger: Option<Arc<CompositorLogger>>,
) {
    let reset = |view: &Arc<Mutex<WidgetView>>| {
        let mut state = lock(view);
        let names = names_of(&state.tree);
        let changes = state.tree.clear();
        state.pending.push((changes, names));
        drop(state);
        let _ = post(view, WM_WIDGET_TREE);
    };
    let mut failures = 0u32;
    while !stop.load(Ordering::Relaxed) {
        let client = {
            let mut connector = connect.lock().unwrap_or_else(|e| e.into_inner());
            HostApiClient::connect(&mut **connector, BUILD, CONNECT)
        };
        let stream = client.and_then(|client| client.watch_widget_accessibility(widget_id));
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(why) => {
                failures += 1;
                if failures == 1 {
                    note(
                        logger.as_deref(),
                        &format!(
                            "widget {widget_id}: could not watch the accessibility tree: {why}"
                        ),
                    );
                }
                sleep_unless_stopped(&stop, RETRY);
                continue;
            }
        };
        failures = 0;
        loop {
            if stop.load(Ordering::Relaxed) {
                return;
            }
            loop {
                match actions.try_recv() {
                    Ok(action) => {
                        if stream.perform(&action).is_err() {
                            break;
                        }
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }
            match stream.next(POLL) {
                Ok(None) => {}
                Ok(Some(GuestAccessibility::Tree(update))) => {
                    let mut state = lock(&view);
                    let names = names_of(&state.tree);
                    match state.tree.apply(update) {
                        Ok(changes) => {
                            if !changes.is_empty() {
                                state.pending.push((changes, names));
                                drop(state);
                                let _ = post(&view, WM_WIDGET_TREE);
                            }
                        }
                        Err(TreeError::Epoch { .. }) => {
                            // A change that missed its tree: watch again, for a full tree.
                            drop(state);
                            break;
                        }
                        Err(TreeError::Malformed(why)) => {
                            drop(state);
                            note(
                                logger.as_deref(),
                                &format!(
                                    "widget {widget_id}: dropped an accessibility tree: {why}"
                                ),
                            );
                            reset(&view);
                            break;
                        }
                    }
                }
                Ok(Some(GuestAccessibility::ActionResult(result))) => {
                    if let Err(why) = result.outcome {
                        note(
                            logger.as_deref(),
                            &format!(
                                "widget {widget_id}: an accessibility action was not done: {why}"
                            ),
                        );
                    }
                }
                // Announcements go to tasks; a widget's watch never carries one.
                Ok(Some(GuestAccessibility::Announcement(_))) => {}
                Err(why) => {
                    note(
                        logger.as_deref(),
                        &format!("widget {widget_id}: the accessibility watch ended: {why}"),
                    );
                    reset(&view);
                    sleep_unless_stopped(&stop, RETRY);
                    break;
                }
            }
        }
    }
}

fn sleep_unless_stopped(stop: &AtomicBool, wait: Duration) {
    let until = Instant::now() + wait;
    while !stop.load(Ordering::Relaxed) && Instant::now() < until {
        thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dsa_host_core::accessibility::{NodeStates, TreeUpdate};
    use dsa_host_core::tasks::Bounds;

    /// A clock widget's tree: its time, and a button that opens its app.
    fn clock_tree() -> TreeUpdate {
        let time = Node {
            id: 2,
            class_name: "android.widget.TextClock".into(),
            text: "10:42".into(),
            bounds: Bounds {
                left: 10,
                top: 10,
                right: 150,
                bottom: 60,
            },
            states: NodeStates {
                enabled: true,
                ..NodeStates::default()
            },
            ..Node::default()
        };
        let open = Node {
            id: 3,
            class_name: "android.widget.ImageButton".into(),
            description: "Open Clock".into(),
            bounds: Bounds {
                left: 120,
                top: 70,
                right: 170,
                bottom: 100,
            },
            states: NodeStates {
                enabled: true,
                clickable: true,
                ..NodeStates::default()
            },
            actions: vec![NodeAction::Click],
            ..Node::default()
        };
        let root = Node {
            id: 1,
            children: vec![2, 3],
            class_name: "android.widget.FrameLayout".into(),
            bounds: Bounds {
                left: 0,
                top: 0,
                right: 180,
                bottom: 110,
            },
            ..Node::default()
        };
        TreeUpdate {
            task_id: 0,
            epoch: 1,
            full: true,
            roots: vec![1],
            nodes: vec![root, time, open],
            removed: vec![],
            focused: None,
            accessibility_focused: None,
        }
    }

    /// A widget's nodes are where its unscaled image has them: a node's bounds on the screen
    /// are the widget's top left plus its place in the image, cut to the widget, and a point
    /// on the screen comes back to the same node.
    #[test]
    fn fr18_a_widgets_nodes_are_where_its_image_shows_them() {
        let mut tree = Tree::new();
        tree.apply(clock_tree()).unwrap();
        let geometry = widget_geometry((180, 110));
        let button = tree.node(3).unwrap();
        let rect = to_screen(button.bounds, geometry, (500, 300));
        assert_eq!(
            (rect.left, rect.top, rect.width, rect.height),
            (620.0, 370.0, 50.0, 30.0)
        );
        let (x, y) = from_screen((630.0, 380.0), geometry, (500, 300));
        assert_eq!(tree.hit(x, y), Some(3));
        // The button is a named button an Invoke presses; the time is its text.
        assert_eq!(control_type_of(button.role()), control_type::BUTTON);
        assert_eq!(tree.name(3), "Open Clock");
        assert!(patterns_of(button).contains(&Pattern::Invoke));
        assert_eq!(tree.name(2), "10:42");
    }

    /// An Invoke goes to the watch as an action on the node, for the tree's epoch; once the
    /// watch is gone, or for a node that is gone, it is refused.
    #[test]
    fn fr18_invoke_on_a_widget_button_asks_android_to_press_it() {
        let mut view = WidgetView::default();
        view.tree.apply(clock_tree()).unwrap();
        let (tx, rx) = mpsc::channel();
        view.actions = Some(tx);
        view.perform(1, 3, NodeAction::Click, String::new())
            .unwrap();
        let sent = rx.try_recv().unwrap();
        assert_eq!(
            (sent.epoch, sent.node_id, sent.action),
            (1, 3, NodeAction::Click)
        );
        assert!(view
            .perform(2, 3, NodeAction::Click, String::new())
            .is_err());
        assert!(view
            .perform(1, 9, NodeAction::Click, String::new())
            .is_err());
        view.actions = None;
        assert!(view
            .perform(1, 3, NodeAction::Click, String::new())
            .is_err());
    }
}
