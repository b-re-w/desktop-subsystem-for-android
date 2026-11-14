import Foundation

/// The words VoiceOver says for the settings app's controls, where the words on screen are not
/// enough: a button repeated in every row of a list, a banner that appears, a log line, a log
/// priority letter. The same texts as the Windows settings app's, in both languages.
public enum AccessibleText {
    /// The log priorities, least first, as logcat's letters.
    private static let priorities: Set<String> = ["V", "D", "I", "W", "E", "F"]

    /// The name of a control in a list row: what it does (the text `key`) and which row it
    /// belongs to, such as "Open, Notes". Every row has its own Open and Stop buttons, and a
    /// name of "Open" alone does not say whose.
    public static func itemName(_ text: TextSource, _ key: String, _ item: String) -> String {
        itemName(text, label: text.get(key), item: item)
    }

    /// The same, for a control whose words are already in the user's language.
    public static func itemName(_ text: TextSource, label: String, item: String) -> String {
        item.isEmpty || item == label ? label : Format.fill(text.get("A11y_ItemName"), [label, item])
    }

    /// What a banner says, as one sentence to read out: its title, then its message. Either
    /// can be empty.
    public static func notice(_ text: TextSource, title: String, message: String) -> String {
        let title = title.trimmingCharacters(in: .whitespacesAndNewlines)
        let message = message.trimmingCharacters(in: .whitespacesAndNewlines)
        if title.isEmpty || message.isEmpty {
            return title.isEmpty ? message : title
        }
        return Format.fill(text.get("A11y_Notice"), [title, message])
    }

    /// A log priority as a word in the user's language, such as "Warning" for W.
    public static func logLevel(_ text: TextSource, _ priority: String) -> String {
        priorities.contains(priority) ? text.get("Logs_Priority_\(priority)") : priority
    }

    /// A log line as VoiceOver reads its row: the time, the priority as a word, the tag and the
    /// message. The row shows them in columns, and its letters and numbers read as noise. A
    /// record that stands for dropped records says how many.
    public static func logLine(_ text: TextSource, _ line: LogLine) -> String {
        if line.dropped > 0 {
            return text.format("A11y_LogDropped", line.dropped)
        }
        return Format.fill(text.get("A11y_LogLine"), [line.time, logLevel(text, line.priority), line.tag, line.message])
    }
}
