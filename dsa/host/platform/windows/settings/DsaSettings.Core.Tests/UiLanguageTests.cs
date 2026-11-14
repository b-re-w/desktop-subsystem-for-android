using System.Globalization;
using System.Xml.Linq;
using Xunit;

namespace Dsa.Settings.Core.Tests;

/// <summary>The display language picks one of DSA's fourteen interface languages, and anything
/// else falls back to English. The settings app, the APK Installer and setup read it the same
/// way.</summary>
public class UiLanguageTests
{
    public static readonly string[] Tags =
        { "en", "ko", "ja", "zh-CN", "zh-TW", "es", "fr", "de", "pt-BR", "ru", "it", "vi", "id", "tr" };

    [Fact]
    public void Fr17_there_are_the_fourteen_languages_in_order()
    {
        Assert.Equal(Tags, UiLanguage.All.Select(l => l.Tag));
        Assert.Equal(Enumerable.Range(0, Tags.Length), UiLanguage.All.Select(l => l.Index));
        Assert.Same(UiLanguage.English, UiLanguage.All[0]);
        foreach (var language in UiLanguage.All)
        {
            Assert.Same(language, UiLanguage.FromTag(language.Tag));
            Assert.Same(language, UiLanguage.FromTag(language.Tag.ToUpperInvariant()));
        }
        Assert.Null(UiLanguage.FromTag("nl"));
        Assert.Null(UiLanguage.FromTag(null));
    }

    [Theory]
    [InlineData("en-US", "en")]
    [InlineData("en-GB", "en")]
    [InlineData("en", "en")]
    [InlineData("ko-KR", "ko")]
    [InlineData("ko", "ko")]
    [InlineData("ja-JP", "ja")]
    [InlineData("zh-CN", "zh-CN")]
    [InlineData("zh-SG", "zh-CN")]
    [InlineData("zh-Hans", "zh-CN")]
    [InlineData("zh", "zh-CN")]
    [InlineData("zh-TW", "zh-TW")]
    [InlineData("zh-HK", "zh-TW")]
    [InlineData("zh-MO", "zh-TW")]
    [InlineData("zh-Hant", "zh-TW")]
    [InlineData("zh-Hant-HK", "zh-TW")]
    [InlineData("zh-Hans-HK", "zh-CN")]
    [InlineData("pt-BR", "pt-BR")]
    [InlineData("pt-PT", "pt-BR")]
    [InlineData("pt", "pt-BR")]
    [InlineData("es-ES", "es")]
    [InlineData("es-MX", "es")]
    [InlineData("fr-FR", "fr")]
    [InlineData("fr-CA", "fr")]
    [InlineData("de-DE", "de")]
    [InlineData("de-AT", "de")]
    [InlineData("ru-RU", "ru")]
    [InlineData("it-IT", "it")]
    [InlineData("vi-VN", "vi")]
    [InlineData("id-ID", "id")]
    [InlineData("in", "id")]
    [InlineData("tr-TR", "tr")]
    public void Fr17_a_system_language_picks_its_interface_language(string culture, string tag)
    {
        Assert.Equal(tag, UiLanguage.Resolve(culture).Tag);
    }

    [Theory]
    [InlineData("en-US", "en")]
    [InlineData("ko-KR", "ko")]
    [InlineData("ja-JP", "ja")]
    [InlineData("zh-CN", "zh-CN")]
    [InlineData("zh-SG", "zh-CN")]
    [InlineData("zh-Hans", "zh-CN")]
    [InlineData("zh-TW", "zh-TW")]
    [InlineData("zh-HK", "zh-TW")]
    [InlineData("zh-MO", "zh-TW")]
    [InlineData("zh-Hant", "zh-TW")]
    [InlineData("pt-PT", "pt-BR")]
    [InlineData("es-MX", "es")]
    [InlineData("fr-CA", "fr")]
    [InlineData("de-AT", "de")]
    [InlineData("tr-TR", "tr")]
    [InlineData("nl-NL", "en")]
    [InlineData("pl-PL", "en")]
    public void Fr17_a_system_culture_picks_its_interface_language(string culture, string tag)
    {
        // The culture as Windows gives it, with its parents.
        var info = CultureOrNull(culture);
        if (info is null)
        {
            // A system without culture data; the name alone decides, as the test above shows.
            return;
        }
        Assert.Equal(tag, UiLanguage.Resolve(info).Tag);
    }

    [Theory]
    [InlineData("nl-NL")]
    [InlineData("pl-PL")]
    [InlineData("sv-SE")]
    [InlineData("ar-SA")]
    [InlineData("hi-IN")]
    [InlineData("")]
    [InlineData(null)]
    [InlineData("not a culture")]
    public void Fr17_a_language_dsa_is_not_written_in_falls_back_to_english(string? culture)
    {
        Assert.Same(UiLanguage.English, UiLanguage.Resolve(culture));
    }

    [Fact]
    public void Fr17_the_invariant_culture_falls_back_to_english()
    {
        Assert.Same(UiLanguage.English, UiLanguage.Resolve(CultureInfo.InvariantCulture));
        Assert.Same(UiLanguage.English, UiLanguage.Resolve((CultureInfo?)null));
    }

    [Theory]
    [InlineData("pt-PT", "pt-BR")]
    [InlineData("zh-HK", "zh-TW")]
    [InlineData("zh-SG", "zh-CN")]
    [InlineData("fr-CA", "fr")]
    [InlineData("nl-NL", "en")]
    public void Fr17_the_tables_are_read_in_the_culture_of_the_language_they_are_written_in(string culture, string table)
    {
        // A pt-PT or zh-HK desktop reads the table written for it, which .NET's own fallback
        // from those cultures would miss.
        Assert.Equal(table, UiLanguage.Resolve(culture).Culture.Name);
    }

