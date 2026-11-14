//! What a task window's UI Automation tree says about each Android accessibility node.

use dsa_host_core::accessibility::{
    Collection, CollectionItem, Node, NodeAction, NodeRange, NodeStates, Role,
};
use dsa_host_windows::ui_automation::{
    control_type, control_type_of, enclosing_unit, heading_level, is_content_element,
    is_control_element, move_endpoint, move_range, patterns_of, row_at_percent, scroll_action,
    scroll_percent, Pattern, TextUnit,
};

fn node(class_name: &str, states: NodeStates, actions: &[NodeAction]) -> Node {
    Node {
        id: 1,
        class_name: class_name.into(),
        states,
        actions: actions.to_vec(),
        ..Node::default()
    }
}

#[test]
fn fr18_roles_map_to_ui_automation_control_types() {
    let cases = [
        (Role::Button, control_type::BUTTON),
        (Role::Switch, control_type::BUTTON),
        (Role::CheckBox, control_type::CHECK_BOX),
        (Role::RadioButton, control_type::RADIO_BUTTON),
        (Role::Edit, control_type::EDIT),
        (Role::Text, control_type::TEXT),
        (Role::Heading, control_type::TEXT),
        (Role::Image, control_type::IMAGE),
        (Role::List, control_type::LIST),
        (Role::ListItem, control_type::LIST_ITEM),
        (Role::ScrollView, control_type::PANE),
        (Role::Slider, control_type::SLIDER),
        (Role::ProgressBar, control_type::PROGRESS_BAR),
        (Role::ComboBox, control_type::COMBO_BOX),
        (Role::Tab, control_type::TAB_ITEM),
        (Role::TabList, control_type::TAB),
        (Role::Toolbar, control_type::TOOL_BAR),
        (Role::WebView, control_type::DOCUMENT),
        (Role::Group, control_type::GROUP),
        (Role::Link, control_type::HYPERLINK),
    ];
    for (role, expected) in cases {
        assert_eq!(control_type_of(role), expected, "{role:?}");
    }
    // UI Automation's own numbers.
    assert_eq!(control_type::BUTTON, 50000);
    assert_eq!(control_type::LIST_ITEM, 50007);
    assert_eq!(control_type::PANE, 50033);
    assert_eq!(control_type::WINDOW, 50032);
}

#[test]
fn fr18_patterns_follow_what_a_node_can_do() {
    let clickable = NodeStates {
        clickable: true,
        enabled: true,
        ..NodeStates::default()
    };
    // A Settings row: Invoke, and its default action for older clients.
    let row = node(
        "android.widget.LinearLayout",
        clickable,
        &[NodeAction::Click],
    );
    assert_eq!(
        patterns_of(&row),
        vec![Pattern::Invoke, Pattern::LegacyIAccessible]
    );
    // A switch toggles rather than invokes.
    let switch = node(
        "android.widget.Switch",
        NodeStates {
            checkable: true,
            ..clickable
        },
        &[NodeAction::Click],
    );
    assert_eq!(
        patterns_of(&switch),
        vec![Pattern::Toggle, Pattern::LegacyIAccessible]
    );
    // An editable field: Value and Text.
    let field = node(
        "android.widget.EditText",
        NodeStates {
            editable: true,
            ..clickable
        },
        &[NodeAction::Click, NodeAction::SetText],
    );
    assert_eq!(
        patterns_of(&field),
        vec![
            Pattern::Invoke,
            Pattern::Value,
            Pattern::Text,
            Pattern::LegacyIAccessible
        ]
    );
    // A selectable tab.
    let tab = node(
        "com.google.android.material.tabs.TabLayout$TabView",
        clickable,
        &[NodeAction::Click, NodeAction::Select],
    );
    assert_eq!(
        patterns_of(&tab),
        vec![Pattern::SelectionItem, Pattern::LegacyIAccessible]
    );
    // A list that scrolls.
    let list = node(
        "androidx.recyclerview.widget.RecyclerView",
        NodeStates {
            scrollable: true,
            ..NodeStates::default()
        },
        &[NodeAction::ScrollForward],
    );
    assert_eq!(patterns_of(&list), vec![Pattern::Scroll]);
    // Plain text: none.
    let text = node("android.widget.TextView", NodeStates::default(), &[]);
    assert!(patterns_of(&text).is_empty());
    for pattern in [
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
    ] {
        assert_eq!(Pattern::from_id(pattern.id()), Some(pattern));
    }
    assert_eq!(Pattern::Invoke.id(), 10000);
    assert_eq!(Pattern::Toggle.id(), 10015);
    assert_eq!(Pattern::Text.id(), 10014);
    assert_eq!(Pattern::from_id(10008), None);
}

