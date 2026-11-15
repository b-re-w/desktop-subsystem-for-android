//! What every page is built from: running a request off the main thread, the rows and
//! groups of a GNOME Settings page, the banner that says what went wrong, and the questions
//! the app asks.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use adw::prelude::*;
use gtk::{gio, glib};

use crate::api::Failure;
use crate::i18n::tr;
use crate::model::Client;
use crate::view;

/// What the pages share: the client of the local API and the window they are in.
pub struct Ctx {
    pub client: Client,
    pub window: adw::ApplicationWindow,
    pub toasts: adw::ToastOverlay,
    /// The app whose row the Apps page opens and brings into view the next time it lists the
    /// apps, as `--app` asks.
    pub focus_app: RefCell<Option<String>>,
}

pub type Shared = Rc<Ctx>;

impl Ctx {
    /// Says `text` for a moment at the bottom of the window, and to a screen reader, which
    /// would not hear a toast that takes no focus.
    pub fn toast(&self, text: &str) {
        let toast = adw::Toast::new(text);
        toast.set_use_markup(false);
        self.toasts.add_toast(toast);
        announce(&self.window, text, false);
    }
}

/// Has a screen reader say `text` now, through GTK's announcements (GTK 4.14 and later). The
/// app builds against GTK 4.12, which has none, so the function is looked up when the app runs;
/// with an older GTK nothing is said and the text stays on screen as before. An `important`
/// text (something that went wrong) is read at once; any other waits its turn.
pub fn announce(widget: &impl IsA<gtk::Widget>, text: &str, important: bool) {
    use gtk::glib::translate::ToGlibPtr;
    use std::os::raw::{c_char, c_int};
    type AnnounceFn = unsafe extern "C" fn(*mut gtk::ffi::GtkAccessible, *const c_char, c_int);
    static FOUND: std::sync::OnceLock<Option<AnnounceFn>> = std::sync::OnceLock::new();
    let found = *FOUND.get_or_init(|| {
        // SAFETY: dlsym only looks the name up in what the process has loaded.
        let symbol =
            unsafe { libc::dlsym(libc::RTLD_DEFAULT, c"gtk_accessible_announce".as_ptr()) };
        // SAFETY: GTK's gtk_accessible_announce takes an accessible, a message and a priority.
        (!symbol.is_null())
            .then(|| unsafe { std::mem::transmute::<*mut libc::c_void, AnnounceFn>(symbol) })
    });
    let (Some(call), Ok(message)) = (found, std::ffi::CString::new(text.trim())) else {
        return;
    };
    if message.as_bytes().is_empty() {
        return;
    }
    let widget: &gtk::Widget = widget.upcast_ref();
    let accessible: &gtk::Accessible = widget.upcast_ref();
    // GTK's announcement priorities: 0 low, 1 medium, 2 high.
    let priority: c_int = if important { 2 } else { 1 };
    // SAFETY: the widget and the message live through the call, and GTK copies the message.
    unsafe { call(accessible.to_glib_none().0, message.as_ptr(), priority) };
}

/// Shows `text` on `banner` and tells a screen reader, when the banner opens or its words
/// change: a banner takes no focus, so nothing else would say it.
pub fn reveal_banner(banner: &adw::Banner, text: &str) {
    let changed = !banner.is_revealed() || banner.title().as_str() != text;
    banner.set_title(text);
    banner.set_revealed(true);
    if changed {
        announce(banner, text, true);
    }
}

/// The accessible name `label` gets in its own words: for a button that shows only an icon,
/// or that says one word in every row of a list.
pub fn accessible_name(widget: &impl IsA<gtk::Accessible>, label: &str) {
    widget.update_property(&[gtk::accessible::Property::Label(label)]);
}

