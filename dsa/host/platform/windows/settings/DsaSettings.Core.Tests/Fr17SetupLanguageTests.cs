using System.Reflection;
using System.Text.RegularExpressions;
using System.Xml.Linq;
using Xunit;
using SetupText = Dsa.Setup.Text;

namespace Dsa.Settings.Core.Tests;

/// <summary>The setup window is in the user's interface language: one of the fourteen, picked
/// from the display language as the settings app picks it. Setup is a .NET Framework app, so its
/// text file is compiled in here, the way setup compiles it, and read in every language.</summary>
[Collection(nameof(SetupLanguage))]
public class Fr17SetupLanguageTests
{
    /// <summary>The setup texts that are part of a sentence around a link, which a language may
    /// leave empty where its word order puts nothing.</summary>
    private static readonly string[] Fragments = { "FailureLogBefore" };

    /// <summary>The widest a line of the help may be, in columns (a wide character takes two):
    /// the help is set in a fixed-width font in a window of fixed width.</summary>
    private const int HelpColumns = 56;

    private static IReadOnlyList<PropertyInfo> Texts() =>
        typeof(SetupText).GetProperties(BindingFlags.Public | BindingFlags.Static)
            .Where(p => p.PropertyType == typeof(string))
            .ToList();

    /// <summary>Every text of setup in the language, by name.</summary>
    private static Dictionary<string, string> In(UiLanguage language)
    {
        var before = SetupText.Language;
        try
        {
            SetupText.Language = language;
            return Texts().ToDictionary(p => p.Name, p => (string)p.GetValue(null)!);
        }
        finally
        {
            SetupText.Language = before;
        }
    }

    public static IEnumerable<object[]> Languages() => UiLanguage.All.Select(l => new object[] { l.Tag });

