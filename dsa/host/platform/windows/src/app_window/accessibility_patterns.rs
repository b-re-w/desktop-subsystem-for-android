//! The UI Automation patterns of a task's nodes past the first few: Text (with its ranges),
//! RangeValue, ExpandCollapse, Grid, GridItem, Table, TableItem, ScrollItem and
//! LegacyIAccessible; the context menu (long click); the loading item; and the events of
//! text, live regions, opened windows and the app's announcements.
//!
//! A text range is a span of an editable node's text in UTF-16 code units, the way Android
//! counts a selection, so a range's ends and a selection's ends are the same numbers. The
//! text is the node's value: a password's is masked, so no range reads its characters.

use std::sync::Mutex;

use windows::core::AsImpl;
use windows::Win32::System::Variant::{VT_R8, VT_UNKNOWN};

use super::*;
use crate::ui_automation::{enclosing_unit, move_endpoint, move_range, scroll_percent, TextUnit};

// --- The loading item -----------------------------------------------------------------------

/// What the window shows before the app's first tree: a text item that says it is loading.
#[implement(IRawElementProviderSimple, IRawElementProviderFragment)]
pub(super) struct LoadingProvider {
    shared: Arc<Shared>,
}

pub(super) fn loading_of(shared: &Arc<Shared>) -> IRawElementProviderFragment {
    LoadingProvider {
        shared: Arc::clone(shared),
    }
    .into()
}

fn display_loading_name() -> &'static str {
    use windows_sys::Win32::Globalization::GetUserDefaultUILanguage;
    // SAFETY: reads the user's display language.
    loading_name(is_korean(unsafe { GetUserDefaultUILanguage() }))
}

impl LoadingProvider {
    fn loading(&self) -> windows::core::Result<()> {
        if self.shared.lock().tree.epoch().is_some() {
            return Err(not_available());
        }
        Ok(())
    }
}

impl IRawElementProviderSimple_Impl for LoadingProvider_Impl {
    fn ProviderOptions(&self) -> windows::core::Result<ProviderOptions> {
        Ok(ProviderOptions_ServerSideProvider)
    }

    fn GetPatternProvider(&self, _: UIA_PATTERN_ID) -> windows::core::Result<IUnknown> {
        none()
    }

    fn GetPropertyValue(&self, property: UIA_PROPERTY_ID) -> windows::core::Result<VARIANT> {
        self.loading()?;
        Ok(match property {
            UIA_ControlTypePropertyId => variant_i4(UIA_TextControlTypeId.0),
            UIA_NamePropertyId => variant_str(display_loading_name()),
            UIA_AutomationIdPropertyId => variant_str("Loading"),
            UIA_IsEnabledPropertyId => variant_bool(true),
            UIA_IsKeyboardFocusablePropertyId => variant_bool(false),
            UIA_IsControlElementPropertyId => variant_bool(true),
            UIA_IsContentElementPropertyId => variant_bool(true),
            UIA_LiveSettingPropertyId => variant_i4(Polite.0),
            _ => variant_empty(),
        })
    }

    fn HostRawElementProvider(&self) -> windows::core::Result<IRawElementProviderSimple> {
        none()
    }
}

impl IRawElementProviderFragment_Impl for LoadingProvider_Impl {
    fn Navigate(
        &self,
        direction: NavigateDirection,
    ) -> windows::core::Result<IRawElementProviderFragment> {
        match direction {
            NavigateDirection_Parent => root_of(&self.shared).cast(),
            NavigateDirection_PreviousSibling if self.shared.lock().back.is_some() => {
                Ok(back_of(&self.shared))
            }
            _ => none(),
        }
    }

    fn GetRuntimeId(&self) -> windows::core::Result<*mut SAFEARRAY> {
        safe_array(&LOADING_RUNTIME_ID)
    }

    fn BoundingRectangle(&self) -> windows::core::Result<UiaRect> {
        let (origin, client) = self.shared.client();
        Ok(UiaRect {
            left: origin.0 as f64,
            top: origin.1 as f64,
            width: client.0 as f64,
            height: client.1 as f64,
        })
    }

    fn GetEmbeddedFragmentRoots(&self) -> windows::core::Result<*mut SAFEARRAY> {
        Ok(std::ptr::null_mut())
    }

    fn SetFocus(&self) -> windows::core::Result<()> {
        Err(invalid_operation())
    }

