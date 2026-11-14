//! What a task window's UI Automation tree says about each Android accessibility node: its
//! control type, the patterns it supports, and whether a client lists it.
//!
//! The UI Automation provider of the app window host ([`crate::app_window::accessibility`])
//! asks these questions; the answers are plain values, so they are tested on every host. The
//! numbers are UI Automation's own (UIAutomationClient.h): control type IDs from 50000,
//! pattern IDs from 10000.

use dsa_host_core::accessibility::{Node, NodeAction, Role};

/// UI Automation control type IDs.
pub mod control_type {
    pub const BUTTON: i32 = 50000;
    pub const CHECK_BOX: i32 = 50002;
    pub const COMBO_BOX: i32 = 50003;
    pub const EDIT: i32 = 50004;
    pub const HYPERLINK: i32 = 50005;
    pub const IMAGE: i32 = 50006;
    pub const LIST_ITEM: i32 = 50007;
    pub const LIST: i32 = 50008;
    pub const MENU: i32 = 50009;
    pub const MENU_ITEM: i32 = 50011;
    pub const PROGRESS_BAR: i32 = 50012;
    pub const RADIO_BUTTON: i32 = 50013;
    pub const SLIDER: i32 = 50015;
    pub const TAB: i32 = 50018;
    pub const TAB_ITEM: i32 = 50019;
    pub const TEXT: i32 = 50020;
    pub const TOOL_BAR: i32 = 50021;
    pub const GROUP: i32 = 50026;
    pub const DATA_GRID: i32 = 50028;
    pub const DOCUMENT: i32 = 50030;
    pub const WINDOW: i32 = 50032;
    pub const PANE: i32 = 50033;
    pub const SEPARATOR: i32 = 50038;
}

/// The UI Automation control patterns a node can support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pattern {
    Invoke,
    Value,
    Toggle,
    SelectionItem,
    Scroll,
    /// An editable node's text by character, word and line, and its selection.
    Text,
    /// A slider's or a progress bar's value in its range.
    RangeValue,
    ExpandCollapse,
    /// A list or a grid: its size and its items by row and column.
    Grid,
    Table,
    /// An item of a list or a grid: its row and column.
    GridItem,
    TableItem,
    /// An item its container can scroll onto the screen.
    ScrollItem,
    /// The node's default action, for clients that ask the way older ones did.
    LegacyIAccessible,
    /// A list whose items a client can find, those off the screen too.
    ItemContainer,
}

/// Every pattern, in the order [`Pattern::from_id`] looks.
const PATTERNS: [Pattern; 15] = [
    Pattern::ItemContainer,
    Pattern::Invoke,
    Pattern::Value,
    Pattern::Toggle,
    Pattern::SelectionItem,
    Pattern::Scroll,
    Pattern::Text,
    Pattern::RangeValue,
    Pattern::ExpandCollapse,
    Pattern::Grid,
    Pattern::Table,
    Pattern::GridItem,
    Pattern::TableItem,
    Pattern::ScrollItem,
    Pattern::LegacyIAccessible,
];

impl Pattern {
    /// The pattern's UI Automation ID.
    pub fn id(self) -> i32 {
        match self {
            Pattern::Invoke => 10000,
            Pattern::Value => 10002,
            Pattern::RangeValue => 10003,
            Pattern::Scroll => 10004,
            Pattern::ExpandCollapse => 10005,
            Pattern::Grid => 10006,
            Pattern::GridItem => 10007,
            Pattern::SelectionItem => 10010,
            Pattern::Table => 10012,
            Pattern::TableItem => 10013,
            Pattern::Text => 10014,
            Pattern::Toggle => 10015,
            Pattern::ScrollItem => 10017,
            Pattern::LegacyIAccessible => 10018,
            Pattern::ItemContainer => 10019,
        }
    }

    pub fn from_id(id: i32) -> Option<Pattern> {
        PATTERNS.into_iter().find(|pattern| pattern.id() == id)
    }
}

/// The UI Automation control type of a role. A switch is a button with the Toggle pattern, as
/// UI Automation describes toggle switches; a scrolling container is a pane.
pub fn control_type_of(role: Role) -> i32 {
    use control_type::*;
    match role {
        Role::Button | Role::Switch => BUTTON,
        Role::CheckBox => CHECK_BOX,
        Role::RadioButton => RADIO_BUTTON,
        Role::Edit => EDIT,
        Role::Text | Role::Heading => TEXT,
        Role::Image => IMAGE,
        Role::List => LIST,
        Role::ListItem => LIST_ITEM,
        Role::Grid => DATA_GRID,
        Role::ScrollView => PANE,
        Role::Slider => SLIDER,
        Role::ProgressBar => PROGRESS_BAR,
        Role::ComboBox => COMBO_BOX,
        Role::Tab => TAB_ITEM,
        Role::TabList => TAB,
        Role::Menu => MENU,
        Role::MenuItem => MENU_ITEM,
        Role::Toolbar => TOOL_BAR,
        Role::Link => HYPERLINK,
        Role::WebView => DOCUMENT,
        Role::Group => GROUP,
        Role::Unknown => PANE,
    }
}

