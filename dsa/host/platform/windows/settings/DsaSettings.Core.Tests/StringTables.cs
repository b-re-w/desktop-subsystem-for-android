using System.Text.RegularExpressions;
using System.Xml.Linq;
using Xunit;

namespace Dsa.Settings.Core.Tests;

/// <summary>The settings app's string tables, one for each interface language: Strings.resx
/// (English) and Strings.&lt;tag&gt;.resx beside it. The APK Installer embeds the same files.</summary>
public static class StringTables
{
    /// <summary>The file name of a language's table.</summary>
    public static string FileName(UiLanguage language) =>
        language == UiLanguage.English ? "Strings.resx" : $"Strings.{language.Tag}.resx";

    /// <summary>Every table, English first, for a theory.</summary>
    public static IEnumerable<object[]> Files() => UiLanguage.All.Select(l => new object[] { FileName(l) });

    /// <summary>Every table but English, for a theory that compares a translation with it.</summary>
    public static IEnumerable<object[]> Translations() =>
        UiLanguage.All.Where(l => l != UiLanguage.English).Select(l => new object[] { FileName(l) });

    /// <summary>Where the table is, or null when it is not there.</summary>
    public static string? Path(string file)
    {
        // The tests run from dsa/target/settings/..., outside the source tree, so each folder
        // above is tried both as the settings folder and as the repository's dsa folder.
        string[] relative = { System.IO.Path.Combine("DsaSettings", file),
            System.IO.Path.Combine("host", "platform", "windows", "settings", "DsaSettings", file) };
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var found = relative.Select(r => System.IO.Path.Combine(dir.FullName, r)).FirstOrDefault(File.Exists);
            if (found is not null)
            {
                return found;
            }
        }
        return null;
    }

    /// <summary>The table's texts by key.</summary>
    public static Dictionary<string, string> Load(string file)
    {
        var path = Path(file);
        Assert.True(path is not null, file + " was not found above " + AppContext.BaseDirectory);
        return XDocument.Load(path!).Descendants("data")
            .ToDictionary(d => (string)d.Attribute("name")!, d => d.Element("value")?.Value ?? "");
    }

    /// <summary>The format holes of a text, by number and as often as each comes:
    /// <c>{1} of {0}, {0}</c> is 0, 0, 1. A hole with a format (<c>{0:N0}</c>) counts as its
    /// number.</summary>
    public static IReadOnlyList<int> Holes(string text) =>
        Regex.Matches(text, @"(?<!\{)\{(\d+)(?:[,:][^}]*)?\}")
            .Select(m => int.Parse(m.Groups[1].Value, System.Globalization.CultureInfo.InvariantCulture))
            .Order()
            .ToList();
}

/// <summary>Texts from one of the settings app's string tables, as the apps read them, so a test
/// sees the words a user reads. A key the table lacks comes back as the key.</summary>
internal sealed class TableText : ITextSource
{
    private readonly Dictionary<string, string> _strings;

    private TableText(Dictionary<string, string> strings)
    {
        _strings = strings;
    }

    public static TableText Of(string file) => new(StringTables.Load(file));

    public string Get(string key) => _strings.TryGetValue(key, out var value) ? value : key;
}