    fn FragmentRoot(&self) -> windows::core::Result<IRawElementProviderFragmentRoot> {
        Ok(root_of(&self.shared))
    }
}

// --- Plain answers --------------------------------------------------------------------------

/// The nearest ancestor of `id` (within a few levels, as a list's rows sit) that is a list or
/// a grid.
fn container_of(tree: &Tree, id: u32) -> Option<&Node> {
    let mut at = tree.parent(id);
    for _ in 0..3 {
        let node = tree.node(at?)?;
        if node.collection.is_some() {
            return Some(node);
        }
        at = tree.parent(node.id);
    }
    None
}

/// An item's place in its list or grid, counted from 1, and the number of items: a column of
/// rows counts rows, a row of columns counts columns, a grid counts cells row by row.
pub(super) fn place_in_set(tree: &Tree, node: &Node) -> Option<(i32, i32)> {
    let item = node.collection_item?;
    let collection = container_of(tree, node.id)?.collection?;
    let (rows, columns) = (collection.rows, collection.columns);
    let (place, size) = if columns <= 1 {
        (item.row + 1, rows)
    } else if rows <= 1 {
        (item.column + 1, columns)
    } else {
        (item.row * columns + item.column + 1, rows * columns)
    };
    (size > 0 && place > 0).then_some((place, size))
}

/// An expandable node's state: one that can collapse is expanded, one that can expand is
/// collapsed.
pub(super) fn expand_state(node: &Node) -> ExpandCollapseState {
    if node.supports(NodeAction::Collapse) {
        ExpandCollapseState_Expanded
    } else if node.supports(NodeAction::Expand) {
        ExpandCollapseState_Collapsed
    } else {
        ExpandCollapseState_LeafNode
    }
}

/// The items of a list or a grid on the screen: its children, or theirs for a list that
/// wraps its rows.
fn items_of<'a>(tree: &'a Tree, node: &Node) -> Vec<&'a Node> {
    let mut items = Vec::new();
    for child in &node.children {
        let Some(child) = tree.node(*child) else {
            continue;
        };
        if child.collection_item.is_some() {
            items.push(child);
        } else {
            items.extend(
                child
                    .children
                    .iter()
                    .filter_map(|id| tree.node(*id))
                    .filter(|grandchild| grandchild.collection_item.is_some()),
            );
        }
    }
    items
}

/// Where a list is scrolled to, from the items it shows ([`scroll_percent`]).
pub(super) fn list_scroll(tree: &Tree, node: &Node) -> Option<(f64, f64)> {
    let collection = node.collection?;
    let horizontal = collection.rows <= 1 && collection.columns > 1;
    let places: HashSet<i32> = items_of(tree, node)
        .iter()
        .filter_map(|item| item.collection_item)
        .map(|item| if horizontal { item.column } else { item.row })
        .collect();
    let first = places.iter().copied().min()?;
    let count = if horizontal {
        collection.columns
    } else {
        collection.rows
    };
    scroll_percent(count, first, places.len() as i32)
}

/// A SAFEARRAY of interface pointers, as UI Automation takes element and range lists.
fn unknown_array(items: &[IUnknown]) -> windows::core::Result<*mut SAFEARRAY> {
    // SAFETY: a new vector of interface pointers; SafeArrayPutElement adds its own reference
    // to each.
    unsafe {
        let array = SafeArrayCreateVector(VT_UNKNOWN, 0, items.len() as u32);
        if array.is_null() {
            return Err(Error::from_hresult(HRESULT(0x8007000Eu32 as i32)));
        }
        for (index, item) in items.iter().enumerate() {
            let index = index as i32;
            SafeArrayPutElement(array, &index, item.as_raw())?;
        }
        Ok(array)
    }
}

fn empty_doubles() -> windows::core::Result<*mut SAFEARRAY> {
    // SAFETY: an empty vector.
    let array = unsafe { SafeArrayCreateVector(VT_R8, 0, 0) };
    if array.is_null() {
        return Err(Error::from_hresult(HRESULT(0x8007000Eu32 as i32)));
    }
    Ok(array)
}

fn elements(shared: &Arc<Shared>, epoch: u32, ids: &[u32]) -> windows::core::Result<*mut SAFEARRAY> {
    let items: Vec<IUnknown> = ids
        .iter()
        .filter_map(|id| node_of(shared, epoch, *id).cast().ok())
        .collect();
    unknown_array(&items)
}