    [Fact]
    public void Fr17_every_language_has_a_string_table_name()
    {
        Assert.Equal("Strings.resx", StringTables.FileName(UiLanguage.English));
        Assert.Equal("Strings.ko.resx", StringTables.FileName(UiLanguage.Korean));
        Assert.Equal("Strings.zh-TW.resx", StringTables.FileName(UiLanguage.ChineseTraditional));
        Assert.Equal("Strings.pt-BR.resx", StringTables.FileName(UiLanguage.PortugueseBrazil));
        Assert.Equal(14, StringTables.Files().Count());
    }

    [Fact]
    public void Fr17_korean_and_chinese_draw_strong_text_in_the_regular_face()
    {
        var regular = UiLanguage.All.Where(UiFonts.HasNoSemibold).Select(l => l.Tag);
        Assert.Equal(new[] { "ko", "zh-CN", "zh-TW" }, regular);
    }

    [Theory]
    [InlineData("ja", "Yu Gothic UI")]
    [InlineData("zh-CN", "Microsoft YaHei UI")]
    [InlineData("zh-TW", "Microsoft JhengHei UI")]
    public void Fr17_japanese_and_chinese_take_their_own_font_before_malgun_gothic(string tag, string font)
    {
        var fonts = UiFonts.For(UiLanguage.FromTag(tag)!);
        Assert.NotNull(fonts);
        foreach (var list in new[] { fonts!.Text, fonts.Display, fonts.StrongText, fonts.StrongDisplay })
        {
            var families = list.Split(',').Select(f => f.Trim()).ToList();
            var own = families.FindIndex(f => f.StartsWith(font, StringComparison.Ordinal));
            Assert.True(own >= 0, $"{list} has no {font}");
            Assert.True(own < families.IndexOf("Malgun Gothic"), $"{list} takes Malgun Gothic before {font}");
            Assert.StartsWith("Segoe UI Variable", families[0]);
        }
    }

    [Fact]
    public void Fr17_the_other_languages_keep_the_apps_own_fonts()
    {
        foreach (var language in UiLanguage.All.Where(l => l != UiLanguage.Japanese
            && l != UiLanguage.ChineseSimplified && l != UiLanguage.ChineseTraditional))
        {
            Assert.Null(UiFonts.For(language));
        }
    }

    [Theory]
    [InlineData("DsaSettings", "DsaSettings.csproj")]
    [InlineData("DsaApkInstaller", "DsaApkInstaller.csproj")]
    public void Fr17_the_apps_keep_the_resources_of_every_language(string folder, string project)
    {
        var path = Settings(folder, project);
        var doc = XDocument.Load(path);
        var kept = ((string?)doc.Descendants("SatelliteResourceLanguages").Single() ?? "").Split(';');
        foreach (var tag in Tags)
        {
            Assert.Contains(tag, kept);
        }
        // WPF's and WPF UI's own Chinese resources are named by script.
        Assert.Contains("zh-Hans", kept);
        Assert.Contains("zh-Hant", kept);
    }

    [Fact]
    public void Fr17_the_settings_app_builds_every_table_into_a_satellite()
    {
        // The SDK's default items take every Strings.<tag>.resx beside the app's project, and
        // the culture in its name makes it a satellite; nothing may switch them off or name
        // only some.
        var project = File.ReadAllText(Settings("DsaSettings", "DsaSettings.csproj"));
        Assert.DoesNotContain("EnableDefaultEmbeddedResourceItems", project);
        Assert.DoesNotContain("<EmbeddedResource Remove", project);
        Assert.DoesNotContain("Strings.ko.resx", project);
        var folder = Path.GetDirectoryName(Settings("DsaSettings", "DsaSettings.csproj"))!;
        Assert.True(File.Exists(Path.Combine(folder, "Strings.resx")));
    }

    [Fact]
    public void Fr17_the_settings_app_reads_its_texts_in_the_resolved_language()
    {
        var source = File.ReadAllText(Settings("DsaSettings", "Text.cs"));
        Assert.Contains("UiLanguage.Resolve(CultureInfo.CurrentUICulture).Culture", source);
        // --lang still shows the app in another language than the display language.
        var app = File.ReadAllText(Settings("DsaSettings", "App.xaml.cs"));
        Assert.Contains("Array.IndexOf(e.Args, \"--lang\")", app);
        Assert.Contains("CultureInfo.CurrentUICulture = culture", app);
    }

    private static CultureInfo? CultureOrNull(string name)
    {
        try
        {
            return CultureInfo.GetCultureInfo(name);
        }
        catch (CultureNotFoundException)
        {
            return null;
        }
    }

    internal static string Settings(params string[] parts)
    {
        // The tests run from dsa/target/settings/..., outside the source tree, so each folder
        // above is tried both as the settings folder and as the repository's dsa folder.
        var inSettings = Path.Combine(parts);
        string[] relative = { inSettings, Path.Combine("host", "platform", "windows", "settings", inSettings) };
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var found = relative.Select(r => Path.Combine(dir.FullName, r))
                .FirstOrDefault(p => File.Exists(p) || Directory.Exists(p));
            if (found is not null)
            {
                return found;
            }
        }
        throw new FileNotFoundException(inSettings + " was not found above " + AppContext.BaseDirectory);
    }
}