#[test]
fn fr18_sliders_lists_and_expandable_nodes_get_their_patterns() {
    let slider = Node {
        range: Some(NodeRange {
            min: 0.0,
            max: 100.0,
            current: 40.0,
        }),
        ..node(
            "android.widget.SeekBar",
            NodeStates::default(),
            &[NodeAction::SetProgress],
        )
    };
    assert_eq!(patterns_of(&slider), vec![Pattern::RangeValue]);
    let list = Node {
        collection: Some(Collection {
            rows: 40,
            columns: 1,
            hierarchical: false,
        }),
        ..node(
            "androidx.recyclerview.widget.RecyclerView",
            NodeStates {
                scrollable: true,
                ..NodeStates::default()
            },
            &[NodeAction::ScrollToPosition],
        )
    };
    assert_eq!(
        patterns_of(&list),
        vec![Pattern::Scroll, Pattern::Grid, Pattern::Table]
    );
    let item = Node {
        collection_item: Some(CollectionItem {
            row: 3,
            row_span: 1,
            column: 0,
            column_span: 1,
            heading: false,
        }),
        ..node("android.widget.LinearLayout", NodeStates::default(), &[])
    };
    assert_eq!(
        patterns_of(&item),
        vec![Pattern::GridItem, Pattern::TableItem, Pattern::ScrollItem]
    );
    let group = node(
        "android.widget.LinearLayout",
        NodeStates::default(),
        &[NodeAction::Expand],
    );
    assert_eq!(patterns_of(&group), vec![Pattern::ExpandCollapse]);
}

#[test]
fn fr18_a_heading_has_a_level_and_other_nodes_have_none() {
    let mut title = node(
        "android.widget.TextView",
        NodeStates {
            heading: true,
            ..NodeStates::default()
        },
        &[],
    );
    // HeadingLevel2 when the app says no level.
    assert_eq!(heading_level(&title), 80052);
    title.heading_level = 1;
    assert_eq!(heading_level(&title), 80051);
    title.heading_level = 12;
    assert_eq!(heading_level(&title), 80059);
    let text = node("android.widget.TextView", NodeStates::default(), &[]);
    assert_eq!(heading_level(&text), 80050);
}

#[test]
fn fr18_a_link_is_a_hyperlink() {
    let link = Node {
        link: true,
        ..node("android.text.style.URLSpan", NodeStates::default(), &[NodeAction::Click])
    };
    assert_eq!(link.role(), Role::Link);
    assert_eq!(control_type_of(link.role()), control_type::HYPERLINK);
}

#[test]
fn fr18_scroll_percent_follows_the_items_on_the_screen() {
    // 40 rows, 10 on the screen from row 15: 15 of the 30 hidden rows are above.
    assert_eq!(scroll_percent(40, 15, 10), Some((50.0, 25.0)));
    assert_eq!(scroll_percent(40, 0, 10), Some((0.0, 25.0)));
    assert_eq!(scroll_percent(40, 30, 10), Some((100.0, 25.0)));
    // Everything shows: nothing to scroll.
    assert_eq!(scroll_percent(5, 0, 5), Some((0.0, 100.0)));
    assert_eq!(scroll_percent(0, 0, 0), None);
    // A scroll to a percent goes to the row there, even one the list has not made yet.
    assert_eq!(row_at_percent(40, 50.0), 20);
    assert_eq!(row_at_percent(40, 100.0), 39);
    assert_eq!(row_at_percent(1, 70.0), 0);
}