fn invalid_argument() -> Error {
    Error::from_hresult(HRESULT(0x80070057u32 as i32))
}

// --- Text -----------------------------------------------------------------------------------

/// A span of an editable node's text.
#[implement(ITextRangeProvider)]
pub(super) struct TextRange {
    shared: Arc<Shared>,
    epoch: u32,
    id: u32,
    span: Mutex<(usize, usize)>,
}

fn range_of(
    shared: &Arc<Shared>,
    epoch: u32,
    id: u32,
    span: (usize, usize),
) -> ITextRangeProvider {
    TextRange {
        shared: Arc::clone(shared),
        epoch,
        id,
        span: Mutex::new((span.0.min(span.1), span.0.max(span.1))),
    }
    .into()
}

impl NodeProvider {
    /// The node's text as a client reads it, in UTF-16 code units.
    fn text(&self) -> windows::core::Result<Vec<u16>> {
        self.with_node(|_, node| node.value().unwrap_or_default().encode_utf16().collect())
    }
}

impl TextRange {
    fn text(&self) -> windows::core::Result<Vec<u16>> {
        let state = self.shared.lock();
        if state.tree.epoch() != Some(self.epoch) {
            return Err(not_available());
        }
        let node = state.tree.node(self.id).ok_or_else(not_available)?;
        Ok(node.value().unwrap_or_default().encode_utf16().collect())
    }

    fn span(&self, len: usize) -> (usize, usize) {
        let (start, end) = *self.span.lock().unwrap_or_else(|e| e.into_inner());
        (start.min(len), end.min(len))
    }

    fn set(&self, start: usize, end: usize) {
        *self.span.lock().unwrap_or_else(|e| e.into_inner()) = (start.min(end), start.max(end));
    }

    /// Another range of the same node, as UI Automation passes them back.
    fn other<'a>(
        &self,
        range: &'a windows::core::Ref<'_, ITextRangeProvider>,
    ) -> windows::core::Result<&'a TextRange> {
        let range = range.ok()?;
        // SAFETY: every range UI Automation hands this provider back is one it made.
        let other: &TextRange = unsafe { range.as_impl() };
        if !Arc::ptr_eq(&other.shared, &self.shared) || other.id != self.id {
            return Err(invalid_argument());
        }
        Ok(other)
    }

    fn endpoint(&self, len: usize, which: TextPatternRangeEndpoint) -> usize {
        let (start, end) = self.span(len);
        if which == TextPatternRangeEndpoint_Start {
            start
        } else {
            end
        }
    }

    fn set_endpoint(&self, len: usize, which: TextPatternRangeEndpoint, at: usize) {
        let (start, end) = self.span(len);
        // An endpoint moved past the other takes it along.
        if which == TextPatternRangeEndpoint_Start {
            self.set(at, end.max(at));
        } else {
            self.set(start.min(at), at);
        }
    }
}

impl ITextProvider_Impl for NodeProvider_Impl {
    fn GetSelection(&self) -> windows::core::Result<*mut SAFEARRAY> {
        let len = self.text()?.len();
        let (start, end) = self
            .with_node(|_, node| node.selection)?
            .map_or((0, 0), |(start, end)| (start as usize, end as usize));
        let range: IUnknown =
            range_of(&self.shared, self.epoch, self.id, (start.min(len), end.min(len))).cast()?;
        unknown_array(&[range])
    }

    fn GetVisibleRanges(&self) -> windows::core::Result<*mut SAFEARRAY> {
        let range: IUnknown = self.DocumentRange()?.cast()?;
        unknown_array(&[range])
    }

    fn RangeFromChild(
        &self,
        _: windows::core::Ref<IRawElementProviderSimple>,
    ) -> windows::core::Result<ITextRangeProvider> {
        // An editable node's text has no elements inside it.
        Err(invalid_argument())
    }

    fn RangeFromPoint(&self, _: &UiaPoint) -> windows::core::Result<ITextRangeProvider> {
        // Android gives no place for each character: the start of the text.
        self.text()?;
        Ok(range_of(&self.shared, self.epoch, self.id, (0, 0)))
    }

