namespace Dsa.Settings.Core;

/// <summary>The four font lists the settings app and the APK Installer draw text with: the
/// text and display faces, and their strong (Semibold) forms.</summary>
public sealed record UiFontSet(string Text, string Display, string StrongText, string StrongDisplay);

/// <summary>
/// The fonts for the user's language. The apps' own lists (Segoe UI Variable, Segoe UI, then
/// Malgun Gothic) draw Latin, Cyrillic, Vietnamese and Korean text. Japanese and Chinese need
/// their own UI font before Malgun Gothic, which has kana and many Han characters in their
/// Korean forms and lacks most Simplified ones, so their text would come out in Korean shapes
/// or in a mix of fonts.
///
/// Malgun Gothic, Microsoft YaHei UI and Microsoft JhengHei UI have no Semibold. WPF applies a
/// Semibold request to the whole list, even when it comes from a family name in it, so those
/// fonts draw it in their Bold; in Korean and Chinese the strong fonts are the regular ones,
/// as Windows' Settings app shows its titles. Yu Gothic UI has a Semibold, so Japanese keeps
/// its strong fonts.
/// </summary>
public static class UiFonts
{
    private static UiFontSet Lists(string font, string strong) => new(
        $"Segoe UI Variable Text, Segoe UI, {font}, Malgun Gothic",
        $"Segoe UI Variable Display, Segoe UI, {font}, Malgun Gothic",
        $"Segoe UI Variable Text Semibold, Segoe UI Semibold, {strong}, Malgun Gothic",
        $"Segoe UI Variable Display Semibold, Segoe UI Semibold, {strong}, Malgun Gothic");

    /// <summary>The lists that replace the apps' own for <paramref name="language"/>, or null
    /// when the apps' own lists draw it.</summary>
    public static UiFontSet? For(UiLanguage language)
    {
        if (language == UiLanguage.Japanese)
        {
            return Lists("Yu Gothic UI", "Yu Gothic UI Semibold");
        }
        if (language == UiLanguage.ChineseSimplified)
        {
            return Lists("Microsoft YaHei UI", "Microsoft YaHei UI");
        }
        if (language == UiLanguage.ChineseTraditional)
        {
            return Lists("Microsoft JhengHei UI", "Microsoft JhengHei UI");
        }
        return null;
    }

    /// <summary>Whether the language's font has no Semibold, so the strong fonts are the
    /// regular ones: Korean (Malgun Gothic) and Chinese (Microsoft YaHei UI and Microsoft
    /// JhengHei UI).</summary>
    public static bool HasNoSemibold(UiLanguage language) =>
        language == UiLanguage.Korean
        || language == UiLanguage.ChineseSimplified
        || language == UiLanguage.ChineseTraditional;
}