fn utf16(text: &str) -> Vec<u16> {
    text.encode_utf16().collect()
}

#[test]
fn fr18_text_units_split_characters_words_and_lines() {
    let text = utf16("Hello big world\nsecond line");
    // A word owns the spaces after it.
    assert_eq!(enclosing_unit(&text, 0, TextUnit::Word), (0, 6));
    assert_eq!(enclosing_unit(&text, 7, TextUnit::Word), (6, 10));
    assert_eq!(enclosing_unit(&text, 3, TextUnit::Character), (3, 4));
    // A line ends after its newline.
    assert_eq!(enclosing_unit(&text, 4, TextUnit::Line), (0, 16));
    assert_eq!(enclosing_unit(&text, 20, TextUnit::Line), (16, 27));
    assert_eq!(enclosing_unit(&text, 5, TextUnit::Document), (0, 27));
    // At the end: the last unit.
    assert_eq!(enclosing_unit(&text, 27, TextUnit::Line), (16, 27));
    // A character outside the basic plane is one character, two code units.
    let emoji = utf16("a\u{1F600}b");
    assert_eq!(enclosing_unit(&emoji, 2, TextUnit::Character), (1, 3));
    assert_eq!(TextUnit::from_uia(1), TextUnit::Word);
    assert_eq!(TextUnit::from_uia(4), TextUnit::Line);
    assert_eq!(TextUnit::from_uia(5), TextUnit::Document);
}

#[test]
fn fr18_text_ranges_move_by_unit_and_stop_at_the_ends() {
    let text = utf16("one two three");
    // The word range moves to the next word, then the one after.
    assert_eq!(move_range(&text, (0, 4), TextUnit::Word, 1), ((4, 8), 1));
    assert_eq!(move_range(&text, (0, 4), TextUnit::Word, 5), ((8, 13), 2));
    assert_eq!(move_range(&text, (8, 13), TextUnit::Word, -1), ((4, 8), -1));
    // A cursor moves as a cursor, and may reach the end.
    assert_eq!(move_range(&text, (3, 3), TextUnit::Character, 1), ((4, 4), 1));
    assert_eq!(move_range(&text, (12, 12), TextUnit::Character, 1), ((13, 13), 1));
    assert_eq!(move_range(&text, (13, 13), TextUnit::Character, 1), ((13, 13), 0));
    assert_eq!(move_range(&text, (13, 13), TextUnit::Character, -1), ((12, 12), -1));
    // An endpoint moves by units to either end.
    assert_eq!(move_endpoint(&text, 0, TextUnit::Word, 2), (8, 2));
    assert_eq!(move_endpoint(&text, 8, TextUnit::Word, 3), (13, 1));
    assert_eq!(move_endpoint(&text, 5, TextUnit::Word, -1), (4, -1));
    assert_eq!(move_endpoint(&text, 0, TextUnit::Line, -1), (0, 0));
    // An empty text has nowhere to go.
    assert_eq!(move_range(&[], (0, 0), TextUnit::Word, 1), ((0, 0), 0));
    assert_eq!(enclosing_unit(&[], 0, TextUnit::Line), (0, 0));
}

#[test]
fn fr18_empty_layouts_are_left_out_of_the_control_view() {
    let frame = node("android.widget.FrameLayout", NodeStates::default(), &[]);
    assert!(!is_control_element(&frame, ""));
    assert!(is_control_element(&frame, "Toolbar"));
    let text = node("android.widget.TextView", NodeStates::default(), &[]);
    assert!(is_control_element(&text, "Wi-Fi"));
    assert!(is_content_element(&text, "Wi-Fi"));
    assert!(!is_content_element(&text, ""));
}

#[test]
fn fr18_a_scroll_amount_names_its_direction() {
    assert_eq!(scroll_action(0), Some(NodeAction::ScrollBackward));
    assert_eq!(scroll_action(1), Some(NodeAction::ScrollBackward));
    assert_eq!(scroll_action(2), None);
    assert_eq!(scroll_action(3), Some(NodeAction::ScrollForward));
    assert_eq!(scroll_action(4), Some(NodeAction::ScrollForward));
}