    fn DocumentRange(&self) -> windows::core::Result<ITextRangeProvider> {
        let len = self.text()?.len();
        Ok(range_of(&self.shared, self.epoch, self.id, (0, len)))
    }

    fn SupportedTextSelection(&self) -> windows::core::Result<SupportedTextSelection> {
        Ok(SupportedTextSelection_Single)
    }
}

impl ITextRangeProvider_Impl for TextRange_Impl {
    fn Clone(&self) -> windows::core::Result<ITextRangeProvider> {
        let len = self.text()?.len();
        Ok(range_of(&self.shared, self.epoch, self.id, self.span(len)))
    }

    fn Compare(
        &self,
        range: windows::core::Ref<ITextRangeProvider>,
    ) -> windows::core::Result<BOOL> {
        let len = self.text()?.len();
        let other = self.other(&range)?;
        Ok(BOOL::from(self.span(len) == other.span(len)))
    }

    fn CompareEndpoints(
        &self,
        endpoint: TextPatternRangeEndpoint,
        target: windows::core::Ref<ITextRangeProvider>,
        target_endpoint: TextPatternRangeEndpoint,
    ) -> windows::core::Result<i32> {
        let len = self.text()?.len();
        let other = self.other(&target)?;
        let mine = self.endpoint(len, endpoint) as i64;
        let theirs = other.endpoint(len, target_endpoint) as i64;
        Ok((mine - theirs).signum() as i32)
    }

    fn ExpandToEnclosingUnit(&self, unit: TextUnit_) -> windows::core::Result<()> {
        let text = self.text()?;
        let (start, _) = self.span(text.len());
        let (start, end) = enclosing_unit(&text, start, TextUnit::from_uia(unit.0));
        self.set(start, end);
        Ok(())
    }

    fn FindAttribute(
        &self,
        _: UIA_TEXTATTRIBUTE_ID,
        _: &VARIANT,
        _: BOOL,
    ) -> windows::core::Result<ITextRangeProvider> {
        none()
    }

    fn FindText(
        &self,
        text: &BSTR,
        backward: BOOL,
        ignore_case: BOOL,
    ) -> windows::core::Result<ITextRangeProvider> {
        let all = self.text()?;
        let (start, end) = self.span(all.len());
        let fold = |units: &[u16]| -> Vec<u16> {
            let text = String::from_utf16_lossy(units);
            if ignore_case.as_bool() {
                text.to_lowercase().encode_utf16().collect()
            } else {
                text.encode_utf16().collect()
            }
        };
        let haystack = fold(&all[start..end]);
        let needle = fold(text.as_wide());
        if needle.is_empty() || needle.len() > haystack.len() || haystack.len() != end - start {
            return none();
        }
        let mut places = 0..=haystack.len() - needle.len();
        let found = if backward.as_bool() {
            places.rev().find(|at| haystack[*at..*at + needle.len()] == needle[..])
        } else {
            places.find(|at| haystack[*at..*at + needle.len()] == needle[..])
        };
        match found {
            Some(at) => Ok(range_of(
                &self.shared,
                self.epoch,
                self.id,
                (start + at, start + at + needle.len()),
            )),
            None => none(),
        }
    }

    fn GetAttributeValue(&self, attribute: UIA_TEXTATTRIBUTE_ID) -> windows::core::Result<VARIANT> {
        let read_only = {
            let state = self.shared.lock();
            let node = state.tree.node(self.id).ok_or_else(not_available)?;
            !node.supports(NodeAction::SetText)
        };
        if attribute == UIA_IsReadOnlyAttributeId {
            return Ok(variant_bool(read_only));
        }
        // SAFETY: UI Automation's own value for an attribute a provider does not support.
        let unsupported = unsafe { UiaGetReservedNotSupportedValue() }?;
        let mut variant = VARIANT::default();
        // SAFETY: writing the tag and the matching member of a fresh VARIANT.
        unsafe {
            let inner = &mut *variant.Anonymous.Anonymous;
            inner.vt = VT_UNKNOWN;
            inner.Anonymous.punkVal = ManuallyDrop::new(Some(unsupported));
        }
        Ok(variant)
    }

    fn GetBoundingRectangles(&self) -> windows::core::Result<*mut SAFEARRAY> {
        // Android says where the node is, not where each line of its text is.
        empty_doubles()
    }

