//! AT-SPI's own vocabulary: role numbers, state bits, coordinate types, and the role and
//! states each Android accessibility node takes.
//!
//! The numbers are AT-SPI's (`atspi-constants.h` in at-spi2-core), which every client on
//! the accessibility bus reads them by. They are plain values, so they are tested on every
//! host.

use dsa_host_core::accessibility::{Node, NodeAction, Role};

/// AT-SPI role numbers (`AtspiRole`).
pub mod role {
    pub const CHECK_BOX: u32 = 7;
    pub const COMBO_BOX: u32 = 11;
    pub const DIALOG: u32 = 16;
    pub const FRAME: u32 = 23;
    pub const IMAGE: u32 = 27;
    pub const LABEL: u32 = 29;
    pub const LIST: u32 = 31;
    pub const LIST_ITEM: u32 = 32;
    pub const MENU: u32 = 33;
    pub const MENU_ITEM: u32 = 35;
    pub const PAGE_TAB: u32 = 37;
    pub const PAGE_TAB_LIST: u32 = 38;
    pub const PANEL: u32 = 39;
    pub const PASSWORD_TEXT: u32 = 40;
    pub const POPUP_MENU: u32 = 41;
    pub const PROGRESS_BAR: u32 = 42;
    pub const PUSH_BUTTON: u32 = 43;
    pub const RADIO_BUTTON: u32 = 44;
    pub const SCROLL_PANE: u32 = 49;
    pub const SLIDER: u32 = 51;
    pub const TABLE: u32 = 55;
    pub const TABLE_CELL: u32 = 56;
    pub const TOGGLE_BUTTON: u32 = 62;
    pub const TOOL_BAR: u32 = 63;
    pub const APPLICATION: u32 = 75;
    pub const ENTRY: u32 = 79;
    pub const HEADING: u32 = 83;
    pub const LINK: u32 = 88;
    pub const DOCUMENT_WEB: u32 = 95;
}

/// AT-SPI state numbers (`AtspiStateType`): bit positions in the two 32-bit words of a state
/// set.
pub mod state {
    pub const ACTIVE: u32 = 1;
    pub const CHECKED: u32 = 4;
    pub const COLLAPSED: u32 = 5;
    pub const DEFUNCT: u32 = 6;
    pub const EDITABLE: u32 = 7;
    pub const ENABLED: u32 = 8;
    pub const EXPANDABLE: u32 = 9;
    pub const EXPANDED: u32 = 10;
    pub const FOCUSABLE: u32 = 11;
    pub const FOCUSED: u32 = 12;
    pub const MODAL: u32 = 16;
    pub const PRESSED: u32 = 20;
    pub const RESIZABLE: u32 = 21;
    pub const SELECTABLE: u32 = 22;
    pub const SELECTED: u32 = 23;
    pub const SENSITIVE: u32 = 24;
    pub const SHOWING: u32 = 25;
    pub const VISIBLE: u32 = 30;
    pub const CHECKABLE: u32 = 41;
    pub const READ_ONLY: u32 = 43;
}

/// AT-SPI coordinate types (`AtspiCoordType`).
pub mod coord {
    pub const SCREEN: u32 = 0;
    pub const WINDOW: u32 = 1;
    pub const PARENT: u32 = 2;
}

/// AT-SPI layers (`AtspiComponentLayer`).
pub mod layer {
    pub const WIDGET: u32 = 3;
    pub const WINDOW: u32 = 7;
}

/// A set of AT-SPI states, as the two words `GetState` answers with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StateSet(u64);

impl StateSet {
    pub fn new() -> StateSet {
        StateSet(0)
    }

    pub fn with(mut self, state: u32, on: bool) -> StateSet {
        if on && state < 64 {
            self.0 |= 1 << state;
        }
        self
    }

    pub fn insert(&mut self, state: u32) {
        *self = self.with(state, true);
    }

    pub fn contains(&self, state: u32) -> bool {
        state < 64 && self.0 & (1 << state) != 0
    }

    /// The set as AT-SPI sends it: the low word, then the high word.
    pub fn words(&self) -> [u32; 2] {
        [self.0 as u32, (self.0 >> 32) as u32]
    }
}