/// The patterns a node supports, from what it can do and what it is.
pub fn patterns_of(node: &Node) -> Vec<Pattern> {
    let role = node.role();
    let mut patterns = Vec::new();
    let toggles = node.states.checkable || role == Role::Switch || role == Role::CheckBox;
    let selects =
        node.supports(NodeAction::Select) || matches!(role, Role::Tab | Role::RadioButton);
    if node.supports(NodeAction::Click)
        && !toggles
        && !matches!(role, Role::RadioButton | Role::Tab)
    {
        patterns.push(Pattern::Invoke);
    }
    if toggles && role != Role::RadioButton {
        patterns.push(Pattern::Toggle);
    }
    if selects {
        patterns.push(Pattern::SelectionItem);
    }
    if node.states.editable {
        patterns.push(Pattern::Value);
        patterns.push(Pattern::Text);
    }
    if node.states.scrollable
        && (node.supports(NodeAction::ScrollForward)
            || node.supports(NodeAction::ScrollBackward)
            || node.supports(NodeAction::ScrollToPosition))
    {
        patterns.push(Pattern::Scroll);
    }
    if node.range.is_some() {
        patterns.push(Pattern::RangeValue);
    }
    if node.supports(NodeAction::Expand) || node.supports(NodeAction::Collapse) {
        patterns.push(Pattern::ExpandCollapse);
    }
    if node.collection.is_some() {
        patterns.push(Pattern::Grid);
        patterns.push(Pattern::Table);
        patterns.push(Pattern::ItemContainer);
    }
    if node.collection_item.is_some() {
        patterns.push(Pattern::GridItem);
        patterns.push(Pattern::TableItem);
    }
    if node.collection_item.is_some() || node.supports(NodeAction::ShowOnScreen) {
        patterns.push(Pattern::ScrollItem);
    }
    if node.supports(NodeAction::Click) || node.supports(NodeAction::LongClick) {
        patterns.push(Pattern::LegacyIAccessible);
    }
    patterns
}

/// UI Automation's HeadingLevel property for a node: HeadingLevel_None (80050) for one that is
/// no heading, else HeadingLevel1 (80051) to HeadingLevel9 (80059). A heading whose app gives
/// no level is a level 2 heading, the level of a section's title.
pub fn heading_level(node: &Node) -> i32 {
    if !node.states.heading {
        return 80050;
    }
    let level = if node.heading_level == 0 {
        2
    } else {
        node.heading_level.min(9)
    };
    80050 + level as i32
}

/// Where a list or a grid is scrolled to, as UI Automation's scroll percent and view size:
/// `rows` items in all, of which `visible` from `first` (counted from 0) are on the screen.
/// `None` while the list does not say how many it has.
pub fn scroll_percent(rows: i32, first: i32, visible: i32) -> Option<(f64, f64)> {
    if rows <= 0 || visible <= 0 {
        return None;
    }
    let visible = visible.min(rows);
    let view = f64::from(visible) * 100.0 / f64::from(rows);
    let hidden = rows - visible;
    let percent = if hidden <= 0 {
        0.0
    } else {
        (f64::from(first.clamp(0, hidden)) * 100.0 / f64::from(hidden)).clamp(0.0, 100.0)
    };
    Some((percent, view))
}

/// How many items a list or grid of `rows` and `columns` has: a list that says no columns
/// has one.
pub fn item_count(rows: i32, columns: i32) -> i32 {
    rows.max(0).saturating_mul(columns.max(1))
}

/// The place, counted row by row from 0, of the item at `row` and `column`.
pub fn item_index(row: i32, column: i32, columns: i32) -> i32 {
    row.saturating_mul(columns.max(1)).saturating_add(column)
}

/// The row and column of the item at place `index`.
pub fn item_place(index: i32, columns: i32) -> (i32, i32) {
    let columns = columns.max(1);
    (index / columns, index % columns)
}

/// The row a scroll to `percent` of a list of `rows` items goes to.
pub fn row_at_percent(rows: i32, percent: f64) -> i32 {
    if rows <= 1 {
        return 0;
    }
    ((percent.clamp(0.0, 100.0) / 100.0) * f64::from(rows - 1)).round() as i32
}

/// Whether UI Automation's control view lists the node: everything but a layout that only
/// groups others and that the user cannot act on, which would make a screen reader stop on
/// every row's empty frame.
pub fn is_control_element(node: &Node, name: &str) -> bool {
    let role = node.role();
    if matches!(role, Role::Group | Role::Unknown) {
        return !name.is_empty() || node.states.focusable || node.states.clickable;
    }
    true
}

/// Whether the content view lists the node: a control that carries something to read or do.
pub fn is_content_element(node: &Node, name: &str) -> bool {
    is_control_element(node, name) && (!name.is_empty() || !patterns_of(node).is_empty())
}

/// The text units an editable node's TextPattern moves by. UI Automation's other units fall
/// back to the next larger one this supports, as it allows: Format to Word, Paragraph to Line
/// (a line ends at a newline, which is where a paragraph ends too), Page to Document.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextUnit {
    Character,
    Word,
    Line,
    Document,
}

