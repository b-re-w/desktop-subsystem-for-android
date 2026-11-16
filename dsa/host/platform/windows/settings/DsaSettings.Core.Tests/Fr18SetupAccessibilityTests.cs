using System.Text.RegularExpressions;
using System.Xml.Linq;
using Xunit;
using SetupScreenReader = Dsa.Setup.ScreenReader;

namespace Dsa.Settings.Core.Tests;

/// <summary>The setup window works with a screen reader and the keyboard alone: its progress
/// bar has a name, a new screen puts the keyboard focus on its first control, and how setup went
/// is told to the screen reader. Setup is a .NET Framework app, so its files are read here and
/// its plain helper is compiled in.</summary>
public class Fr18SetupAccessibilityTests
{
    private static readonly XNamespace Wpf = "http://schemas.microsoft.com/winfx/2006/xaml/presentation";
    private static readonly XNamespace X = "http://schemas.microsoft.com/winfx/2006/xaml";

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

    private static XDocument Window() => XDocument.Load(Find("MainWindow.xaml"));

    private static string Code() => File.ReadAllText(Find("MainWindow.xaml.cs"));

    private static string? Name(XElement e) => (string?)e.Attribute(X + "Name");

    /// <summary>The body of the window's method that starts with the signature: up to the end
    /// of its block, which closes at the methods' indent.</summary>
    private static string Method(string code, string signature)
    {
        var start = code.IndexOf(signature, StringComparison.Ordinal);
        Assert.True(start >= 0, signature + " is not in MainWindow.xaml.cs");
        var body = code[start..];
        var end = Regex.Match(body, @"\r?\n        \}\r?\n");
        Assert.True(end.Success, signature + " has no end");
        return body[..end.Index];
    }

    private static bool IsNamed(XElement control, string code) =>
        control.Attribute(XName.Get("AutomationProperties.Name")) is not null
        || (Name(control) is { } name && code.Contains($"AutomationProperties.SetName(this.{name},"));

    [Fact]
    public void Fr18_setup_progress_has_a_name()
    {
        var code = Code();
        var bars = Window().Descendants(Wpf + "ProgressBar").ToList();
        Assert.NotEmpty(bars);
        foreach (var bar in bars)
        {
            Assert.True(Name(bar) is not null, "a progress bar has no x:Name, so nothing can name it");
            Assert.True(IsNamed(bar, code), Name(bar) + " has no UI Automation name");
        }
        // Each task names the bar for what it does: installing, repairing, uninstalling, copying.
        var progress = Method(code, "internal void ShowProgress(");
        Assert.Contains("AutomationProperties.SetName(this.Progress, this.ProgressTitle.Text)", progress);
        foreach (var words in new[] { "Text.ProgressUninstall", "Text.ProgressRepair", "Text.ProgressLayout", "Text.ProgressInstall" })
        {
            Assert.Contains(words, progress);
        }
    }

    [Fact]
    public void Fr18_setup_controls_and_images_have_names()
    {
        var code = Code();
        var doc = Window();
        // Every button shows words the code puts in it, or is given a name: the caption buttons
        // draw only a glyph.
        foreach (var button in doc.Descendants(Wpf + "Button"))
        {
            var name = Name(button)!;
            Assert.True(code.Contains($"this.{name}.Content =") || IsNamed(button, code), name + " shows no words and has no name");
        }
        Assert.Contains("AutomationProperties.SetName(this.CaptionMinimize, Text.Minimize)", code);
        Assert.Contains("AutomationProperties.SetName(this.CaptionClose, Text.CloseButton)", code);
        // The folder boxes are named by their labels.
        foreach (var box in doc.Descendants(Wpf + "TextBox"))
        {
            Assert.True(IsNamed(box, code), Name(box) + " has no UI Automation name");
        }
        // The logo, in the caption and beside the title, says whose it is.
        var images = doc.Descendants(Wpf + "Image").ToList();
        Assert.Equal(2, images.Count);
        foreach (var image in images)
        {
            Assert.True(IsNamed(image, code), (Name(image) ?? "an image") + " has no UI Automation name");
        }
        // The logo's name is in every interface language: English first, then Korean, and one
        // text for each of the other twelve.
        var text = File.ReadAllText(Find("Text.cs"));
        var logo = Regex.Match(text, @"Logo => Pick\((?<texts>[^;]*)\);");
        Assert.True(logo.Success, "Text.Logo is not a Pick of its names");
        var names = Regex.Matches(logo.Groups["texts"].Value, "\"([^\"]*)\"").Select(m => m.Groups[1].Value).ToList();
        Assert.Equal(14, names.Count);
        Assert.Equal("Desktop Subsystem for Android™ logo", names[0]);
        Assert.Equal("Desktop Subsystem for Android™ 로고", names[1]);
        Assert.All(names, name => Assert.Contains("Desktop Subsystem for Android™", name));
        // The Browse buttons say which folder each one picks.
        Assert.Contains("AutomationProperties.SetHelpText(this.BrowseProgram, Text.BrowseProgramTitle)", code);
        Assert.Contains("AutomationProperties.SetHelpText(this.BrowseData, Text.BrowseDataTitle)", code);
    }

    private sealed record Control(string Id, bool Visible = true, bool Enabled = true, bool Main = false);

    private static Control? First(Control[] fields, params Control[] buttons) =>
        SetupScreenReader.FirstControl(fields, buttons, c => c.Visible && c.Enabled, c => c.Main);

    private static readonly Control[] NoFields = Array.Empty<Control>();

