using System.Globalization;
using System.Text.RegularExpressions;
using System.Xml.Linq;
using Xunit;
using ApkScreenReader = Dsa.ApkInstaller.ScreenReader;

namespace Dsa.Settings.Core.Tests;

/// <summary>The APK Installer's window works with a screen reader and the keyboard alone: its
/// progress bars have names, a new page puts the keyboard focus on its first control, and how
/// the install went is told to the screen reader.</summary>
public class Fr18ApkInstallerAccessibilityTests
{
    private static readonly XNamespace Wpf = "http://schemas.microsoft.com/winfx/2006/xaml/presentation";
    private static readonly XNamespace X = "http://schemas.microsoft.com/winfx/2006/xaml";
    private const string Path = @"C:\Users\me\Downloads\notes.apk";

    private static string Find(string file)
    {
        // The tests run from dsa/target/settings/..., outside the source tree, so each folder
        // above is tried both as the settings folder and as the repository's dsa folder.
        string[] relative = { System.IO.Path.Combine("DsaApkInstaller", file),
            System.IO.Path.Combine("host", "platform", "windows", "settings", "DsaApkInstaller", file) };
        for (var dir = new DirectoryInfo(AppContext.BaseDirectory); dir is not null; dir = dir.Parent)
        {
            var found = relative.Select(r => System.IO.Path.Combine(dir.FullName, r)).FirstOrDefault(File.Exists);
            if (found is not null)
            {
                return found;
            }
        }
        throw new FileNotFoundException(file + " was not found above " + AppContext.BaseDirectory);
    }

    private static XDocument Window() => XDocument.Load(Find("InstallerWindow.xaml"));

    private static string Code() => File.ReadAllText(Find("InstallerWindow.xaml.cs"));

    private static string? Name(XElement e) => (string?)e.Attribute(X + "Name");

    /// <summary>Whether the window gives the control a name: in its XAML, or in its code.</summary>
    private static bool IsNamed(XElement control, string code, string nameProperty = "SetName")
    {
        if (control.Attribute(XName.Get("AutomationProperties.Name")) is not null)
        {
            return true;
        }
        var name = Name(control);
        return name is not null && Regex.IsMatch(code, $@"AutomationProperties\.{nameProperty}\(\s*{Regex.Escape(name)}\s*,");
    }

    [Fact]
    public void Fr18_apk_installer_progress_has_a_name()
    {
        var code = Code();
        var bars = Window().Descendants(Wpf + "ProgressBar").ToList();
        // The bar while the file is read, and the install's bar.
        Assert.Equal(2, bars.Count);
        foreach (var bar in bars)
        {
            Assert.True(Name(bar) is not null, "a progress bar has no x:Name, so nothing can name it");
            Assert.True(IsNamed(bar, code), Name(bar) + " has no UI Automation name");
        }
        // The install's bar is named for the step it measures, the reading bar for reading.
        Assert.Contains("SetName(Progress,", code);
        Assert.Contains("_vm.ProgressTitle", code);
        Assert.Contains("SetName(ReadingProgress, ReadingText.Text)", code);
    }

    [Fact]
    public void Fr18_apk_installer_controls_and_the_app_icon_have_names()
    {
        var code = Code();
        var doc = Window();
        // Buttons, the link and the checkbox are named by the words they show, which the code
        // puts in them; none is an icon alone.
        var controls = doc.Descendants().Where(e => e.Name.LocalName is "Button" or "CheckBox" or "ExpandLink").ToList();
        Assert.NotEmpty(controls);
        foreach (var control in controls)
        {
            var name = Name(control)!;
            Assert.True(control.Attribute("Content") is not null || code.Contains(name + ".Content =") || IsNamed(control, code),
                name + " shows no words and has no name");
        }
        // The app's icon is the one image, and it says whose it is.
        var images = doc.Descendants(Wpf + "Image").ToList();
        Assert.Single(images);
        Assert.True(IsNamed(images[0], code), "the app icon has no UI Automation name");
    }

    private sealed record Control(string Id, bool Visible = true, bool Enabled = true, bool Main = false);

    private static Control? First(params Control[] buttons) =>
        ApkScreenReader.FirstControl(buttons, b => b.Visible && b.Enabled, b => b.Main);

    [Fact]
    public void Fr18_apk_installer_page_change_moves_focus_to_first_control()
    {
        // The details page: Install is the main button, before Cancel.
        Assert.Equal("Install", First(new("Install", Main: true), new("Open", Visible: false), new("Cancel"), new("Close", Visible: false))?.Id);
        // Reading the file, or a blocked app: Install is off, so the focus goes to Cancel.
        Assert.Equal("Cancel", First(new("Install", Enabled: false, Main: false), new("Open", Visible: false), new("Cancel"))?.Id);
        // Installed: Launch is the main button.
        Assert.Equal("Open", First(new("Install", Visible: false), new("Open", Main: true), new("Cancel", Visible: false), new("Close"))?.Id);
        // Not installed: Close is the main button and the only one.
        Assert.Equal("Close", First(new("Install", Visible: false), new("Open", Visible: false), new("Cancel", Visible: false), new("Close", Main: true))?.Id);
        // A hidden or disabled main button is passed over.
        Assert.Equal("Cancel", First(new("Install", Enabled: false, Main: true), new("Cancel"))?.Id);
        // Nothing to focus.
        Assert.Null(First(new("Install", Visible: false), new("Cancel", Enabled: false)));
    }