/// The AT-SPI role of a node. A switch is a toggle button, which every AT-SPI client knows
/// (the newer switch role is not read by older screen readers); a password field is a
/// password text, so the screen reader does not speak what is typed.
pub fn role_of(node: &Node) -> u32 {
    // The root of a dialog or a pop-up window is that window.
    match &node.window {
        Some(window) if window.modal => return role::DIALOG,
        Some(window) if window.popup && node.class_name.contains("Menu") => {
            return role::POPUP_MENU
        }
        _ => {}
    }
    let role = node.role();
    if role == Role::Edit && node.states.password {
        return role::PASSWORD_TEXT;
    }
    match role {
        Role::Button => role::PUSH_BUTTON,
        Role::CheckBox => role::CHECK_BOX,
        Role::RadioButton => role::RADIO_BUTTON,
        Role::Switch => role::TOGGLE_BUTTON,
        Role::Edit => role::ENTRY,
        Role::Text => role::LABEL,
        Role::Heading => role::HEADING,
        Role::Image => role::IMAGE,
        Role::List => role::LIST,
        Role::ListItem => role::LIST_ITEM,
        Role::Grid => role::TABLE,
        Role::ScrollView => role::SCROLL_PANE,
        Role::Slider => role::SLIDER,
        Role::ProgressBar => role::PROGRESS_BAR,
        Role::ComboBox => role::COMBO_BOX,
        Role::Tab => role::PAGE_TAB,
        Role::TabList => role::PAGE_TAB_LIST,
        Role::Menu => role::MENU,
        Role::MenuItem => role::MENU_ITEM,
        Role::Toolbar => role::TOOL_BAR,
        Role::Link => role::LINK,
        Role::WebView => role::DOCUMENT_WEB,
        Role::Group | Role::Unknown => role::PANEL,
    }
}

/// The name AT-SPI gives a role (`atspi_role_get_name`), which `GetRoleName` answers with.
pub fn role_name(role: u32) -> &'static str {
    match role {
        role::CHECK_BOX => "check box",
        role::COMBO_BOX => "combo box",
        role::DIALOG => "dialog",
        role::POPUP_MENU => "popup menu",
        role::TABLE_CELL => "table cell",
        role::FRAME => "frame",
        role::IMAGE => "image",
        role::LABEL => "label",
        role::LIST => "list",
        role::LIST_ITEM => "list item",
        role::MENU => "menu",
        role::MENU_ITEM => "menu item",
        role::PAGE_TAB => "page tab",
        role::PAGE_TAB_LIST => "page tab list",
        role::PANEL => "panel",
        role::PASSWORD_TEXT => "password text",
        role::PROGRESS_BAR => "progress bar",
        role::PUSH_BUTTON => "push button",
        role::RADIO_BUTTON => "radio button",
        role::SCROLL_PANE => "scroll pane",
        role::SLIDER => "slider",
        role::TABLE => "table",
        role::TOGGLE_BUTTON => "toggle button",
        role::TOOL_BAR => "tool bar",
        role::APPLICATION => "application",
        role::ENTRY => "entry",
        role::HEADING => "heading",
        role::LINK => "link",
        role::DOCUMENT_WEB => "document web",
        _ => "unknown",
    }
}

/// Whether a node can be selected, as an item of a list or a tab.
pub fn selectable(node: &Node) -> bool {
    node.supports(NodeAction::Select) || matches!(node.role(), Role::Tab | Role::RadioButton)
}