    fn GetEnclosingElement(&self) -> windows::core::Result<IRawElementProviderSimple> {
        self.text()?;
        node_of(&self.shared, self.epoch, self.id).cast()
    }

    fn GetText(&self, max_length: i32) -> windows::core::Result<BSTR> {
        let text = self.text()?;
        let (start, mut end) = self.span(text.len());
        if max_length >= 0 {
            end = end.min(start + max_length as usize);
        }
        Ok(BSTR::from_wide(&text[start..end]))
    }

    fn Move(&self, unit: TextUnit_, count: i32) -> windows::core::Result<i32> {
        let text = self.text()?;
        let span = self.span(text.len());
        let ((start, end), moved) = move_range(&text, span, TextUnit::from_uia(unit.0), count);
        self.set(start, end);
        Ok(moved)
    }

    fn MoveEndpointByUnit(
        &self,
        endpoint: TextPatternRangeEndpoint,
        unit: TextUnit_,
        count: i32,
    ) -> windows::core::Result<i32> {
        let text = self.text()?;
        let at = self.endpoint(text.len(), endpoint);
        let (at, moved) = move_endpoint(&text, at, TextUnit::from_uia(unit.0), count);
        self.set_endpoint(text.len(), endpoint, at);
        Ok(moved)
    }

    fn MoveEndpointByRange(
        &self,
        endpoint: TextPatternRangeEndpoint,
        target: windows::core::Ref<ITextRangeProvider>,
        target_endpoint: TextPatternRangeEndpoint,
    ) -> windows::core::Result<()> {
        let len = self.text()?.len();
        let at = self.other(&target)?.endpoint(len, target_endpoint);
        self.set_endpoint(len, endpoint, at);
        Ok(())
    }

    fn Select(&self) -> windows::core::Result<()> {
        let len = self.text()?.len();
        let (start, end) = self.span(len);
        let supported = {
            let state = self.shared.lock();
            let node = state.tree.node(self.id).ok_or_else(not_available)?;
            node.supports(NodeAction::SetSelection)
        };
        if !supported {
            return Err(invalid_operation());
        }
        self.shared.perform(ActionRequest {
            epoch: self.epoch,
            node_id: self.id,
            action: NodeAction::SetSelection,
            selection: (start as u32, end as u32),
            ..ActionRequest::default()
        })
    }

    fn AddToSelection(&self) -> windows::core::Result<()> {
        Err(invalid_operation())
    }

    fn RemoveFromSelection(&self) -> windows::core::Result<()> {
        Err(invalid_operation())
    }

    fn ScrollIntoView(&self, _: BOOL) -> windows::core::Result<()> {
        Ok(())
    }

    fn GetChildren(&self) -> windows::core::Result<*mut SAFEARRAY> {
        unknown_array(&[])
    }
}

/// UI Automation's TextUnit, named apart from this module's own.
type TextUnit_ = windows::Win32::UI::Accessibility::TextUnit;

// --- Values, lists and the rest ---------------------------------------------------------------

impl IRangeValueProvider_Impl for NodeProvider_Impl {
    fn SetValue(&self, value: f64) -> windows::core::Result<()> {
        let range = self.with_node(|_, node| node.range)?.ok_or_else(invalid_operation)?;
        let value = value.clamp(f64::from(range.min), f64::from(range.max));
        self.act_with(ActionRequest {
            action: NodeAction::SetProgress,
            progress: value as f32,
            ..ActionRequest::default()
        })
    }

    fn Value(&self) -> windows::core::Result<f64> {
        self.with_node(|_, node| f64::from(node.range.map_or(0.0, |range| range.current)))
    }

    fn IsReadOnly(&self) -> windows::core::Result<BOOL> {
        self.with_node(|_, node| BOOL::from(!node.supports(NodeAction::SetProgress)))
    }

    fn Maximum(&self) -> windows::core::Result<f64> {
        self.with_node(|_, node| f64::from(node.range.map_or(0.0, |range| range.max)))
    }

    fn Minimum(&self) -> windows::core::Result<f64> {
        self.with_node(|_, node| f64::from(node.range.map_or(0.0, |range| range.min)))
    }

    fn LargeChange(&self) -> windows::core::Result<f64> {
        self.with_node(|_, node| {
            node.range
                .map_or(0.0, |range| f64::from(range.max - range.min) / 10.0)
        })
    }

