using System.Text.RegularExpressions;
using System.Xml.Linq;
using Xunit;

namespace Dsa.Settings.Core.Tests;

/// <summary>The APK Installer's window is in the user's interface language, as the settings
/// app is: it embeds every string table of the settings app, reads them in the language the
/// display language resolves to, has no English of its own, and takes its title from the
/// tables. It carries DSA's own parcel icon.</summary>
public class ApkInstallerLanguageTests
{
    private static string Find(params string[] parts) => UiLanguageTests.Settings(parts);

    private static XDocument Project() => XDocument.Load(Find("DsaApkInstaller", "DsaApkInstaller.csproj"));

    /// <summary>Whether a file name matches an MSBuild wildcard of <c>*</c> and <c>?</c>.</summary>
    private static bool Matches(string pattern, string name) =>
        Regex.IsMatch(name, "^" + Regex.Escape(pattern).Replace(@"\*", ".*").Replace(@"\?", ".") + "$",
            RegexOptions.IgnoreCase);

    [Fact]
    public void Fr11_installer_embeds_every_string_table()
    {
        var resources = Project().Descendants("EmbeddedResource").ToList();
        var tables = resources
            .Where(e => ((string?)e.Attribute("Include") ?? "").Replace('/', '\\').StartsWith(@"..\DsaSettings\Strings", StringComparison.Ordinal))
            .ToList();
        Assert.NotEmpty(tables);
        foreach (var language in UiLanguage.All)
        {
            var file = StringTables.FileName(language);
            Assert.True(tables.Any(e => Matches(Path.GetFileName((string)e.Attribute("Include")!), file)),
                $"the APK Installer does not embed {file}, so it would not show {language.Tag}");
        }
        // Each table keeps its file name, so the culture in it makes it a satellite of the
        // installer's own Strings resources.
        foreach (var table in tables)
        {
            var link = (string?)table.Attribute("Link") ?? "";
            Assert.True(link == "%(Filename)%(Extension)" || Regex.IsMatch(link, @"^Strings(\.[A-Za-z-]+)?\.resx$"),
                $"{table.Attribute("Include")} is linked as {link}, which loses the culture in its name");
        }
        var kept = ((string?)Project().Descendants("SatelliteResourceLanguages").Single() ?? "").Split(';');
        foreach (var language in UiLanguage.All)
        {
            Assert.Contains(language.Tag, kept);
        }
    }

    [Fact]
    public void Fr11_installer_reads_its_texts_in_the_ui_language()
    {
        var source = File.ReadAllText(Find("DsaApkInstaller", "Text.cs"));
        Assert.Contains("UiLanguage.Resolve(CultureInfo.CurrentUICulture).Culture", source);
        Assert.DoesNotContain("InvariantCulture", source);
        // --lang still shows the window in another language than the display language.
        var app = File.ReadAllText(Find("DsaApkInstaller", "App.xaml.cs"));
        Assert.Contains("Array.IndexOf(e.Args, \"--lang\")", app);
        Assert.Contains("CultureInfo.CurrentUICulture = culture", app);
    }

    /// <summary>The attributes whose words a user sees or a screen reader says.</summary>
    private static readonly string[] ShownAttributes =
    {
        "Text", "Content", "Title", "ToolTip", "Header", "PlaceholderText",
        "AutomationProperties.Name", "AutomationProperties.HelpText",
    };

    [Fact]
    public void Fr11_installer_has_no_english_of_its_own()
    {
        var folder = Find("DsaApkInstaller");
        // XAML: a shown attribute holds a binding, a resource or a symbol, never words.
        foreach (var file in Directory.EnumerateFiles(folder, "*.xaml"))
        {
            foreach (var attribute in XDocument.Load(file).Descendants().Attributes()
                .Where(a => ShownAttributes.Contains(a.Name.LocalName)))
            {
                Assert.False(Regex.IsMatch(attribute.Value, "^[^{].*[A-Za-z]"),
                    $"{Path.GetFileName(file)}: {attribute.Name.LocalName}=\"{attribute.Value}\" is written into the window, not taken from the string tables");
            }
        }
        // Code: no string literal is a phrase of words. Comments may be.
        var phrase = new Regex("\"[^\"\\r\\n]*[A-Za-z]{2,} [A-Za-z]{2,}[^\"\\r\\n]*\"");
        foreach (var file in Directory.EnumerateFiles(folder, "*.cs"))
        {
            var code = File.ReadAllLines(file).Where(line => !line.TrimStart().StartsWith("//", StringComparison.Ordinal));
            foreach (var line in code)
            {
                Assert.False(phrase.IsMatch(line), $"{Path.GetFileName(file)} writes words into the window: {line.Trim()}");
            }
        }
        // The view model's labels and the sentences for files it cannot install come from the
        // tables too.
        var model = File.ReadAllText(Find("DsaSettings.Core", "ApkInstallViewModel.cs"));
        foreach (var english in new[] { "\"Install ", "\"Package Name:\"", "\"Publisher:\"", "\"Version:\"", "\"Source:\"",
            "Capabilities (", "\"More\"", "\"Less\"", "dsa ended with code" })
        {
            Assert.DoesNotContain(english, model);
        }
        var files = File.ReadAllText(Find("DsaSettings.Core", "UnsupportedFile.cs"));
        Assert.DoesNotContain("is not a valid APK file", files);
        Assert.DoesNotContain("cannot run in DSA", files);
    }