impl TextUnit {
    /// The unit for UI Automation's TextUnit number (TextUnit_Character is 0, TextUnit_Format
    /// 1, TextUnit_Word 2, TextUnit_Line 3, TextUnit_Paragraph 4, TextUnit_Page 5,
    /// TextUnit_Document 6).
    pub fn from_uia(unit: i32) -> TextUnit {
        match unit {
            0 => TextUnit::Character,
            1 | 2 => TextUnit::Word,
            3 | 4 => TextUnit::Line,
            _ => TextUnit::Document,
        }
    }
}

fn is_space(unit: u16) -> bool {
    char::from_u32(u32::from(unit)).is_some_and(char::is_whitespace)
}

fn is_low_surrogate(unit: u16) -> bool {
    (0xDC00..0xE000).contains(&unit)
}

/// Whether a `unit` starts at `at` in `text` (UTF-16 code units, as Android counts a text's
/// positions). A character never starts inside a surrogate pair; a word starts where text
/// follows white space (a word owns the spaces after it, as UI Automation's words do); a line
/// starts after a newline. The start of the text starts every unit.
pub fn starts_unit(text: &[u16], at: usize, unit: TextUnit) -> bool {
    if at == 0 {
        return true;
    }
    if at >= text.len() {
        return false;
    }
    match unit {
        TextUnit::Character => !is_low_surrogate(text[at]),
        TextUnit::Word => is_space(text[at - 1]) && !is_space(text[at]),
        TextUnit::Line => text[at - 1] == u16::from(b'\n'),
        TextUnit::Document => false,
    }
}

/// The unit of `text` that holds position `at`: its start and its end. At the very end of a
/// text that is not empty, the last unit.
pub fn enclosing_unit(text: &[u16], at: usize, unit: TextUnit) -> (usize, usize) {
    let len = text.len();
    let mut start = at.min(len);
    if start == len && len > 0 {
        start -= 1;
    }
    while !starts_unit(text, start, unit) {
        start -= 1;
    }
    (start, next_boundary(text, start, unit))
}

/// The next place after `at` where a unit starts, or the end of the text.
fn next_boundary(text: &[u16], at: usize, unit: TextUnit) -> usize {
    let mut next = at + 1;
    while next < text.len() && !starts_unit(text, next, unit) {
        next += 1;
    }
    next.min(text.len())
}

/// The place before `at` where a unit starts, or the start of the text.
fn previous_boundary(text: &[u16], at: usize, unit: TextUnit) -> usize {
    let mut previous = at.min(text.len()).saturating_sub(1);
    while previous > 0 && !starts_unit(text, previous, unit) {
        previous -= 1;
    }
    previous
}

/// Moves the endpoint at `at` by `count` units (back for a negative count), stopping at
/// either end of the text: the new place and how many units it moved, as UI Automation's
/// MoveEndpointByUnit answers.
pub fn move_endpoint(text: &[u16], at: usize, unit: TextUnit, count: i32) -> (usize, i32) {
    let mut at = at.min(text.len());
    let mut moved = 0;
    while moved < count.abs() {
        let next = if count > 0 {
            if at >= text.len() {
                break;
            }
            next_boundary(text, at, unit)
        } else {
            if at == 0 {
                break;
            }
            previous_boundary(text, at, unit)
        };
        at = next;
        moved += 1;
    }
    (at, if count < 0 { -moved } else { moved })
}

/// Moves the range `(start, end)` by `count` units, as UI Automation's Move does: the range
/// first takes the whole unit its start is in, then moves to the unit `count` away. A range
/// that holds nothing (a cursor) moves as a cursor to the start of that unit. Answers the new
/// range and how many units it moved.
pub fn move_range(
    text: &[u16],
    (start, end): (usize, usize),
    unit: TextUnit,
    count: i32,
) -> ((usize, usize), i32) {
    let cursor = start == end;
    let (mut at, _) = enclosing_unit(text, start, unit);
    if cursor && start >= text.len() {
        // A cursor at the end stays there going forward, and counts the last unit as the
        // first one back.
        at = text.len();
    }
    let mut moved = 0;
    while moved < count.abs() {
        let next = if count > 0 {
            let next = next_boundary(text, at, unit);
            // A range cannot start at the end of the text, except as a cursor there.
            if next >= text.len() && !(cursor && next > at) {
                break;
            }
            next
        } else {
            if at == 0 {
                break;
            }
            previous_boundary(text, at, unit)
        };
        at = next;
        moved += 1;
    }
    let moved = if count < 0 { -moved } else { moved };
    if cursor {
        ((at, at), moved)
    } else {
        ((at, next_boundary(text, at, unit).max(at)), moved)
    }
}

/// The action a scroll of `amount` asks for: UI Automation's ScrollAmount, where 0 and 1
/// scroll back and 3 and 4 forward (LargeDecrement, SmallDecrement, NoAmount, LargeIncrement,
/// SmallIncrement).
pub fn scroll_action(amount: i32) -> Option<NodeAction> {
    match amount {
        0 | 1 => Some(NodeAction::ScrollBackward),
        3 | 4 => Some(NodeAction::ScrollForward),
        _ => None,
    }
}