    [Fact]
    public void Fr18_apk_installer_window_moves_focus_when_the_page_changes()
    {
        var code = Code();
        // Each new page, and only a new page, moves the focus, after it is laid out.
        Assert.Contains("if (_focusedPage != page)", code);
        Assert.Contains("Dispatcher.InvokeAsync(FocusFirstControl", code);
        // The candidates are the footer's buttons, in their order, and the main one is the one
        // Enter presses.
        Assert.Contains("ScreenReader.FirstControl(", code);
        Assert.Contains("Buttons.Children.OfType<System.Windows.Controls.Button>()", code);
        Assert.Contains("b => b.IsDefault", code);
        Assert.Contains("first.Focus()", code);
    }

    [Fact]
    public void Fr18_completion_and_failure_are_announced_as_sentences()
    {
        Assert.Equal("Notes is installed. Launch it now, or later from the Start menu.",
            ApkScreenReader.Announcement("Notes is installed", "Launch it now, or later from the Start menu."));
        Assert.Equal("Not installed. Android has no room for the app.",
            ApkScreenReader.Announcement("Not installed", "  Android has no room for the app  "));
        Assert.Equal("Notes is updated.", ApkScreenReader.Announcement("Notes is updated", "", null));
        // The words are the window's own, in whatever language they are in.
        Assert.Equal("메모가 설치되었습니다. 지금 실행하세요.", ApkScreenReader.Announcement("메모가 설치되었습니다", "지금 실행하세요"));
        Assert.Equal("Why not? Ask Android!", ApkScreenReader.Announcement("Why not?", "Ask Android!"));
    }

    /// <summary>What dsa apk-info prints of the notes APK with Android running.</summary>
    private const string Info =
        "{\"package\":\"com.example.notes\",\"label\":\"Notes\",\"version_code\":42,\"version_name\":\"4.2\"," +
        "\"min_sdk\":26,\"min_android\":\"8.0\",\"target_sdk\":34,\"target_android\":\"14\",\"size\":3145728," +
        "\"permissions\":[],\"signer\":{\"scheme\":\"v2\",\"sha256\":\"AB:CD\"},\"abis\":[\"x86_64\"]," +
        "\"android\":\"running\",\"installed\":null,\"install\":\"new\"}";

    [Fact]
    public async Task Fr18_completion_and_failure_are_announced_with_the_result_the_window_shows()
    {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
        var x64 = new[] { "x86_64", "x86" };

        var done = new ApkInstallViewModel(new FakeRunner()
            .Answer($"apk-info {Path} --json", Info)
            .Answer($"install {Path} --progress --json", "{\"installed\":true}"), new ArgsText(), Path, x64) { LaunchWhenReady = false };
        await done.LoadAsync();
        await done.InstallAsync();
        Assert.Equal(ApkInstallPage.Done, done.Page);
        var told = ApkScreenReader.Announcement(done.ResultTitle, done.ResultText);
        Assert.StartsWith("ApkWindow_DoneTitle Notes.", told);
        Assert.Contains("ApkWindow_DoneText", told);

        var failed = new ApkInstallViewModel(new FakeRunner()
            .Answer($"apk-info {Path} --json", Info)
            .Fail($"install {Path} --progress --json",
                "dsa: not installed (insufficient-storage): notes.apk: no room"), new ArgsText(), Path, x64) { LaunchWhenReady = false };
        await failed.LoadAsync();
        await failed.InstallAsync();
        Assert.Equal(ApkInstallPage.Failed, failed.Page);
        told = ApkScreenReader.Announcement(failed.ResultTitle, failed.ResultText);
        Assert.StartsWith("ApkWindow_FailedTitle", told);
        Assert.Contains("ApkWindow_Reason_insufficient-storage", told);
    }

    [Fact]
    public void Fr18_completion_and_failure_are_announced()
    {
        var code = Code();
        // The done and failed pages tell the screen reader their title and text, once each.
        Assert.Contains("page is ApkInstallPage.Done or ApkInstallPage.Failed", code);
        Assert.Contains("ScreenReader.Announcement(_vm.ResultTitle, _vm.ResultText)", code);
        Assert.Contains("if (words != _announced)", code);
        // With a UI Automation notification that interrupts what is being read.
        Assert.Contains("RaiseNotificationEvent(", code);
        Assert.Contains("AutomationNotificationKind.ActionCompleted", code);
        Assert.Contains("AutomationNotificationProcessing.ImportantMostRecent", code);
        // An app that installed but did not open says why, too.
        var open = code[code.IndexOf("private async void OnOpen", StringComparison.Ordinal)..];
        open = open[..open.IndexOf("private void OnCancel", StringComparison.Ordinal)];
        Assert.Contains("Announce(why)", open);
    }
}