    [Fact]
    public void Fr18_page_change_moves_focus_to_first_control()
    {
        // The footer's buttons in the window's order: Cancel, Close, Uninstall, Repair, OK,
        // Restart, Install, Done.
        // Ready to install: Install is the main button.
        Assert.Equal("Install", First(NoFields, new("Cancel"), new("Install", Main: true))?.Id);
        // Still looking at what is installed: Install is off, so Cancel.
        Assert.Equal("Cancel", First(NoFields, new("Cancel"), new("Install", Enabled: false))?.Id);
        // Already installed: Repair is the main button, after Close and Uninstall.
        Assert.Equal("Repair", First(NoFields, new("Close"), new("Uninstall"), new("Repair", Main: true))?.Id);
        // The folders: the program folder box comes before OK.
        Assert.Equal("ProgramFolderBox", First(new[] { new Control("ProgramFolderBox") }, new("Back"), new("OK", Main: true))?.Id);
        // Working: Cancel is the only control.
        Assert.Equal("Cancel", First(NoFields, new("Cancel"), new("Install", Visible: false), new("Done", Visible: false))?.Id);
        // Finished: Done; finished but a restart is needed: Restart.
        Assert.Equal("Done", First(NoFields, new("Cancel", Visible: false), new("Done", Main: true))?.Id);
        Assert.Equal("Restart", First(NoFields, new("Close"), new("Restart", Main: true))?.Id);
        // Failed, or help: Close.
        Assert.Equal("Close", First(NoFields, new("Cancel", Visible: false), new("Close"))?.Id);
        // Passive setup at its end: nothing is shown, so nothing takes the focus.
        Assert.Null(First(NoFields, new("Cancel", Visible: false), new("Done", Visible: false, Main: true)));
    }

    [Fact]
    public void Fr18_setup_window_moves_focus_whenever_the_screen_changes()
    {
        var code = Code();
        // Every screen is shown through Show, and Show ends by moving the focus.
        var show = Method(code, "private void Show(Screen screen,");
        Assert.Contains("this.FocusFirstControl(screen);", show);
        var focus = Method(code, "private void FocusFirstControl(Screen screen)");
        Assert.Contains("ScreenReader.FirstControl<UIElement>(", focus);
        Assert.Contains("this.ProgramFolderBox", focus);
        Assert.Contains("this.Footer.Children.OfType<Button>()", focus);
        Assert.Contains("button.IsDefault", focus);
        Assert.Contains("first.Focus();", focus);
        // No screen picks its own button any more, so none can be left out.
        foreach (var screen in new[] { "internal void ShowInstall()", "internal void ShowMaintenance()" })
        {
            Assert.DoesNotContain(".Focus()", Method(code, screen));
        }
    }

    [Fact]
    public void Fr18_completion_and_failure_are_announced_as_sentences_in_both_languages()
    {
        Assert.Equal("Installed. The DSA service is running. Type dsa status in a terminal to see your Android.",
            SetupScreenReader.Announcement("Installed", "The DSA service is running. Type dsa status in a terminal to see your Android."));
        Assert.Equal("설치됨. DSA 서비스가 실행 중입니다. 마치려면 컴퓨터를 다시 시작하세요.",
            SetupScreenReader.Announcement("설치됨", "DSA 서비스가 실행 중입니다.", "마치려면 컴퓨터를 다시 시작하세요."));
        // A failure: the title, the engine's message, then where the log is; an empty message
        // and no restart leave nothing behind.
        Assert.Equal("Setup did not finish. Error 0x80070005. The log file says what went wrong.",
            SetupScreenReader.Announcement("Setup did not finish", " Error 0x80070005 ", "The log file says what went wrong.", null));
        Assert.Equal("제거를 마치지 못했습니다. 로그 파일에서 무엇이 잘못됐는지 볼 수 있습니다.",
            SetupScreenReader.Announcement("제거를 마치지 못했습니다", null, "로그 파일에서 무엇이 잘못됐는지 볼 수 있습니다."));
        Assert.Equal("Cancelling…", SetupScreenReader.Announcement("Cancelling…"));
    }

    [Fact]
    public void Fr18_completion_and_failure_are_announced()
    {
        var code = Code();
        // Each result screen tells the screen reader what it shows, in the window's language.
        Assert.Contains("this.Announce(ScreenReader.Announcement(title, text, restart ? Text.RestartText : null));",
            Method(code, "internal void ShowSuccess("));
        var failure = Method(code, "internal void ShowFailure(");
        Assert.Contains("ScreenReader.Announcement(title, Text.FailureCancelled", failure);
        Assert.Contains("Text.FailureLogBefore + Text.FailureLogLink + Text.FailureLogAfter", failure);
        Assert.Contains("this.Announce(announcement);", failure);
        Assert.Contains("this.Announce(ScreenReader.Announcement(Text.NewerHeader, Text.NewerText));",
            Method(code, "internal void ShowNewer()"));
        // A UI Automation notification where the framework has one, else the result title's
        // live region, which says it changed.
        var announce = Method(code, "private void Announce(string words)");
        Assert.Contains("RaiseNotification(peer, words)", announce);
        Assert.Contains("AutomationEvents.LiveRegionChanged", announce);
        var notify = Method(code, "private static bool RaiseNotification(");
        Assert.Contains("\"RaiseNotificationEvent\"", notify);
        Assert.Contains("\"ActionCompleted\"", notify);
        Assert.Contains("\"ImportantMostRecent\"", notify);
        var title = Window().Descendants().Single(e => Name(e) == "ResultTitle");
        Assert.Equal("Assertive", (string?)title.Attribute(XName.Get("AutomationProperties.LiveSetting")));
    }
}
