#nullable enable

namespace Dsa.Setup
{
    /// <summary>
    /// What the setup window does for a screen reader and the keyboard, apart from the window so
    /// that it can be checked without one: the words said when setup ends, and the control a
    /// screen's keyboard focus starts on.
    /// </summary>
    internal static class ScreenReader
    {
        /// <summary>
        /// The words said when a screen shows how setup went: its title, then each of its lines,
        /// each one a sentence. Empty lines are left out.
        /// </summary>
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

        /// <summary>
        /// The control a screen's keyboard focus starts on: the first field the screen asks the
        /// user to fill in, else its main button (the one Enter presses), else its first button.
        /// Only a control the user can reach counts, so a hidden or disabled one is passed over.
        /// Null when the screen has none.
        /// </summary>
        internal static T? FirstControl<T>(
            System.Collections.Generic.IEnumerable<T> fields,
            System.Collections.Generic.IEnumerable<T> buttons,
            System.Func<T, bool> usable,
            System.Func<T, bool> isMain)
            where T : class
        {
            foreach (var field in fields)
            {
                if (usable(field))
                {
                    return field;
                }
            }

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

        /// <summary>The text as a sentence: trimmed, and ending in a full stop unless it ends in
        /// one already, or in a question mark, an exclamation mark or an ellipsis.</summary>
        private static string Sentence(string? text)
        {
            var trimmed = (text ?? string.Empty).Trim();
            if (trimmed.Length == 0)
            {
                return trimmed;
            }

            char last = trimmed[trimmed.Length - 1];
            return last == '.' || last == '?' || last == '!' || last == '…' ? trimmed : trimmed + ".";
        }
    }
}