/// The states of a node. `focused` is whether the tree says the node has the input focus;
/// `showing` whether any of it is inside the window.
pub fn states_of(node: &Node, focused: bool, showing: bool) -> StateSet {
    let s = &node.states;
    let role = node.role();
    let toggles = s.checkable || matches!(role, Role::Switch | Role::CheckBox);
    let mut set = StateSet::new()
        .with(state::VISIBLE, true)
        .with(state::SHOWING, showing)
        .with(state::ENABLED, s.enabled)
        .with(state::SENSITIVE, s.enabled)
        .with(state::FOCUSABLE, s.focusable)
        .with(state::FOCUSED, focused || s.focused)
        .with(state::CHECKABLE, toggles)
        .with(state::CHECKED, toggles && s.checked)
        // A toggle button is read as pressed or not.
        .with(state::PRESSED, role == Role::Switch && s.checked)
        .with(state::SELECTABLE, selectable(node))
        .with(state::SELECTED, s.selected)
        .with(state::EDITABLE, s.editable);
    if s.editable && !node.supports(NodeAction::SetText) {
        set.insert(state::READ_ONLY);
    }
    // One that can collapse is expanded; one that can expand is collapsed.
    let expands = node.supports(NodeAction::Expand);
    let collapses = node.supports(NodeAction::Collapse);
    if expands || collapses {
        set.insert(state::EXPANDABLE);
        set.insert(if collapses {
            state::EXPANDED
        } else {
            state::COLLAPSED
        });
    }
    if node.window.as_ref().is_some_and(|window| window.modal) {
        set.insert(state::MODAL);
    }
    set
}

/// The attributes a screen reader reads of a node besides its role: a heading's level (2
/// when the app says none), a link's address, and how a live region's changes are said.
pub fn node_attributes(node: &Node) -> Vec<(String, String)> {
    use dsa_host_core::accessibility::LiveRegion;
    let mut attributes = Vec::new();
    if node.states.heading {
        let level = if node.heading_level == 0 {
            2
        } else {
            node.heading_level.min(9)
        };
        attributes.push(("level".to_string(), level.to_string()));
    }
    if node.link && !node.url.is_empty() {
        attributes.push(("href".to_string(), node.url.clone()));
    }
    match node.live_region {
        LiveRegion::Off => {}
        LiveRegion::Polite => attributes.push(("live".to_string(), "polite".to_string())),
        LiveRegion::Assertive => attributes.push(("live".to_string(), "assertive".to_string())),
    }
    attributes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fr18_state_sets_split_into_atspis_two_words() {
        let set = StateSet::new()
            .with(state::ENABLED, true)
            .with(state::CHECKABLE, true)
            .with(state::FOCUSED, false);
        assert_eq!(set.words(), [1 << 8, 1 << (41 - 32)]);
        assert!(set.contains(state::CHECKABLE));
        assert!(!set.contains(state::FOCUSED));
    }

    #[test]
    fn fr18_dialogs_headings_links_and_expandable_nodes_say_what_they_are() {
        use dsa_host_core::accessibility::{LiveRegion, NodeWindow};
        let dialog = Node {
            class_name: "android.widget.FrameLayout".into(),
            window: Some(NodeWindow {
                title: "Delete?".into(),
                modal: true,
                popup: false,
            }),
            ..Node::default()
        };
        assert_eq!(role_of(&dialog), role::DIALOG);
        assert!(states_of(&dialog, false, true).contains(state::MODAL));
        let menu = Node {
            class_name: "android.widget.MenuPopupWindow$MenuDropDownListView".into(),
            window: Some(NodeWindow {
                title: String::new(),
                modal: false,
                popup: true,
            }),
            ..Node::default()
        };
        assert_eq!(role_of(&menu), role::POPUP_MENU);
        let mut heading = Node::default();
        heading.states.heading = true;
        assert_eq!(
            node_attributes(&heading),
            vec![("level".to_string(), "2".to_string())]
        );
        heading.heading_level = 3;
        assert_eq!(node_attributes(&heading)[0].1, "3");
        let link = Node {
            link: true,
            url: "https://example.com".into(),
            live_region: LiveRegion::Polite,
            ..Node::default()
        };
        assert_eq!(role_of(&link), role::LINK);
        assert_eq!(
            node_attributes(&link),
            vec![
                ("href".to_string(), "https://example.com".to_string()),
                ("live".to_string(), "polite".to_string())
            ]
        );
        let group = Node {
            actions: vec![NodeAction::Collapse],
            ..Node::default()
        };
        let states = states_of(&group, false, true);
        assert!(states.contains(state::EXPANDABLE));
        assert!(states.contains(state::EXPANDED));
        assert!(!states.contains(state::COLLAPSED));
    }
}
