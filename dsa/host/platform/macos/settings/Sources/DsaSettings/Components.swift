import AppKit
import DsaSettingsCore
import SwiftUI

/// Has VoiceOver say something that appeared without taking the focus: a banner, a result, a
/// copy. An important text (something that went wrong) is read at once; any other waits for
/// what VoiceOver is saying.
@MainActor
enum Announce {
    static func say(_ words: String, important: Bool = false) {
        let words = words.trimmingCharacters(in: .whitespacesAndNewlines)
        guard !words.isEmpty, let app = NSApp else {
            return
        }
        let priority: NSAccessibilityPriorityLevel = important ? .high : .medium
        let element: NSResponder = app.mainWindow ?? app
        NSAccessibility.post(element: element, notification: .announcementRequested,
                             userInfo: [.announcement: words, .priority: priority.rawValue])
    }
}

/// A section's title, which VoiceOver lists as a heading of the second level, under the page's
/// title, and jumps to with its rotor.
struct SectionHeading: View {
    let text: String

    var body: some View {
        Text(text)
            .accessibilityAddTraits(.isHeader)
            .accessibilityHeading(.h2)
    }
}

extension Section where Parent == SectionHeading, Content: View, Footer == EmptyView {
    /// A section whose title is a heading: `Section(heading: T("FilesHeader.Text")) { ... }`.
    init(heading: String, @ViewBuilder content: () -> Content) {
        self.init(content: content, header: { SectionHeading(text: heading) })
    }
}

/// The page's title as VoiceOver's first heading. The window's title shows it on screen, as
/// System Settings shows a pane's; this element is only for VoiceOver, so the page reads as a
/// title and its sections under it, as the Windows settings app's pages do.
struct PageHeading: View {
    let text: String

    var body: some View {
        Color.clear
            .frame(maxWidth: .infinity)
            .frame(height: 1)
            .accessibilityElement()
            .accessibilityLabel(text)
            .accessibilityAddTraits(.isHeader)
            .accessibilityHeading(.h1)
    }
}

/// A message on a page: what went wrong, what was done, or what to know. VoiceOver says it
/// when it appears and when its words change, since it takes no focus. A banner that is always
/// on its page does not announce itself, and is read with the page instead.
struct Banner: View {
    enum Kind {
        case info, success, warning, error
    }

    let kind: Kind
    let title: String
    var message: String = ""
    var announces = true

    private var symbol: (String, Color) {
        switch kind {
        case .info: return ("info.circle.fill", .accentColor)
        case .success: return ("checkmark.circle.fill", .green)
        case .warning: return ("exclamationmark.triangle.fill", .orange)
        case .error: return ("xmark.octagon.fill", .red)
        }
    }

    var body: some View {
        HStack(alignment: .firstTextBaseline, spacing: 8) {
            Image(systemName: symbol.0).foregroundStyle(symbol.1)
                .accessibilityHidden(true)
            VStack(alignment: .leading, spacing: 2) {
                if !title.isEmpty {
                    Text(title).fontWeight(.semibold)
                }
                if !message.isEmpty {
                    Text(message).textSelection(.enabled).fixedSize(horizontal: false, vertical: true)
                }
            }
        }
        .accessibilityElement(children: .combine)
        .onAppear { tell() }
        .onChange(of: words) { _ in tell() }
    }

    private var words: String { AccessibleText.notice(appText, title: title, message: message) }

    private func tell() {
        if announces {
            Announce.say(words, important: kind == .error)
        }
    }
}

/// A banner as a row of its own in a grouped form, shown only when `shown`.
struct BannerSection: View {
    let shown: Bool
    let kind: Banner.Kind
    let title: String
    var message: String = ""

    var body: some View {
        if shown {
            Section {
                Banner(kind: kind, title: title, message: message)
            }
        }
    }
}

/// A row's title with the line that explains it below, as System Settings shows them.
struct RowLabel: View {
    let title: String
    var detail: String = ""

    var body: some View {
        VStack(alignment: .leading, spacing: 2) {
            Text(title)
            if !detail.isEmpty {
                Text(detail)
                    .font(.callout)
                    .foregroundStyle(.secondary)
                    .fixedSize(horizontal: false, vertical: true)
            }
        }
    }
}

/// One of a card's options, as System Settings lists them: its name, with the line that says
/// what it does under it, on the left, and its control on the right. The control keeps its own
/// VoiceOver name, which says whose option it is, and the line under the name is its hint, so
/// the text beside it is not read twice.
struct OptionRow<Content: View>: View {
    let title: String
    var detail: String = ""
    @ViewBuilder let content: Content

    var body: some View {
        LabeledContent {
            content
                .accessibilityHint(detail)
        } label: {
            RowLabel(title: title, detail: detail)
                .accessibilityHidden(true)
        }
    }
}

/// A short note in a page: a page's introduction, or what to know under a row. Every page
/// draws its notes with this, so they all look the same and wrap inside the form's width.
struct Note: View {
    let text: String
    var selectable = false

