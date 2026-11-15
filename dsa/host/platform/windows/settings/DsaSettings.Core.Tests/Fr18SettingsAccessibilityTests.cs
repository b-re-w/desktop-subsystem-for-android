using System.Globalization;
using System.Text.RegularExpressions;
using System.Xml;
using System.Xml.Linq;
using Xunit;

namespace Dsa.Settings.Core.Tests;

/// <summary>The settings app works with a screen reader: every control has a name in the user's
/// language, a control repeated in every row of a list says whose it is, page titles and section
/// headers are headings, a notice bar is read out when it opens or changes, the administrator
/// shield is not only a tool tip, and a log line reads as a sentence. The XAML is read as files,
/// every page and window of the app, so a page added later is held to the same rules.</summary>
public class Fr18SettingsAccessibilityTests
{
    private static readonly XNamespace Wpf = "http://schemas.microsoft.com/winfx/2006/xaml/presentation";
    private static readonly XNamespace Ui = "http://schemas.lepo.co/wpfui/2022/xaml";
    private static readonly XNamespace X = "http://schemas.microsoft.com/winfx/2006/xaml";
    private static readonly XNamespace Local = "clr-namespace:Dsa.Settings";
    private static readonly XName AutomationName = XName.Get("AutomationProperties.Name");
    private static readonly XName LabeledBy = XName.Get("AutomationProperties.LabeledBy");
    private static readonly XName HelpText = XName.Get("AutomationProperties.HelpText");
    private const string Shield = "{l:Text AdminShield.ToolTip}";