/// Tells a screen reader, with `widget`'s description, that changing it asks for an
/// administrator's permission. The page says so in its group's description, which a screen
/// reader does not read with the control. `subtitle` is what the control's description says
/// already (its row's subtitle), which the description keeps first.
pub fn needs_administrator(widget: &impl IsA<gtk::Accessible>, subtitle: &str) {
    let why = tr("This setting applies to every user of this computer, so changing it asks for an administrator's permission.");
    let description = if subtitle.is_empty() {
        why
    } else {
        format!("{subtitle}\n{why}")
    };
    widget.update_property(&[gtk::accessible::Property::Description(&description)]);
}

/// A flat button that shows only `icon`, with `label` as its tool tip and as the name a screen
/// reader says. A tool tip alone gives it no name.
pub fn icon_button(icon: &str, label: &str) -> gtk::Button {
    let button = gtk::Button::from_icon_name(icon);
    button.set_tooltip_text(Some(label));
    button.add_css_class("flat");
    accessible_name(&button, label);
    button
}

/// A button that says `label` in a row of a list, named for a screen reader as `label` and
/// `item`, such as "Open, Notes": every row has one, and "Open" alone does not say whose.
pub fn item_button(label: &str, item: &str) -> gtk::Button {
    let button = button(label);
    accessible_name(&button, &view::item_name(label, item));
    button
}

/// The first label under `root` that says `text`.
fn find_label(root: &gtk::Widget, text: &str) -> Option<gtk::Label> {
    let mut child = root.first_child();
    while let Some(widget) = child {
        if let Some(label) = widget.downcast_ref::<gtk::Label>() {
            if label.label().as_str() == text {
                return Some(label.clone());
            }
        }
        if let Some(found) = find_label(&widget, text) {
            return Some(found);
        }
        child = widget.next_sibling();
    }
    None
}

/// A label that a screen reader lists as a heading of `level`.
fn heading_label(text: &str, level: i32) -> gtk::Label {
    let heading: gtk::Label = glib::Object::builder()
        .property("accessible-role", gtk::AccessibleRole::Heading)
        .property("label", text)
        .property("use-markup", false)
        .build();
    heading.update_property(&[gtk::accessible::Property::Level(level)]);
    heading
}

/// Makes the title of `group` a heading of the second level, under the page's title. A
/// screen reader lists a page's headings and jumps between them, and libadwaita draws a
/// group's title as a plain label. The role of a GTK widget is fixed when it is made, so the
/// title is drawn by a heading label in the same place, with the same style, and libadwaita's
/// own label is hidden. A libadwaita whose title is a heading already only gets the level.
fn mark_group_heading(group: &adw::PreferencesGroup, text: &str) {
    let Some(title) = find_label(group.upcast_ref(), text) else {
        return;
    };
    if title.accessible_role() == gtk::AccessibleRole::Heading {
        title.update_property(&[gtk::accessible::Property::Level(2)]);
        return;
    }
    let Some(parent) = title.parent().and_then(|p| p.downcast::<gtk::Box>().ok()) else {
        return;
    };
    let heading = heading_label(text, 2);
    heading.set_wrap(title.wraps());
    heading.set_wrap_mode(title.wrap_mode());
    heading.set_xalign(title.xalign());
    heading.set_halign(title.halign());
    let classes: Vec<String> = title.css_classes().iter().map(|c| c.to_string()).collect();
    let classes: Vec<&str> = classes.iter().map(String::as_str).collect();
    heading.set_css_classes(&classes);
    parent.insert_child_after(&heading, title.prev_sibling().as_ref());
    title.set_visible(false);
}

/// The title of a page in its header bar, which a screen reader lists as the page's first
/// heading, as the Windows settings app's page title is.
pub fn page_title(text: &str) -> gtk::Label {
    let title = heading_label(text, 1);
    title.set_single_line_mode(true);
    title.set_ellipsize(gtk::pango::EllipsizeMode::End);
    title.add_css_class("title");
    title
}

/// Runs `work` on a worker thread, so a request that waits for the service never holds the
/// window, then `done` with its result on the main thread.
pub fn spawn<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
    done: impl FnOnce(T) + 'static,
) {
    glib::spawn_future_local(async move {
        if let Ok(value) = gio::spawn_blocking(work).await {
            done(value);
        }
    });
}