    fn SmallChange(&self) -> windows::core::Result<f64> {
        self.with_node(|_, node| {
            node.range
                .map_or(0.0, |range| f64::from(range.max - range.min) / 100.0)
        })
    }
}

impl IExpandCollapseProvider_Impl for NodeProvider_Impl {
    fn Expand(&self) -> windows::core::Result<()> {
        self.act(NodeAction::Expand, String::new())
    }

    fn Collapse(&self) -> windows::core::Result<()> {
        self.act(NodeAction::Collapse, String::new())
    }

    fn ExpandCollapseState(&self) -> windows::core::Result<ExpandCollapseState> {
        self.with_node(|_, node| expand_state(node))
    }
}

impl IGridProvider_Impl for NodeProvider_Impl {
    fn GetItem(&self, row: i32, column: i32) -> windows::core::Result<IRawElementProviderSimple> {
        let found = self.with_node(|tree, node| {
            items_of(tree, node)
                .iter()
                .find(|item| {
                    item.collection_item
                        .is_some_and(|place| place.row == row && place.column == column)
                })
                .map(|item| item.id)
        })?;
        match found {
            Some(id) => node_of(&self.shared, self.epoch, id).cast(),
            // An item off the screen has not been made; SetScrollPercent brings it.
            None => Err(invalid_argument()),
        }
    }

    fn RowCount(&self) -> windows::core::Result<i32> {
        self.with_node(|_, node| node.collection.map_or(0, |c| c.rows.max(0)))
    }

    fn ColumnCount(&self) -> windows::core::Result<i32> {
        self.with_node(|_, node| node.collection.map_or(0, |c| c.columns.max(0)))
    }
}

impl IGridItemProvider_Impl for NodeProvider_Impl {
    fn Row(&self) -> windows::core::Result<i32> {
        self.with_node(|_, node| node.collection_item.map_or(0, |c| c.row))
    }

    fn Column(&self) -> windows::core::Result<i32> {
        self.with_node(|_, node| node.collection_item.map_or(0, |c| c.column))
    }

    fn RowSpan(&self) -> windows::core::Result<i32> {
        self.with_node(|_, node| node.collection_item.map_or(1, |c| c.row_span.max(1)))
    }

    fn ColumnSpan(&self) -> windows::core::Result<i32> {
        self.with_node(|_, node| node.collection_item.map_or(1, |c| c.column_span.max(1)))
    }

    fn ContainingGrid(&self) -> windows::core::Result<IRawElementProviderSimple> {
        let grid = self.with_node(|tree, node| container_of(tree, node.id).map(|grid| grid.id))?;
        match grid {
            Some(id) => node_of(&self.shared, self.epoch, id).cast(),
            None => none(),
        }
    }
}

impl NodeProvider {
    /// The heading items of the list or grid `grid` on the screen, matching `wanted`.
    fn headers(&self, grid: u32, wanted: impl Fn(&dsa_host_core::accessibility::CollectionItem) -> bool) -> windows::core::Result<*mut SAFEARRAY> {
        let ids = {
            let state = self.shared.lock();
            let node = state.tree.node(grid).ok_or_else(not_available)?;
            items_of(&state.tree, node)
                .iter()
                .filter(|item| item.collection_item.is_some_and(|c| c.heading && wanted(&c)))
                .map(|item| item.id)
                .collect::<Vec<_>>()
        };
        elements(&self.shared, self.epoch, &ids)
    }
}

impl ITableProvider_Impl for NodeProvider_Impl {
    fn GetRowHeaders(&self) -> windows::core::Result<*mut SAFEARRAY> {
        self.headers(self.id, |item| item.column == 0)
    }

    fn GetColumnHeaders(&self) -> windows::core::Result<*mut SAFEARRAY> {
        self.headers(self.id, |item| item.row == 0)
    }

    fn RowOrColumnMajor(&self) -> windows::core::Result<RowOrColumnMajor> {
        Ok(RowOrColumnMajor_RowMajor)
    }
}

impl ITableItemProvider_Impl for NodeProvider_Impl {
    fn GetRowHeaderItems(&self) -> windows::core::Result<*mut SAFEARRAY> {
        let (grid, place) = self.with_node(|tree, node| {
            (container_of(tree, node.id).map(|grid| grid.id), node.collection_item)
        })?;
        match (grid, place) {
            (Some(grid), Some(place)) => self.headers(grid, |item| item.row == place.row),
            _ => unknown_array(&[]),
        }
    }

