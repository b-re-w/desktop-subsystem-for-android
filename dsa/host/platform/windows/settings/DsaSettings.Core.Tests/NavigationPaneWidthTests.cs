using System.Text.RegularExpressions;
using System.Xml.Linq;
using Xunit;

namespace Dsa.Settings.Core.Tests;

/// <summary>The navigation pane is as wide as its longest label needs, as Task Manager's is,
/// and no wider. An item is the 40 px icon column (the selection bar sits inside it), 4 px
/// before the label, the label at 14 px, and a 12 px gap after it; the pane rounds that up to
/// a multiple of 4. The pane's 4 px margins lie outside its width.
///
/// The test project does not reference WPF, so text is measured with a table of advance
/// widths at 14 px instead of FormattedText. Each Latin width is Segoe UI's advance rounded up
/// to the next half pixel; every Hangul syllable is counted as a full 14 px em, which is more
/// than Malgun Gothic draws. The table only overestimates, so a label it says fits does fit.
/// A label with a character the table lacks fails the test, so a new label cannot slip past.</summary>
public class NavigationPaneWidthTests
{
    private const double IconColumn = 40, LabelGap = 4, TrailingGap = 12;

    private const double OpenPaneLength = 236;

    private static readonly Dictionary<char, double> Latin = new()
    {
        [' '] = 4, ['&'] = 10.5, ['\''] = 3.5, ['-'] = 5, ['.'] = 3.5,
        ['A'] = 9.5, ['B'] = 9, ['C'] = 9.5, ['D'] = 10.5, ['E'] = 7.5, ['F'] = 7, ['G'] = 10.5,
        ['H'] = 11, ['I'] = 4, ['J'] = 5, ['K'] = 9, ['L'] = 7, ['M'] = 13, ['N'] = 11,
        ['O'] = 11.5, ['P'] = 8.5, ['Q'] = 11.5, ['R'] = 9, ['S'] = 8, ['T'] = 8, ['U'] = 10.5,
        ['V'] = 9.5, ['W'] = 14, ['X'] = 9, ['Y'] = 8.5, ['Z'] = 8.5,
        ['a'] = 7.5, ['b'] = 8.5, ['c'] = 7, ['d'] = 8.5, ['e'] = 7.5, ['f'] = 5, ['g'] = 8.5,
        ['h'] = 8.5, ['i'] = 3.5, ['j'] = 3.5, ['k'] = 7, ['l'] = 3.5, ['m'] = 12.5, ['n'] = 8.5,
        ['o'] = 8.5, ['p'] = 8.5, ['q'] = 8.5, ['r'] = 5, ['s'] = 6, ['t'] = 5, ['u'] = 8.5,
        ['v'] = 7, ['w'] = 11, ['x'] = 7, ['y'] = 7, ['z'] = 6.5, ['ß'] = 8.5, ['ı'] = 3.5,
        ['đ'] = 8.5, ['Đ'] = 11,
    };

    /// <summary>The fourteen string tables the pane's labels come from.</summary>
    private static readonly string[] Tables =
    {
        "Strings.resx", "Strings.ko.resx", "Strings.ja.resx", "Strings.zh-CN.resx", "Strings.zh-TW.resx",
        "Strings.es.resx", "Strings.fr.resx", "Strings.de.resx", "Strings.pt-BR.resx", "Strings.ru.resx",
        "Strings.it.resx", "Strings.vi.resx", "Strings.id.resx", "Strings.tr.resx",
    };

    private static string SettingsFile(string file)
    {
        string[] relative = { Path.Combine("DsaSettings", file),
            Path.Combine("host", "platform", "windows", "settings", "DsaSettings", file) };
        string? found = null;
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null && found is null; dir = dir.Parent)
        {
            found = relative.Select(r => Path.Combine(dir.FullName, r)).FirstOrDefault(File.Exists);
        }
        Assert.True(found is not null, file + " was not found above " + AppContext.BaseDirectory);
        return found!;
    }

    private static double Width(string label) => label.Sum(c =>
    {
        // Hangul, kana and Han: a full 14 px em, more than their fonts draw.
        if ((c >= '가' && c <= '힣') || (c >= '぀' && c <= 'ヿ') || (c >= '一' && c <= '鿿'))
        {
            return 14.0;
        }
        // Cyrillic: as wide as the widest Latin letters of its case.
        if (c >= 'Ѐ' && c <= 'ӿ')
        {
            return char.IsUpper(c) ? 13.0 : 8.5;
        }
        if (Latin.TryGetValue(c, out var w))
        {
            return w;
        }
        // A Latin letter with marks (é, ộ, ç, ğ) is as wide as its base letter.
        var basic = c.ToString().Normalize(System.Text.NormalizationForm.FormD)[0];
        Assert.True(Latin.TryGetValue(basic, out w), $"no width for '{c}' in '{label}'; add it to the table");
        return w;
    });

    private static IEnumerable<string> Labels()
    {
        var xaml = File.ReadAllText(SettingsFile("MainWindow.xaml"));
        var keys = Regex.Matches(xaml, @"<l:PaneItem Content=""\{l:Text (Nav\w+\.Content)\}""")
            .Select(m => m.Groups[1].Value).ToList();
        Assert.NotEmpty(keys);
        foreach (var resx in Tables)
        {
            var values = XDocument.Load(SettingsFile(resx)).Root!.Elements("data")
                .ToDictionary(d => (string)d.Attribute("name")!, d => (string)d.Element("value")!);
            foreach (var key in keys)
            {
                Assert.True(values.ContainsKey(key), $"{resx} has no {key}");
                yield return values[key];
            }
        }
    }

    [Fact]
    public void Fr17_the_navigation_pane_is_as_wide_as_its_longest_label_needs()
    {
        var xaml = File.ReadAllText(SettingsFile("MainWindow.xaml"));
        Assert.Contains($"OpenPaneLength=\"{OpenPaneLength}\"", xaml);

        // English and Korean set the pane's width; every other language has to fit in it
        // (the next test), so a long translation is shortened rather than widening the pane.
        var longest = Labels().Take(2 * Labels().Count() / Tables.Length).Max(Width);
        var needed = Math.Ceiling((IconColumn + LabelGap + longest + TrailingGap) / 4) * 4;
        Assert.Equal(OpenPaneLength, Math.Max(200, needed));
    }

    [Fact]
    public void Fr17_no_navigation_label_is_truncated_in_the_open_pane()
    {
        foreach (var label in Labels())
        {
            Assert.True(IconColumn + LabelGap + Width(label) + TrailingGap <= OpenPaneLength,
                $"'{label}' is {Width(label)} px wide and does not fit a {OpenPaneLength} px pane");
        }
    }
}