/// Runs a request of the client on a worker thread.
pub fn request<T: Send + 'static>(
    ctx: &Shared,
    work: impl FnOnce(Client) -> Result<T, Failure> + Send + 'static,
    done: impl FnOnce(Result<T, Failure>) + 'static,
) {
    let client = ctx.client.clone();
    spawn(move || work(client), done);
}

/// A page of groups with a banner above it that says what went wrong.
pub struct Page {
    pub root: gtk::Box,
    pub prefs: adw::PreferencesPage,
    pub banner: adw::Banner,
}

impl Default for Page {
    fn default() -> Page {
        Page::new()
    }
}

impl Page {
    pub fn new() -> Page {
        let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let banner = adw::Banner::builder()
            .revealed(false)
            .use_markup(false)
            .build();
        let prefs = adw::PreferencesPage::new();
        prefs.set_vexpand(true);
        root.append(&banner);
        root.append(&prefs);
        Page {
            root,
            prefs,
            banner,
        }
    }

    pub fn add(&self, group: &adw::PreferencesGroup) {
        self.prefs.add(group);
    }

    /// Shows what went wrong, or hides the banner when `failure` is `None`.
    pub fn show_failure(&self, title: &str, failure: Option<&Failure>) {
        match failure {
            Some(failure) => {
                let detail = view::problem_text(failure).replace('\n', " ");
                reveal_banner(&self.banner, &format!("{title}: {detail}"));
            }
            None => self.banner.set_revealed(false),
        }
    }
}

pub fn group(title: &str, description: Option<&str>) -> adw::PreferencesGroup {
    let group = adw::PreferencesGroup::new();
    if !title.is_empty() {
        group.set_title(title);
        mark_group_heading(&group, title);
    }
    if let Some(description) = description {
        group.set_description(Some(description));
    }
    group
}

pub fn action_row(title: &str, subtitle: &str) -> adw::ActionRow {
    let row = adw::ActionRow::builder()
        .title(title)
        .use_markup(false)
        .build();
    if !subtitle.is_empty() {
        row.set_subtitle(subtitle);
    }
    row
}

/// A row that shows a property: its name as the title and its value under it, which can be
/// selected and copied.
pub fn property_row(title: &str) -> adw::ActionRow {
    let row = action_row(title, "");
    row.set_subtitle_selectable(true);
    row.add_css_class("property");
    row
}

/// A row with a box to type a value in.
pub fn entry_row(title: &str) -> adw::EntryRow {
    adw::EntryRow::builder()
        .title(title)
        .use_markup(false)
        .build()
}

/// A row that opens to show more rows under it.
pub fn expander_row(title: &str, subtitle: &str) -> adw::ExpanderRow {
    let row = adw::ExpanderRow::builder()
        .title(title)
        .use_markup(false)
        .build();
    if !subtitle.is_empty() {
        row.set_subtitle(subtitle);
    }
    row
}

/// A block of text at the top of a group, such as a device's name over its maker, spaced from
/// the rows below it as GNOME's About shows the device.
pub fn heading_block() -> gtk::Box {
    let block = gtk::Box::new(gtk::Orientation::Vertical, 4);
    block.set_margin_bottom(6);
    block
}

/// A row that shows a value on its right, which can be selected and copied.
pub fn value_row(title: &str, value: &str) -> (adw::ActionRow, gtk::Label) {
    let row = action_row(title, "");
    let label = gtk::Label::builder()
        .label(value)
        .selectable(true)
        .wrap(true)
        .xalign(1.0)
        .build();
    label.add_css_class("dim-label");
    row.add_suffix(&label);
    (row, label)
}

