namespace Dsa.ApkInstaller;

/// <summary>
/// What the window does for a screen reader and the keyboard, apart from the window so that it
/// can be checked without one: the words said when an install ends, and the control a page's
/// keyboard focus starts on.
/// </summary>
internal static class ScreenReader
{
    /// <summary>The words said when the window shows how the install went: its title, then each
    /// of its lines, each one a sentence. Empty lines are left out.</summary>
    internal static string Announcement(string title, params string?[] lines)
    {
        var words = new System.Text.StringBuilder(Sentence(title));
        foreach (var line in lines)
        {
            var sentence = Sentence(line);
            if (sentence.Length == 0)
            {
                continue;
            }
            if (words.Length > 0)
            {
                words.Append(' ');
            }
            words.Append(sentence);
        }
        return words.ToString();
    }

    /// <summary>The control a page's keyboard focus starts on: its main button (the one Enter
    /// presses), else its first button. Only a control the user can reach counts, so a hidden or
    /// disabled one is passed over. Null when the page has none.</summary>
    internal static T? FirstControl<T>(System.Collections.Generic.IEnumerable<T> buttons,
        System.Func<T, bool> usable, System.Func<T, bool> isMain)
        where T : class
    {
        T? first = null;
        foreach (var button in buttons)
        {
            if (!usable(button))
            {
                continue;
            }
            if (isMain(button))
            {
                return button;
            }
            first ??= button;
        }
        return first;
    }

    /// <summary>The text as a sentence: trimmed, and ending in a full stop unless it ends in one
    /// already, or in a question mark, an exclamation mark or an ellipsis.</summary>
    private static string Sentence(string? text)
    {
        var trimmed = (text ?? string.Empty).Trim();
        if (trimmed.Length == 0)
        {
            return trimmed;
        }
        var last = trimmed[^1];
        return last is '.' or '?' or '!' or '…' ? trimmed : trimmed + ".";
    }
}