    [Fact]
    public void Fr11_installer_title_follows_the_ui_language()
    {
        // Both places the title shows (the window and the file picker) take the settings app's
        // name for the window, so there is one source for it in every language.
        Assert.Contains("Title = App.Text.Get(\"ApkWindow_Caption\")", File.ReadAllText(Find("DsaApkInstaller", "InstallerWindow.xaml.cs")));
        Assert.Contains("Title = Text.Get(\"ApkWindow_Caption\")", File.ReadAllText(Find("DsaApkInstaller", "App.xaml.cs")));
        Assert.False(File.Exists(Path.Combine(Find("DsaSettings.Core"), "ApkInstallerTitle.cs")),
            "a second title lookup beside the string tables would drift from them");
        Assert.Equal("APK Installer", TableText.Of("Strings.resx").Get("ApkWindow_Caption"));
        Assert.Equal("APK 설치 관리자", TableText.Of("Strings.ko.resx").Get("ApkWindow_Caption"));
    }

    [Theory]
    [MemberData(nameof(StringTables.Files), MemberType = typeof(StringTables))]
    public void Fr11_installer_has_its_title_and_labels_in_every_language(string file)
    {
        var strings = StringTables.Load(file);
        foreach (var key in new[] { "ApkWindow_Caption", "ApkWindow_InstallQuestion", "ApkWindow_PackageLabel",
            "ApkWindow_PublisherLabel", "ApkWindow_VersionLabel", "ApkWindow_SourceLabel", "ApkWindow_CapabilitiesHeader",
            "ApkWindow_More", "ApkWindow_Less", "ApkWindow_AppIcon", "ApkWindow_DsaEnded", "ApkWindow_PickerApks",
            "ApkWindow_PickerAllFiles" }.Concat(UnsupportedFile.AllKeys.Keys))
        {
            Assert.True(strings.TryGetValue(key, out var value) && value.Trim().Length > 0, $"{file} has no {key}");
        }
        Assert.Contains("{0}", strings["ApkWindow_InstallQuestion"]);
        Assert.Contains("{0}", strings["ApkWindow_CapabilitiesHeader"]);
        foreach (var (key, count) in UnsupportedFile.AllKeys)
        {
            Assert.Equal(Enumerable.Range(0, count), StringTables.Holes(strings[key]).Distinct());
        }
        // The picker's choices are put between the bars of a filter, so a bar in one would
        // break the filter.
        Assert.DoesNotContain("|", strings["ApkWindow_PickerApks"]);
        Assert.DoesNotContain("|", strings["ApkWindow_PickerAllFiles"]);
    }

    [Fact]
    public void Fr11_the_file_picker_names_its_choices_in_the_users_language()
    {
        var app = File.ReadAllText(Find("DsaApkInstaller", "App.xaml.cs"));
        Assert.Contains("Filter = PickerFilter(Text)", app);
        Assert.Contains("text.Get(\"ApkWindow_PickerApks\")", app);
        Assert.Contains("text.Get(\"ApkWindow_PickerAllFiles\")", app);
        Assert.Contains("|*.apk;*.xapk;*.apks;*.apkm|", app);
    }

    [Fact]
    public void Fr11_installer_icon_is_dsas_open_box_drawing()
    {
        var project = Project();
        Assert.Equal(@"Assets\apk-installer.ico", (string?)project.Descendants("ApplicationIcon").Single());
        var ico = File.ReadAllBytes(Find("DsaApkInstaller", "Assets", "apk-installer.ico"));
        var png = File.ReadAllBytes(Find("DsaApkInstaller", "Assets", "apk-installer.png"));
        Assert.True(Contains(ico, png), "the .ico does not hold the 256 px open box image");
        // The drawing it is rendered from: the open cardboard box, not DSA's symbol with a badge
        // and not the earlier parcel with an arrow.
        var master = File.ReadAllText(Find("..", "..", "..", "..", "brand", "apk-installer.svg"));
        Assert.Contains("open cardboard box", master);
        Assert.DoesNotContain("arrow", master);
        Assert.DoesNotContain("<filter", master);
        var window = File.ReadAllText(Find("DsaApkInstaller", "InstallerWindow.xaml.cs"));
        Assert.DoesNotContain("WindowsApps", window);
        Assert.DoesNotContain("AppInstaller", window);
    }

    private static bool Contains(byte[] haystack, byte[] needle)
    {
        for (var i = 0; i + needle.Length <= haystack.Length; i++)
        {
            if (haystack.AsSpan(i, needle.Length).SequenceEqual(needle))
            {
                return true;
            }
        }
        return false;
    }
}