/// A flat copy button that puts what `source` shows at the moment of the click on the
/// clipboard, and shows a check mark for a moment so the user sees that the copy happened.
pub fn copy_button(source: &gtk::Label) -> gtk::Button {
    let copy = gtk::Button::builder()
        .icon_name("edit-copy-symbolic")
        .tooltip_text(tr("Copy"))
        .valign(gtk::Align::Center)
        .build();
    copy.add_css_class("flat");
    copy.update_property(&[gtk::accessible::Property::Label(&tr("Copy"))]);
    let source = source.clone();
    copy.connect_clicked(move |button| {
        button.clipboard().set_text(&source.label());
        // The check mark is only seen; a screen reader says it in words.
        announce(button, &tr("Copied"), false);
        button.set_icon_name("object-select-symbolic");
        let button = button.clone();
        glib::timeout_add_local_once(std::time::Duration::from_millis(1500), move || {
            button.set_icon_name("edit-copy-symbolic");
        });
    });
    copy
}

/// A row that shows one command in monospaced, selectable text, with a copy button that
/// copies exactly that command.
pub fn command_row(command: &str) -> (adw::ActionRow, gtk::Label) {
    let row = action_row("", "");
    let label = gtk::Label::builder()
        .label(command)
        .selectable(true)
        .wrap(true)
        .xalign(0.0)
        .hexpand(true)
        .build();
    label.add_css_class("monospace");
    row.add_prefix(&label);
    row.add_suffix(&copy_button(&label));
    (row, label)
}

pub fn button(label: &str) -> gtk::Button {
    gtk::Button::builder()
        .label(label)
        .valign(gtk::Align::Center)
        .build()
}

/// A row with a button on its right that the whole row activates. The button's name says the
/// row's title too: several rows have a Remove or a Get button, and a screen reader that
/// reads only "Remove" does not say what goes.
pub fn button_row(title: &str, subtitle: &str, label: &str) -> (adw::ActionRow, gtk::Button) {
    let row = action_row(title, subtitle);
    let button = if title.is_empty() || title == label {
        button(label)
    } else {
        item_button(label, title)
    };
    row.add_suffix(&button);
    row.set_activatable_widget(Some(&button));
    (row, button)
}

/// A row that opens something elsewhere, with the arrow GNOME Settings shows for it.
pub fn link_row(title: &str, subtitle: &str) -> adw::ActionRow {
    let row = action_row(title, subtitle);
    row.set_activatable(true);
    row.add_suffix(&gtk::Image::from_icon_name("adw-external-link-symbolic"));
    row
}

/// A switch whose changes by the user run `changed`, and which [`Switch::set`] moves without
/// running it.
#[derive(Clone)]
pub struct Switch {
    pub row: adw::SwitchRow,
    quiet: Rc<Cell<bool>>,
}

impl Switch {
    pub fn new(title: &str, subtitle: &str, changed: impl Fn(bool) + 'static) -> Switch {
        let row = adw::SwitchRow::builder()
            .title(title)
            .use_markup(false)
            .build();
        if !subtitle.is_empty() {
            row.set_subtitle(subtitle);
        }
        let quiet = Rc::new(Cell::new(false));
        let q = quiet.clone();
        row.connect_active_notify(move |row| {
            if !q.get() {
                changed(row.is_active());
            }
        });
        Switch { row, quiet }
    }

    pub fn set(&self, on: bool) {
        self.quiet.set(true);
        self.row.set_active(on);
        self.quiet.set(false);
    }
}

/// A drop-down list whose choices by the user run `changed`, and which [`Combo::set`] moves
/// without running it.
#[derive(Clone)]
pub struct Combo {
    pub row: adw::ComboRow,
    quiet: Rc<Cell<bool>>,
}

impl Combo {
    pub fn new(title: &str, subtitle: &str, changed: impl Fn(u32) + 'static) -> Combo {
        let row = adw::ComboRow::builder()
            .title(title)
            .use_markup(false)
            .build();
        if !subtitle.is_empty() {
            row.set_subtitle(subtitle);
        }
        let quiet = Rc::new(Cell::new(false));
        let q = quiet.clone();
        row.connect_selected_notify(move |row| {
            if !q.get() && row.selected() != gtk::INVALID_LIST_POSITION {
                changed(row.selected());
            }
        });
        Combo { row, quiet }
    }