    fn GetColumnHeaderItems(&self) -> windows::core::Result<*mut SAFEARRAY> {
        let (grid, place) = self.with_node(|tree, node| {
            (container_of(tree, node.id).map(|grid| grid.id), node.collection_item)
        })?;
        match (grid, place) {
            (Some(grid), Some(place)) => self.headers(grid, |item| item.column == place.column),
            _ => unknown_array(&[]),
        }
    }
}

impl IScrollItemProvider_Impl for NodeProvider_Impl {
    fn ScrollIntoView(&self) -> windows::core::Result<()> {
        let (show, item) = self.with_node(|_, node| {
            (node.supports(NodeAction::ShowOnScreen), node.collection_item)
        })?;
        if show {
            return self.act(NodeAction::ShowOnScreen, String::new());
        }
        // An item that cannot show itself is scrolled to through its list.
        let grid = self.with_node(|tree, node| {
            container_of(tree, node.id)
                .filter(|grid| grid.supports(NodeAction::ScrollToPosition))
                .map(|grid| grid.id)
        })?;
        match (grid, item) {
            (Some(grid), Some(place)) => self.shared.perform(ActionRequest {
                epoch: self.epoch,
                node_id: grid,
                action: NodeAction::ScrollToPosition,
                position: (place.row, place.column),
                ..ActionRequest::default()
            }),
            _ => Ok(()),
        }
    }
}

/// The default action's name in the user's display language.
fn action_name(long: bool) -> &'static str {
    use windows_sys::Win32::Globalization::GetUserDefaultUILanguage;
    // SAFETY: reads the user's display language.
    let korean = is_korean(unsafe { GetUserDefaultUILanguage() });
    match (long, korean) {
        (false, false) => "Click",
        (true, false) => "Long press",
        (false, true) => "누르기",
        (true, true) => "길게 누르기",
    }
}

impl ILegacyIAccessibleProvider_Impl for NodeProvider_Impl {
    fn Select(&self, _: i32) -> windows::core::Result<()> {
        IRawElementProviderFragment_Impl::SetFocus(self)
    }

    fn DoDefaultAction(&self) -> windows::core::Result<()> {
        let click = self.with_node(|_, node| node.supports(NodeAction::Click))?;
        self.act(
            if click {
                NodeAction::Click
            } else {
                NodeAction::LongClick
            },
            String::new(),
        )
    }

    fn SetValue(&self, value: &PCWSTR) -> windows::core::Result<()> {
        // SAFETY: UI Automation hands a NUL-terminated string that lives through the call.
        let text = unsafe { value.to_string() }.map_err(|_| invalid_operation())?;
        self.act(NodeAction::SetText, text)
    }

    fn GetIAccessible(&self) -> windows::core::Result<IAccessible> {
        // There is no MSAA object behind the node.
        none()
    }

    fn ChildId(&self) -> windows::core::Result<i32> {
        Ok(0)
    }

    fn Name(&self) -> windows::core::Result<BSTR> {
        self.with_node(|tree, node| BSTR::from(tree.name(node.id)))
    }

    fn Value(&self) -> windows::core::Result<BSTR> {
        self.with_node(|_, node| BSTR::from(node.value().unwrap_or_default()))
    }

    fn Description(&self) -> windows::core::Result<BSTR> {
        self.with_node(|_, node| BSTR::from(node.state_description.as_str()))
    }

    fn Role(&self) -> windows::core::Result<u32> {
        use dsa_host_core::accessibility::Role;
        self.with_node(|_, node| match node.role() {
            Role::Button | Role::Switch => 0x2b,
            Role::CheckBox => 0x2c,
            Role::RadioButton => 0x2d,
            Role::Link => 0x1e,
            Role::ListItem => 0x22,
            Role::Edit | Role::Text | Role::Heading => 0x2a,
            Role::Slider => 0x33,
            _ => 0x0a,
        })
    }

    fn State(&self) -> windows::core::Result<u32> {
        self.with_node(|tree, node| {
            let s = &node.states;
            let mut state = 0u32;
            if !s.enabled {
                state |= 0x1;
            }
            if s.selected {
                state |= 0x2;
            }
            if tree.client_focus() == Some(node.id) {
                state |= 0x4;
            }
            if s.checked {
                state |= 0x10;
            }
            if s.focusable {
                state |= 0x10_0000;
            }
            if s.password {
                state |= 0x2000_0000;
            }
            state
        })
    }