    var body: some View {
        if selectable {
            styled.textSelection(.enabled)
        } else {
            styled
        }
    }

    private var styled: some View {
        Text(text)
            .font(.callout)
            .foregroundStyle(.secondary)
            .fixedSize(horizontal: false, vertical: true)
    }
}

/// More about a row, folded away until the user opens it, so the row itself stays short.
struct LearnMore: View {
    let text: String

    var body: some View {
        DisclosureGroup {
            Note(text: text)
        } label: {
            Text(T("LearnMore.Label")).font(.callout)
        }
    }
}

/// A switch row. Only the user's change is sent; the switch shows what the service keeps
/// once it answers.
struct SwitchRow: View {
    let title: String
    var detail: String = ""
    let isOn: Bool
    var enabled = true
    let set: (Bool) async -> Void

    var body: some View {
        Toggle(isOn: Binding(get: { isOn }, set: { value in Task { await set(value) } })) {
            RowLabel(title: title, detail: detail)
        }
        .toggleStyle(.switch)
        .disabled(!enabled)
        // The title is the switch's name; the line under it is what VoiceOver adds after it.
        .accessibilityLabel(title)
        .accessibilityHint(detail)
    }
}

/// A row with a button at its end. The button's name says the row's title too: several rows
/// have a Remove or a Get button, and VoiceOver reading only "Remove" does not say what goes.
struct ButtonRow: View {
    let title: String
    var detail: String = ""
    let action: String
    var enabled = true
    var busy = false
    let run: () -> Void

    var body: some View {
        HStack {
            RowLabel(title: title, detail: detail)
            Spacer()
            if busy {
                ProgressView().controlSize(.small)
                    .accessibilityLabel(title)
            }
            Button(action, action: run).disabled(!enabled)
                .accessibilityLabel(AccessibleText.itemName(appText, label: action, item: title))
                .accessibilityHint(detail)
        }
    }
}

/// A row that opens something: a folder or Android's own settings. VoiceOver reads it as a
/// button named with its title, the line under the title as its hint.
struct LinkRow: View {
    let title: String
    var detail: String = ""
    var enabled = true
    let run: () -> Void

    var body: some View {
        Button(action: run) {
            HStack {
                RowLabel(title: title, detail: detail)
                Spacer()
                Image(systemName: "chevron.right").foregroundStyle(.tertiary)
                    .accessibilityHidden(true)
            }
            .contentShape(Rectangle())
        }
        .buttonStyle(.plain)
        .disabled(!enabled)
        .accessibilityLabel(title)
        .accessibilityHint(detail)
    }
}

/// A value row: a label and what the service reports.
struct ValueRow: View {
    let title: String
    var detail: String = ""
    let value: String

    var body: some View {
        LabeledContent {
            Text(value).textSelection(.enabled).multilineTextAlignment(.trailing)
        } label: {
            RowLabel(title: title, detail: detail)
        }
    }
}

/// A borderless copy button that puts `text` on the pasteboard and shows a check mark for a
/// moment, so the user sees that the copy happened; VoiceOver says it. `item` names what it
/// copies, for a page with several copy buttons ("Copy, dsa status").
struct CopyButton: View {
    let text: String
    var item = ""
    @State private var copied = false

    var body: some View {
        Button {
            NSPasteboard.general.clearContents()
            NSPasteboard.general.setString(text, forType: .string)
            copied = true
            Announce.say(T("A11y_Copied"))
            Task {
                try? await Task.sleep(nanoseconds: 1_500_000_000)
                copied = false
            }
        } label: {
            Image(systemName: copied ? "checkmark" : "doc.on.doc")
        }
        .buttonStyle(.borderless)
        .help(T("CopyButton.ToolTip"))
        .accessibilityLabel(AccessibleText.itemName(appText, label: T("CopyButton.ToolTip"), item: item))
    }
}

/// One command in monospaced, selectable text, with a button that copies exactly that command.
struct CommandRow: View {
    let command: String

    var body: some View {
        HStack {
            Text(command).font(.system(.body, design: .monospaced)).textSelection(.enabled)
            Spacer()
            CopyButton(text: command, item: command)
        }
    }
}

/// A picture from bytes Android sent (an app's icon, a widget's preview), or a stand-in.
struct PngImage: View {
    let data: Data?
    var size: CGFloat = 32
    var placeholder = "app.dashed"

    var body: some View {
        Group {
            if let data, let image = NSImage(data: data) {
                Image(nsImage: image).resizable().interpolation(.high).aspectRatio(contentMode: .fit)
            } else {
                Image(systemName: placeholder).resizable().aspectRatio(contentMode: .fit).foregroundStyle(.secondary).padding(4)
            }
        }
        .frame(width: size, height: size)
    }
}

extension View {
    /// The grouped form every page is, as System Settings draws its panes.
    func settingsPage() -> some View {
        formStyle(.grouped)
    }
}