    /// Replaces the choices with `labels` and selects `selected`.
    pub fn set(&self, labels: &[String], selected: u32) {
        self.quiet.set(true);
        let refs: Vec<&str> = labels.iter().map(String::as_str).collect();
        self.row.set_model(Some(&gtk::StringList::new(&refs)));
        self.row.set_selected(selected);
        self.quiet.set(false);
    }
}

/// Rows added to a group that are replaced together.
pub struct Rows {
    pub group: adw::PreferencesGroup,
    rows: RefCell<Vec<gtk::Widget>>,
}

impl Rows {
    pub fn new(group: adw::PreferencesGroup) -> Rc<Rows> {
        Rc::new(Rows {
            group,
            rows: RefCell::new(Vec::new()),
        })
    }

    pub fn clear(&self) {
        for row in self.rows.borrow_mut().drain(..) {
            self.group.remove(&row);
        }
    }

    pub fn add(&self, row: &impl IsA<gtk::Widget>) {
        self.group.add(row);
        self.rows.borrow_mut().push(row.clone().upcast());
    }

    /// Adds a row that only says `text`, for an empty list.
    pub fn note(&self, text: &str) {
        let row = action_row(text, "");
        row.add_css_class("dim-label");
        self.add(&row);
    }
}

/// An icon from PNG bytes, or `fallback` from the icon theme when there are none.
pub fn png_image(png: &[u8], fallback: &str, size: i32) -> gtk::Image {
    let texture = (!png.is_empty())
        .then(|| gtk::gdk::Texture::from_bytes(&glib::Bytes::from(png)).ok())
        .flatten();
    let image = match texture {
        Some(texture) => gtk::Image::from_paintable(Some(&texture)),
        None => gtk::Image::from_icon_name(fallback),
    };
    image.set_pixel_size(size);
    image
}

/// Asks before doing something that cannot be undone, and runs `then` when the user agrees.
pub fn confirm(
    ctx: &Shared,
    heading: &str,
    body: &str,
    action: &str,
    destructive: bool,
    then: impl Fn() + 'static,
) {
    let dialog = adw::MessageDialog::new(Some(&ctx.window), Some(heading), Some(body));
    dialog.add_response("cancel", &tr("Cancel"));
    dialog.add_response("ok", action);
    if destructive {
        dialog.set_response_appearance("ok", adw::ResponseAppearance::Destructive);
    } else {
        dialog.set_response_appearance("ok", adw::ResponseAppearance::Suggested);
    }
    dialog.set_default_response(Some("cancel"));
    dialog.set_close_response("cancel");
    dialog.connect_response(None, move |_, response| {
        if response == "ok" {
            then();
        }
    });
    dialog.present();
}

/// Tells the user something that needs their attention.
pub fn tell(ctx: &Shared, heading: &str, body: &str) {
    let dialog = adw::MessageDialog::new(Some(&ctx.window), Some(heading), Some(body));
    dialog.add_response("ok", &tr("OK"));
    dialog.present();
}

/// Opens a folder in the file manager, or says why it cannot.
pub fn open_folder(ctx: &Shared, path: &std::path::Path) {
    if !path.is_dir() {
        tell(
            ctx,
            &tr("The folder could not be opened"),
            &crate::i18n::trf(
                "{0} does not exist yet, or you may not open it.",
                &[&path.display().to_string()],
            ),
        );
        return;
    }
    let uri = gio::File::for_path(path).uri();
    open_uri(ctx, &uri);
}

/// Opens `uri` in the program the desktop opens it with.
pub fn open_uri(ctx: &Shared, uri: &str) {
    if let Err(error) = gio::AppInfo::launch_default_for_uri(uri, None::<&gio::AppLaunchContext>) {
        tell(ctx, &tr("The folder could not be opened"), error.message());
    }
}

/// Starts a program of the desktop and does not wait for it. Returns why it could not.
pub fn start_program(command: &[&str]) -> Result<(), String> {
    let (program, args) = command.split_first().ok_or("no program")?; // not translated
    std::process::Command::new(program)
        .args(args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("{program}: {e}"))
}