    fn Help(&self) -> windows::core::Result<BSTR> {
        self.with_node(|_, node| BSTR::from(node.hint.as_str()))
    }

    fn KeyboardShortcut(&self) -> windows::core::Result<BSTR> {
        Ok(BSTR::new())
    }

    fn GetSelection(&self) -> windows::core::Result<*mut SAFEARRAY> {
        unknown_array(&[])
    }

    fn DefaultAction(&self) -> windows::core::Result<BSTR> {
        self.with_node(|_, node| {
            if node.supports(NodeAction::Click) {
                BSTR::from(action_name(false))
            } else if node.supports(NodeAction::LongClick) {
                BSTR::from(action_name(true))
            } else {
                BSTR::new()
            }
        })
    }
}

impl IRawElementProviderSimple2_Impl for NodeProvider_Impl {
    /// The context menu a client asks for (Shift+F10, the Applications key) is the node's long
    /// click, which opens an Android app's context menu.
    fn ShowContextMenu(&self) -> windows::core::Result<()> {
        self.act(NodeAction::LongClick, String::new())
    }
}

// --- Events ---------------------------------------------------------------------------------

/// Raises the events a batch of changes makes besides properties and structure: text and
/// selection changes of editable nodes, live regions that changed, and dialogs and pop-ups
/// that opened.
pub(super) fn raise_node_events(shared: &Arc<Shared>, epoch: u32, changes: &Changes) {
    use dsa_host_core::accessibility::LiveRegion;
    let mut events: Vec<(u32, UIA_EVENT_ID)> = Vec::new();
    {
        let state = shared.lock();
        for old in &changes.changed {
            let Some(new) = state.tree.node(old.id) else {
                continue;
            };
            if new.states.editable {
                let text = old.value() != new.value();
                if text {
                    events.push((new.id, UIA_Text_TextChangedEventId));
                }
                if text || old.selection != new.selection {
                    events.push((new.id, UIA_Text_TextSelectionChangedEventId));
                }
            }
            if new.live_region != LiveRegion::Off
                && (old.text != new.text || old.description != new.description)
            {
                events.push((new.id, UIA_LiveRegionChangedEventId));
            }
        }
        if !changes.replaced {
            for id in &changes.added {
                let Some(node) = state.tree.node(*id) else {
                    continue;
                };
                match &node.window {
                    Some(window) if window.popup => {
                        let menu = node.class_name.contains("Menu");
                        events.push((
                            *id,
                            if menu {
                                UIA_MenuOpenedEventId
                            } else {
                                UIA_Window_WindowOpenedEventId
                            },
                        ));
                    }
                    Some(window) if window.modal => {
                        events.push((*id, UIA_Window_WindowOpenedEventId))
                    }
                    _ => {}
                }
                if node.live_region != LiveRegion::Off {
                    events.push((*id, UIA_LiveRegionChangedEventId));
                }
            }
        }
    }
    for (id, event) in events {
        if let Ok(provider) = node_of(shared, epoch, id).cast::<IRawElementProviderSimple>() {
            // SAFETY: UI Automation's event call, with a provider of this window.
            unsafe {
                let _ = UiaRaiseAutomationEvent(&provider, event);
            }
        }
    }
}

/// Raises an app's announcement or toast as a notification of the window, which a screen
/// reader says: an assertive one cuts in, a polite one waits its turn.
pub(super) fn raise_announcement(root: &IRawElementProviderSimple, announcement: &Announcement) {
    if announcement.text.is_empty() {
        return;
    }
    let processing = if announcement.assertive {
        NotificationProcessing_ImportantMostRecent
    } else {
        NotificationProcessing_All
    };
    let activity = BSTR::from(if announcement.toast {
        "dsa.android.toast"
    } else {
        "dsa.android.announcement"
    });
    // SAFETY: UI Automation's event call, with the window's own root.
    unsafe {
        let _ = UiaRaiseNotificationEvent(
            root,
            NotificationKind_Other,
            processing,
            &BSTR::from(announcement.text.as_str()),
            &activity,
        );
    }
}