    private static string SettingsFolder()
    {
        // The tests run from dsa/target/settings/..., outside the source tree, so each folder
        // above is tried both as the settings folder and as the repository's dsa folder.
        string[] relative = { "DsaSettings", Path.Combine("host", "platform", "windows", "settings", "DsaSettings") };
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var found = relative.Select(r => Path.Combine(dir.FullName, r))
                .FirstOrDefault(p => File.Exists(Path.Combine(p, "DsaSettings.csproj")));
            if (found is not null)
            {
                return found;
            }
        }
        throw new DirectoryNotFoundException("the settings app's source was not found above " + AppContext.BaseDirectory);
    }

    private static List<(string File, XDocument Xaml)> AllXaml()
    {
        var app = SettingsFolder();
        return Directory.EnumerateFiles(app, "*.xaml", SearchOption.AllDirectories)
            .Where(f => !f.Split(Path.DirectorySeparatorChar).Any(p => p is "bin" or "obj"))
            .Select(f => (Path.GetRelativePath(app, f), XDocument.Load(f, LoadOptions.SetLineInfo)))
            .ToList();
    }

    private static XDocument Xaml(string relative) =>
        XDocument.Load(Path.Combine(SettingsFolder(), relative), LoadOptions.SetLineInfo);

    private static string Code(string relative) => File.ReadAllText(Path.Combine(SettingsFolder(), relative));

    private static string Where(string file, XElement e) =>
        $"{file}:{((IXmlLineInfo)e).LineNumber} <{e.Name.LocalName}>";

    private static bool Named(XElement e) => e.Attribute(AutomationName) is not null || e.Attribute(LabeledBy) is not null;

    private static List<string> Unnamed(params XName[] types) => AllXaml()
        .SelectMany(x => x.Xaml.Descendants().Where(e => types.Contains(e.Name) && !Named(e)).Select(e => Where(x.File, e)))
        .ToList();

    private static Dictionary<string, string> Strings(string file)
    {
        var path = Path.Combine(SettingsFolder(), file);
        return XDocument.Load(path).Descendants("data")
            .ToDictionary(d => (string)d.Attribute("name")!, d => d.Element("value")?.Value ?? "");
    }

    /// <summary>The texts of one of the app's languages, as the app reads them.</summary>
    private sealed class ResxText : ITextSource
    {
        private readonly Dictionary<string, string> _strings;

        public ResxText(string file)
        {
            _strings = Strings(file);
        }

        public string Get(string key) => _strings.TryGetValue(key, out var value) ? value : key;
    }

    private static readonly XName[] Buttons =
    {
        Wpf + "Button", Ui + "Button", Wpf + "ToggleButton", Wpf + "RepeatButton", Ui + "HyperlinkButton",
    };

    private static readonly XName[] RowControls =
    {
        Wpf + "Button", Ui + "Button", Ui + "HyperlinkButton", Ui + "ToggleSwitch", Wpf + "ComboBox", Wpf + "CheckBox",
    };

    [Fact]
    public void Fr18_every_toggle_has_an_automation_name()
    {
        var offenders = Unnamed(Ui + "ToggleSwitch");
        Assert.True(offenders.Count == 0,
            "these switches have no AutomationProperties.Name, so a screen reader reads only \"On\" or \"Off\": "
            + string.Join("; ", offenders));
    }

    [Fact]
    public void Fr18_every_combo_and_search_box_has_an_automation_name()
    {
        var offenders = Unnamed(Wpf + "ComboBox", Ui + "AutoSuggestBox", Ui + "TextBox", Ui + "NumberBox",
            Ui + "PasswordBox", Wpf + "PasswordBox", Wpf + "TextBox");
        Assert.True(offenders.Count == 0,
            "these boxes have no AutomationProperties.Name; a placeholder or a label beside them is not their name: "
            + string.Join("; ", offenders));
    }

    [Fact]
    public void Fr18_every_progress_indicator_has_a_name()
    {
        var offenders = Unnamed(Wpf + "ProgressBar", Ui + "ProgressRing", Local + "ProgressStrip");
        Assert.True(offenders.Count == 0,
            "these progress indicators have no name, so a screen reader cannot say what is in progress: "
            + string.Join("; ", offenders));
    }

    /// <summary>Whether a button shows text of its own, which is its name.</summary>
    private static bool HasText(XElement button) =>
        button.Attribute("Content") is not null
        || button.Nodes().OfType<XText>().Any(t => t.Value.Trim().Length > 0)
        || button.Elements().Where(c => !c.Name.LocalName.Contains('.'))
            .DescendantsAndSelf().Any(d => d.Name == Wpf + "TextBlock" || d.Name == Wpf + "AccessText");

    [Fact]
    public void Fr18_icon_only_buttons_have_names()
    {
        var offenders = AllXaml()
            .SelectMany(x => x.Xaml.Descendants()
                .Where(e => Buttons.Contains(e.Name) && !HasText(e) && !Named(e))
                .Select(e => Where(x.File, e)))
            .ToList();
        Assert.True(offenders.Count == 0,
            "these buttons show only an icon and have no AutomationProperties.Name (a tool tip is not a name): "
            + string.Join("; ", offenders));
    }

    [Fact]
    public void Fr18_card_actions_have_names()
    {
        var offenders = Unnamed(Ui + "CardAction");
        Assert.True(offenders.Count == 0,
            "these clickable cards hold their title in a panel, which is not their name: " + string.Join("; ", offenders));
    }

    private static bool NamesItsItem(XElement e) =>
        ((string?)e.Attribute(AutomationName) ?? "") is var name
        && (name.StartsWith("{l:ItemName ", StringComparison.Ordinal) || name.StartsWith("{Binding", StringComparison.Ordinal));

    [Fact]
    public void Fr18_repeated_item_buttons_name_their_item()
    {
        var offenders = new List<string>();
        foreach (var (file, xaml) in AllXaml())
        {
            // A control in a list row's template is repeated in every row.
            offenders.AddRange(xaml.Descendants(Wpf + "DataTemplate")
                .SelectMany(t => t.Descendants().Where(e => RowControls.Contains(e.Name) && !NamesItsItem(e)))
                .Select(e => Where(file, e) + " in a list row"));

            // Buttons on one page that say the same thing, or that run one handler for different
            // items (their Tag), such as the copy button of each command.
            var outside = xaml.Descendants()
                .Where(e => Buttons.Contains(e.Name) && !e.Ancestors(Wpf + "DataTemplate").Any()
                    && !e.Ancestors(Wpf + "ControlTemplate").Any())
                .ToList();
            var groups = outside.Where(e => e.Attribute("Click") is not null && e.Attribute("Tag") is not null)
                .GroupBy(e => "handler " + (string)e.Attribute("Click")!)
                .Concat(outside.Where(e => e.Attribute("Content") is not null).GroupBy(e => "label " + (string)e.Attribute("Content")!));
            foreach (var group in groups.Where(g => g.Count() > 1))
            {
                var names = group.Select(e => (string?)e.Attribute(AutomationName)).ToList();
                var told = group.All(NamesItsItem) || (names.All(n => n is not null) && names.Distinct().Count() == names.Count);
                if (!told)
                {
                    offenders.AddRange(group.Select(e => Where(file, e) + " shares its " + group.Key));
                }
            }
        }
        Assert.True(offenders.Count == 0,
            "these repeated controls do not say whose they are: " + string.Join("; ", offenders));
    }

    [Fact]
    public void Fr18_page_titles_and_section_headers_have_heading_levels()
    {
        var app = Xaml("App.xaml");
        string? Level(string key) => app.Descendants(Wpf + "Style")
            .Single(s => (string?)s.Attribute(X + "Key") == key)
            .Elements(Wpf + "Setter")
            .FirstOrDefault(s => (string?)s.Attribute("Property") == "AutomationProperties.HeadingLevel")
            ?.Attribute("Value")?.Value;
        Assert.Equal("Level1", Level("PageTitle"));
        Assert.Equal("Level2", Level("SectionHeader"));

        // Every page has its title in the heading style, and no page header overrides it.
        var pages = AllXaml().Where(x => x.Xaml.Root?.Name == Wpf + "Page").ToList();
        Assert.NotEmpty(pages);
        foreach (var (file, xaml) in pages)
        {
            Assert.True(xaml.Descendants(Wpf + "TextBlock").Any(t => (string?)t.Attribute("Style") == "{StaticResource PageTitle}"),
                file + " has no TextBlock in the PageTitle style");
            Assert.DoesNotContain(xaml.Descendants(),
                e => e.Attribute(XName.Get("AutomationProperties.HeadingLevel")) is { Value: "None" });
        }
    }

    [Fact]
    public void Fr18_every_info_bar_announces()
    {
        var app = Xaml("App.xaml");
        Assert.Equal("clr-namespace:Dsa.Settings", (string?)app.Root!.Attribute(XNamespace.Xmlns + "l"));
        var style = app.Descendants(Wpf + "Style")
            .Single(s => (string?)s.Attribute("TargetType") == "ui:InfoBar" && s.Attribute(X + "Key") is null);
        Assert.Contains(style.Elements(Wpf + "Setter"),
            s => (string?)s.Attribute("Property") == "l:Announce.Changes" && (string?)s.Attribute("Value") == "True");

        // A bar with a style of its own would lose the announcement the implicit style gives it.
        var restyled = AllXaml()
            .SelectMany(x => x.Xaml.Descendants(Ui + "InfoBar").Where(b => b.Attribute("Style") is not null)
                .Select(b => Where(x.File, b)))
            .ToList();
        Assert.True(restyled.Count == 0, "these notice bars replace the style that announces them: " + string.Join("; ", restyled));

        var announce = Code("Announce.cs");
        Assert.Contains("RaiseNotificationEvent", announce);
        foreach (var property in new[] { "InfoBar.IsOpenProperty", "InfoBar.TitleProperty", "InfoBar.MessageProperty" })
        {
            Assert.Contains(property, announce);
        }
    }

    [Fact]
    public void Fr18_shield_info_is_not_tooltip_only()
    {
        var offenders = new List<string>();
        var shields = 0;
        foreach (var (file, xaml) in AllXaml())
        {
            foreach (var shield in xaml.Descendants().Where(e => (string?)e.Attribute("ToolTip") == Shield))
            {
                shields++;
                if ((string?)shield.Attribute(AutomationName) != Shield && (string?)shield.Attribute(HelpText) != Shield)
                {
                    offenders.Add(Where(file, shield) + " says it only in its tool tip");
                }
                // The control a screen reader lands on says it too: the icon itself takes no focus.
                var card = shield.Ancestors(Ui + "CardControl").FirstOrDefault();
                if (card is null || !card.Descendants().Any(e => RowControls.Contains(e.Name)
                        && (string?)e.Attribute(HelpText) == Shield))
                {
                    offenders.Add(Where(file, shield) + ": no control in its card has the help text");
                }
            }
        }
        Assert.True(shields > 0, "no administrator shield was found; the test reads the wrong files");
        Assert.True(offenders.Count == 0, string.Join("; ", offenders));
    }

    [Theory]
    [InlineData("Strings.resx", "2026-11-04 10:00:00.000, Warning, ActivityManager: Start proc", "3 records dropped")]
    [InlineData("Strings.ko.resx", "2026-11-04 10:00:00.000, 경고, ActivityManager: Start proc", "기록 3개가 빠졌습니다")]
    public void Fr18_log_line_reads_as_text(string file, string spoken, string dropped)
    {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        var text = new ResxText(file);
        var line = LogLine.Parse(
            "{\"time\":\"2026-11-04 10:00:00.000\",\"priority\":\"W\",\"pid\":12,\"tid\":13,\"tag\":\"ActivityManager\",\"message\":\"Start proc\"}")!;
        Assert.Equal(spoken, line.Spoken(text));
        var gap = LogLine.Parse("{\"time\":\"2026-11-04 10:00:00.000\",\"priority\":\"W\",\"dropped\":3}")!;
        Assert.Equal(dropped, gap.Spoken(text));

        // A row whose words are not set reads the line as the log prints it, not the record's fields.
        Assert.Equal(line.Text, line.ToString());
        Assert.DoesNotContain("LogLine {", line.ToString());

        // The log page sets each row's name to those words.
        var setter = Xaml(Path.Combine("Pages", "LogsPage.xaml")).Descendants(Wpf + "ListBox.ItemContainerStyle")
            .Descendants(Wpf + "Setter").Single(s => (string?)s.Attribute("Property") == "AutomationProperties.Name");
        Assert.Contains("LogLineName", (string?)setter.Attribute("Value"));
    }

    [Theory]
    [MemberData(nameof(StringTables.Files), MemberType = typeof(StringTables))]
    public void Fr18_a_row_control_name_says_what_it_does_and_whose_it_is(string file)
    {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        var text = new ResxText(file);
        var open = text.Get("AppsOpenButton.Content");
        var name = AccessibleText.ItemName(text, "AppsOpenButton.Content", "Notes");
        Assert.Contains(open, name);
        Assert.Contains("Notes", name);
        Assert.NotEqual(open, name);

        // A label with braces stays as it is.
        Assert.Equal("Open {beta}, Notes", string.Format(CultureInfo.InvariantCulture,
            AccessibleText.ItemNameFormat(new FixedText("{0}, {1}", "Open {beta}"), "Open"), "Notes"));

        // A notice bar reads its title and its message; either alone reads as it is.
        Assert.Equal("Problem", AccessibleText.Notice(text, "Problem", ""));
        Assert.Equal("Gone", AccessibleText.Notice(text, null, "Gone"));
        var both = AccessibleText.Notice(text, "Problem", "Gone");
        Assert.True(both.IndexOf("Problem", StringComparison.Ordinal) < both.IndexOf("Gone", StringComparison.Ordinal), both);
    }

    private sealed class FixedText : ITextSource
    {
        private readonly string _format;
        private readonly string _label;

        public FixedText(string format, string label)
        {
            _format = format;
            _label = label;
        }

        public string Get(string key) => key == "A11y_ItemName" ? _format : _label;
    }

    [Fact]
    public void Fr18_new_strings_exist_in_korean_and_english()
    {
        var english = Strings("Strings.resx");
        var korean = Strings("Strings.ko.resx");
        var keys = new HashSet<string>(StringComparer.Ordinal);
        var used = new Regex(@"\{l:(?:Text|ItemName) ([A-Za-z0-9_.]+)|\bItem=([A-Za-z0-9_.]+)");
        foreach (var (_, xaml) in AllXaml())
        {
            foreach (var attribute in xaml.Descendants().Attributes())
            {
                foreach (Match m in used.Matches(attribute.Value))
                {
                    keys.Add(m.Groups[1].Success ? m.Groups[1].Value : m.Groups[2].Value);
                }
            }
        }
        var code = Directory.EnumerateFiles(SettingsFolder(), "*.cs", SearchOption.AllDirectories)
            .Concat(Directory.EnumerateFiles(Path.Combine(SettingsFolder(), "..", "DsaSettings.Core"), "*.cs", SearchOption.AllDirectories))
            .Select(File.ReadAllText);
        foreach (var source in code)
        {
            foreach (Match m in Regex.Matches(source, "\"(A11y_[A-Za-z0-9_]+)\""))
            {
                keys.Add(m.Groups[1].Value);
            }
        }
        Assert.Contains("A11y_ItemName", keys);
        var missing = keys.Where(k => !english.ContainsKey(k) || !korean.ContainsKey(k)).ToList();
        Assert.True(missing.Count == 0, "these texts are missing in English or Korean: " + string.Join(", ", missing));

        // A format says the same things in both languages, whatever its order.
        string Holes(string s) => string.Join(",", Regex.Matches(s, @"\{\d+\}").Select(m => m.Value).Order());
        foreach (var key in english.Keys.Where(k => k.StartsWith("A11y_", StringComparison.Ordinal)))
        {
            Assert.True(korean.ContainsKey(key), key + " has no Korean text");
            Assert.Equal(Holes(english[key]), Holes(korean[key]));
            Assert.False(string.IsNullOrWhiteSpace(korean[key]), key + " is empty in Korean");
        }
    }

    [Fact]
    public void Fr18_the_title_bar_and_navigation_buttons_have_names()
    {
        var code = Code("MainWindow.xaml.cs");
        Assert.Contains("NameTitleBarButtons();", code);
        foreach (var key in new[] { "A11y_Minimize", "A11y_Maximize", "A11y_Restore", "A11y_Close" })
        {
            Assert.Contains("\"" + key + "\"", code);
        }
        var toggle = Xaml("MainWindow.xaml").Descendants(Wpf + "Button")
            .Single(b => (string?)b.Attribute(X + "Name") == "PART_ToggleButton");
        Assert.Equal("{l:Text A11y_NavToggle}", (string?)toggle.Attribute(AutomationName));
    }

    [Fact]
    public void Fr18_the_uninstall_window_names_its_progress_and_announces_its_result()
    {
        var window = Xaml("UninstallAppWindow.xaml");
        var strip = window.Descendants(Local + "ProgressStrip").Single();
        Assert.Equal("ProgressMarquee", (string?)strip.Attribute(X + "Name"));
        Assert.NotNull(strip.Attribute(AutomationName));
        var code = Code("UninstallAppWindow.xaml.cs");
        Assert.Contains("Announce.Say(this", code);
        Assert.Contains("CloseButton.Focus()", code);
        Assert.Contains("AutomationControlType.ProgressBar", Code("Accessible.cs"));
    }
}
