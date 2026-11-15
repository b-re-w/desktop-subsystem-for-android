using System.Globalization;

namespace Dsa.Settings.Core;

/// <summary>The words a screen reader says for the settings app's controls, where the words on
/// screen are not enough: a button repeated in every row of a list, a notice bar that opens, a
/// log line, a log priority letter.</summary>
public static class AccessibleText
{
    /// <summary>The format of the name of a control in a list row: what the control does and
    /// which row it belongs to, such as "Open, Notes". Every row has its own Open and Stop
    /// buttons, and a name of "Open" alone does not say whose. The result is a format string
    /// with the row's name as <c>{0}</c>, so a binding can fill it in.</summary>
    public static string ItemNameFormat(ITextSource text, string key) =>
        string.Format(CultureInfo.InvariantCulture, text.Get("A11y_ItemName"), Escape(text.Get(key)), "{0}");

    /// <summary>The name of a control in the row of <paramref name="item"/>.</summary>
    public static string ItemName(ITextSource text, string key, string item) =>
        string.Format(CultureInfo.CurrentCulture, ItemNameFormat(text, key), item);

    /// <summary>What a notice bar says, as one sentence to read out: its title, then its
    /// message. Either can be empty.</summary>
    public static string Notice(ITextSource text, string? title, string? message)
    {
        title = title?.Trim() ?? "";
        message = message?.Trim() ?? "";
        if (title.Length == 0 || message.Length == 0)
        {
            return title.Length > 0 ? title : message;
        }
        return string.Format(CultureInfo.CurrentCulture, text.Get("A11y_Notice"), title, message);
    }

    /// <summary>A log priority as a word in the user's language, such as "Warning" for W.</summary>
    public static string LogLevel(ITextSource text, string priority) =>
        LogViewModel.Priorities.Contains(priority) ? text.Get("Logs_Priority_" + priority) : priority;

    /// <summary>Doubles the braces in a text that goes into a format string, so a label such as
    /// "{beta}" stays as it is.</summary>
    private static string Escape(string value) => value.Replace("{", "{{").Replace("}", "}}");
}
