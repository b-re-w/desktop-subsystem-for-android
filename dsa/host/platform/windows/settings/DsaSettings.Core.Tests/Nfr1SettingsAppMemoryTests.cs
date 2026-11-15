using System.Text.RegularExpressions;
using System.Xml.Linq;
using Xunit;

namespace Dsa.Settings.Core.Tests;

/// <summary>The settings app's own memory: it makes page state when a page is used, decodes
/// app icons at the size they are shown, polls only while its window can be seen, and runs
/// with a small .NET footprint.</summary>
public class Nfr1SettingsAppMemoryTests
{
    private static string Find(params string[] parts)
    {
        var inSettings = Path.Combine(parts);
        string[] relative = { inSettings,
            Path.Combine("host", "platform", "windows", "settings", inSettings) };
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var found = relative.Select(r => Path.Combine(dir.FullName, r)).FirstOrDefault(File.Exists);
            if (found is not null)
            {
                return found;
            }
        }
        throw new FileNotFoundException(inSettings);
    }

    [Fact]
    public void Nfr1_page_view_models_are_made_when_first_used()
    {
        var app = File.ReadAllText(Find("DsaSettings", "App.xaml.cs"));
        var eager = Regex.Matches(app, @"internal static \w+ViewModel \w+ \{ get; \} = new");
        Assert.Empty(eager);
        Assert.Matches(@"internal static MemoryViewModel Memory => _memory \?\?= new", app);
    }

    [Fact]
    public void Nfr1_app_icons_are_decoded_at_the_size_they_are_shown()
    {
        // 32 px on a 150% monitor, at 100%, and an icon smaller than the space it gets.
        Assert.Equal(48, IconDecodeSize.Pixels(32, 1.5, 256));
        Assert.Equal(32, IconDecodeSize.Pixels(32, 1.0, 256));
        Assert.Equal(32, IconDecodeSize.Pixels(40, 2.0, 32));
        Assert.Equal(40, IconDecodeSize.Pixels(40, 0.5, 0));
        foreach (var page in new[] { "AppsPage.xaml", "BackgroundPage.xaml" })
        {
            var xaml = File.ReadAllText(Find("DsaSettings", "Pages", page));
            foreach (Match image in Regex.Matches(xaml, @"<Image Source=""\{Binding Icon[^""]*"""))
            {
                Assert.Contains("Converter={StaticResource IconImage}", image.Value);
            }
        }
    }

    [Fact]
    public void Nfr1_pages_stop_polling_while_the_window_is_minimised_or_hidden()
    {
        Assert.True(PagePolling.ShouldPoll(loaded: true, windowVisible: true, windowMinimized: false));
        Assert.False(PagePolling.ShouldPoll(loaded: true, windowVisible: true, windowMinimized: true));
        Assert.False(PagePolling.ShouldPoll(loaded: true, windowVisible: false, windowMinimized: false));
        Assert.False(PagePolling.ShouldPoll(loaded: false, windowVisible: true, windowMinimized: false));
        var pages = Path.GetDirectoryName(Find("DsaSettings", "Pages", "MemoryPage.xaml.cs"))!;
        foreach (var file in Directory.GetFiles(pages, "*.xaml.cs"))
        {
            var code = File.ReadAllText(file);
            if (!code.Contains("DispatcherTimer"))
            {
                continue;
            }
            Assert.Contains("PollingTimers.Attach(this", code);
            Assert.DoesNotMatch(@"_\w*[tT]imer\.Start\(\)", code);
        }
    }

    [Fact]
    public void Nfr1_the_settings_app_runs_with_a_small_dotnet_footprint()
    {
        var project = XDocument.Load(Find("DsaSettings", "DsaSettings.csproj"));
        string? Property(string name) => (string?)project.Descendants(name).SingleOrDefault();
        Assert.Equal("true", Property("PublishReadyToRun"));
        Assert.Equal("false", Property("ServerGarbageCollection"));
        Assert.Equal("false", Property("ConcurrentGarbageCollection"));
        Assert.Equal("false", Property("TieredPGO"));
        Assert.Contains(project.Descendants("RuntimeHostConfigurationOption"),
            o => (string?)o.Attribute("Include") == "System.GC.ConserveMemory");
    }
}