    private static string Find(string file)
    {
        // The tests run from dsa/target/settings/..., outside the source tree, so each folder
        // above is tried both as the settings folder and as the repository's dsa folder.
        string[] relative = { Path.Combine("..", "installer", "setup-ui", file),
            Path.Combine("host", "platform", "windows", "installer", "setup-ui", file) };
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var found = relative.Select(r => Path.Combine(dir.FullName, r)).FirstOrDefault(File.Exists);
            if (found is not null)
            {
                return found;
            }
        }
        throw new FileNotFoundException(file + " was not found above " + AppContext.BaseDirectory);
    }

    [Fact]
    public void Fr17_setup_has_its_texts()
    {
        // The reflection below sees the texts; a change that hid them would pass every test.
        Assert.True(Texts().Count >= 50, $"setup has only {Texts().Count} texts");
        Assert.Contains(Texts(), p => p.Name == "HelpText");
    }

    [Theory]
    [MemberData(nameof(Languages))]
    public void Fr17_every_setup_text_is_in_every_language(string tag)
    {
        var language = UiLanguage.FromTag(tag)!;
        var english = In(UiLanguage.English);
        var texts = In(language);
        foreach (var (name, value) in texts)
        {
            if (!Fragments.Contains(name))
            {
                Assert.False(string.IsNullOrWhiteSpace(value), $"{name} is empty in {tag}");
            }
            Assert.True(StringTables.Holes(english[name]).SequenceEqual(StringTables.Holes(value)),
                $"{name} in {tag} takes other values than in English: {value}");
            Assert.DoesNotContain(TextLengthTests.EmDash, value);
            Assert.DoesNotContain('\uFFFD', value);
        }
        // The sentence around the log link reads as a whole.
        Assert.False(string.IsNullOrWhiteSpace(texts["FailureLogBefore"] + texts["FailureLogLink"] + texts["FailureLogAfter"]));
    }

    [Theory]
    [MemberData(nameof(Languages))]
    public void Fr17_setup_names_stay_as_they_are_in_every_language(string tag)
    {
        var texts = In(UiLanguage.FromTag(tag)!);
        Assert.Contains("Desktop Subsystem for Android™", texts["Caption"]);
        Assert.Contains("Desktop Subsystem for Android™", texts["Logo"]);
        // The window marks the command in these as code, so it must be there word for word.
        Assert.Contains(SetupText.CodeInText, texts["SuccessInstallText"]);
        Assert.Contains(SetupText.CodeInText, texts["SuccessInstallTextBundledImage"]);
        // The notice under the licence link.
        Assert.Contains("Google LLC", texts["LegalAfter"]);
        Assert.Contains("Microsoft", texts["LegalAfter"]);
        Assert.Contains("AppData\\Local\\DSA", texts["OptionsDataNote"]);
        Assert.Contains("AppData\\Local\\DSA", texts["SuccessUninstallText"]);
        Assert.Contains("C:\\Program Files\\DSA", texts["OptionsInvalid"]);
        // The maintenance screen names its two buttons as they are labelled.
        Assert.Contains(texts["RepairButton"], texts["ModifyText"]);
        Assert.Contains(texts["UninstallButton"], texts["ModifyText"]);
        Assert.Equal("Desktop Subsystem for Android™", SetupText.Title);
        Assert.Equal("ThisIsThePy", SetupText.Publisher);
        Assert.Equal("Apache License 2.0", SetupText.LegalLink);
        Assert.Equal("dsa status", SetupText.CodeInText);
    }

    [Theory]
    [InlineData("en", "Cancel", "Close", "Minimize")]
    [InlineData("ko", "취소", "닫기", "최소화")]
    [InlineData("ja", "キャンセル", "閉じる", "最小化")]
    [InlineData("zh-CN", "取消", "关闭", "最小化")]
    [InlineData("zh-TW", "取消", "關閉", "最小化")]
    [InlineData("es", "Cancelar", "Cerrar", "Minimizar")]
    [InlineData("fr", "Annuler", "Fermer", "Réduire")]
    [InlineData("de", "Abbrechen", "Schließen", "Minimieren")]
    [InlineData("pt-BR", "Cancelar", "Fechar", "Minimizar")]
    [InlineData("ru", "Отмена", "Закрыть", "Свернуть")]
    [InlineData("it", "Annulla", "Chiudi", "Riduci a icona")]
    [InlineData("vi", "Hủy", "Đóng", "Thu nhỏ")]
    [InlineData("id", "Batal", "Tutup", "Minimalkan")]
    [InlineData("tr", "İptal", "Kapat", "Simge durumuna küçült")]
    public void Fr17_each_language_gets_its_own_words(string tag, string cancel, string close, string minimize)
    {
        // The words Windows itself uses in that language, so each language's texts are the ones
        // setup shows in it and not a neighbour's.
        var texts = In(UiLanguage.FromTag(tag)!);
        Assert.Equal(cancel, texts["CancelButton"]);
        Assert.Equal(close, texts["CloseButton"]);
        Assert.Equal(minimize, texts["Minimize"]);
    }

    [Theory]
    [MemberData(nameof(Languages))]
    public void Fr17_setup_help_keeps_its_lines_and_columns(string tag)
    {
        var english = In(UiLanguage.English)["HelpText"].Split('\n');
        var help = In(UiLanguage.FromTag(tag)!)["HelpText"].Split('\n');
        Assert.Equal(english.Length, help.Length);
        for (var i = 0; i < english.Length; i++)
        {
            // A switch line starts at the left with the same switches; what it does is indented
            // by four spaces, on the same lines as in English.
            var indented = english[i].StartsWith("    ", StringComparison.Ordinal);
            Assert.Equal(indented, help[i].StartsWith("    ", StringComparison.Ordinal));
            Assert.False(help[i].StartsWith("     ", StringComparison.Ordinal), $"line {i + 1} in {tag} is indented by more than four spaces");
            if (!indented)
            {
                Assert.Equal(Switches(english[i]), Switches(help[i]));
            }
            Assert.True(help[i].Trim().Length > 0, $"line {i + 1} of the help is empty in {tag}");
            Assert.True(TextLengthTests.Columns(help[i]) <= HelpColumns,
                $"line {i + 1} of the help in {tag} is {TextLengthTests.Columns(help[i])} columns: {help[i]}");
        }
    }

    /// <summary>The switches and settings a help line names, without the words in brackets that
    /// a language translates: <c>/layout [folder]</c> is <c>/layout</c>.</summary>
    private static string Switches(string line) =>
        string.Join(" ", Regex.Replace(line, @"\[[^\]]*\]|<[^>]*>", "").Split(' ', StringSplitOptions.RemoveEmptyEntries));

    [Fact]
    public void Fr17_setup_follows_the_display_language_as_the_settings_app_does()
    {
        var text = File.ReadAllText(Find("Text.cs"));
        Assert.Contains("UiLanguage.Resolve(CultureInfo.CurrentUICulture)", text);
        Assert.DoesNotContain("TwoLetterISOLanguageName", text);
        // Setup compiles the settings app's resolver in, so both pick the same language.
        var project = XDocument.Load(Find("DsaSetup.csproj"));
        Assert.Contains(project.Descendants("Compile"), c =>
            ((string?)c.Attribute("Include") ?? "").Replace('/', '\\').EndsWith(@"settings\DsaSettings.Core\UiLanguage.cs", StringComparison.Ordinal));
        // The window's language picks the fallback font for Japanese, Chinese and Korean.
        Assert.Contains("this.Language = System.Windows.Markup.XmlLanguage.GetLanguage(Text.Language.Tag)", File.ReadAllText(Find("MainWindow.xaml.cs")));
    }

    [Theory]
    [InlineData("pt-PT", "pt-BR")]
    [InlineData("zh-HK", "zh-TW")]
    [InlineData("zh-SG", "zh-CN")]
    [InlineData("ko-KR", "ko")]
    [InlineData("nl-NL", "en")]
    public void Fr17_setup_maps_system_languages_as_the_settings_app_does(string culture, string tag)
    {
        // The one resolver both compile: what it says for a culture is what setup shows.
        var language = UiLanguage.Resolve(culture);
        Assert.Equal(tag, language.Tag);
        var cancel = In(language)["CancelButton"];
        Assert.Equal(In(UiLanguage.FromTag(tag)!)["CancelButton"], cancel);
    }
}

/// <summary>The setup tests change the language setup's texts are read in, so they run one at a
/// time.</summary>
[CollectionDefinition(nameof(SetupLanguage), DisableParallelization = true)]
public sealed class SetupLanguage
{
}
