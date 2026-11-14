using System.Globalization;
using Xunit;

namespace Dsa.Settings.Core.Tests;

/// <summary>The window settings of each app on the Apps page: whether its windows may be
/// resized, and the size they open at, kept by the service through dsa app-window.</summary>
public class AppWindowSettingsTests
{
    private const string List =
        "{\"apps\":[" +
        "{\"package\":\"com.example.notes\",\"label\":\"Notes\",\"version_name\":\"4.1\",\"version_code\":41," +
        "\"first_install_time_ms\":0,\"last_update_time_ms\":0,\"system_app\":false," +
        "\"app_bytes\":0,\"data_bytes\":0,\"cache_bytes\":0,\"total_bytes\":0,\"icon_png\":\"\"}," +
        "{\"package\":\"com.example.reader\",\"label\":\"Reader\",\"version_name\":\"1\",\"version_code\":1," +
        "\"first_install_time_ms\":0,\"last_update_time_ms\":0,\"system_app\":false," +
        "\"app_bytes\":0,\"data_bytes\":0,\"cache_bytes\":0,\"total_bytes\":0,\"icon_png\":\"\"}]}";

    private const string Windows =
        "{\"apps\":[{\"package\":\"com.example.reader\",\"fixed_size\":true,\"start_size\":[412,915]}]}";

    public AppWindowSettingsTests()
    {
        CultureInfo.CurrentCulture = CultureInfo.InvariantCulture;
    }

    private static AppsViewModel Make(FakeCli cli) => new(new AppsService(cli), new AppsText());

    [Fact]
    public async Task Fr30_each_app_shows_whether_its_windows_resize_and_the_size_they_open_at()
    {
        var vm = Make(new FakeCli()
            .Answer("apps list --json", List)
            .Answer("app-window --json", Windows));
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        Assert.False(vm.HasError, vm.Error);
        var notes = vm.Rows.Single(r => r.Package == "com.example.notes");
        Assert.True(notes.Resizable);
        Assert.Equal(0, notes.StartSizeIndex);
        var reader = vm.Rows.Single(r => r.Package == "com.example.reader");
        Assert.False(reader.Resizable);
        Assert.Equal(412, vm.StartSizeChoices[reader.StartSizeIndex].Width);
        Assert.Equal(915, vm.StartSizeChoices[reader.StartSizeIndex].Height);
    }

    [Fact]
    public async Task Fr30_the_user_makes_an_apps_windows_fixed_in_size_and_opens_them_as_a_tablet()
    {
        var cli = new FakeCli()
            .Answer("apps list --json", List)
            .Answer("app-window --json", "{\"apps\":[]}")
            .Answer("app-window com.example.notes --fixed-size on --start-size auto --dpi-scaling auto --fullscreen default --immersive default --json",
                "{\"apps\":[{\"package\":\"com.example.notes\",\"fixed_size\":true,\"start_size\":null}]}")
            .Answer("app-window com.example.notes --fixed-size on --start-size 1280x800 --dpi-scaling auto --fullscreen default --immersive default --json",
                "{\"apps\":[{\"package\":\"com.example.notes\",\"fixed_size\":true,\"start_size\":[1280,800]}]}");
        var vm = Make(cli);
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        await vm.SetResizableAsync("com.example.notes", false);
        Assert.False(vm.HasError, vm.Error);
        Assert.Equal("Apps_WindowChanged", vm.Message);
        Assert.False(vm.Rows.Single(r => r.Package == "com.example.notes").Resizable);

        var tablet = vm.StartSizeChoices.ToList().FindIndex(c => c.Width == 1280 && c.Height == 800);
        await vm.SetStartSizeAsync("com.example.notes", tablet);
        Assert.False(vm.HasError, vm.Error);
        Assert.Equal(tablet, vm.Rows.Single(r => r.Package == "com.example.notes").StartSizeIndex);
        // Choosing the size the app has already sends nothing.
        var calls = cli.Calls.Count;
        await vm.SetStartSizeAsync("com.example.notes", tablet);
        Assert.Equal(calls, cli.Calls.Count);
    }

    [Fact]
    public async Task Fr30_a_service_that_keeps_no_window_settings_leaves_every_app_at_the_defaults()
    {
        var vm = Make(new FakeCli()
            .Answer("apps list --json", List)
            .Fail("app-window --json", "dsa: unexpected \"app-window\""));
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        Assert.False(vm.HasError, vm.Error);
        Assert.All(vm.Rows, row => Assert.True(row.Resizable));
    }

    [Fact]
    public async Task Fr4_dpi_scaling_choice_is_read_and_sent()
    {
        var cli = new FakeCli()
            .Answer("apps list --json", List)
            .Answer("app-window --json",
                "{\"apps\":[{\"package\":\"com.example.reader\",\"fixed_size\":false,\"start_size\":null,\"dpi_scaling\":\"system\"}]," +
                "\"system_forced_available\":false}")
            .Answer("app-window com.example.notes --fixed-size off --start-size auto --dpi-scaling app --fullscreen default --immersive default --json",
                "{\"apps\":[{\"package\":\"com.example.notes\",\"fixed_size\":false,\"start_size\":null,\"dpi_scaling\":\"app\"}]}");
        var vm = Make(cli);
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        Assert.False(vm.HasError, vm.Error);
        // An app without settings, and one dsa names nothing for, follow the automatic choice.
        var notes = vm.Rows.Single(r => r.Package == "com.example.notes");
        Assert.Equal(DpiScaling.Auto, vm.DpiScalingChoices[notes.DpiScalingIndex].Scaling);
        var reader = vm.Rows.Single(r => r.Package == "com.example.reader");
        Assert.Equal(DpiScaling.System, vm.DpiScalingChoices[reader.DpiScalingIndex].Scaling);

        var app = vm.DpiScalingChoices.ToList().FindIndex(c => c.Scaling == DpiScaling.App);
        await vm.SetDpiScalingAsync("com.example.notes", app);
        Assert.False(vm.HasError, vm.Error);
        Assert.Equal("Apps_WindowChanged", vm.Message);
        Assert.Equal(app, vm.Rows.Single(r => r.Package == "com.example.notes").DpiScalingIndex);
        // Choosing what the app has already sends nothing.
        var calls = cli.Calls.Count;
        await vm.SetDpiScalingAsync("com.example.notes", app);
        Assert.Equal(calls, cli.Calls.Count);
    }

    [Fact]
    public async Task Fr4_system_forced_is_marked_not_available_yet()
    {
        var cli = new FakeCli()
            .Answer("apps list --json", List)
            .Answer("app-window --json",
                "{\"apps\":[{\"package\":\"com.example.reader\",\"fixed_size\":false,\"start_size\":null,\"dpi_scaling\":\"system-forced\"}]}");
        var vm = Make(cli);
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        Assert.False(vm.HasError, vm.Error);
        // A dsa that does not say whether Android can do it counts as one that cannot yet.
        Assert.False(vm.SystemForcedAvailable);
        var forced = vm.DpiScalingChoices.Single(c => c.Scaling == DpiScaling.SystemForced);
        Assert.False(forced.Available);
        Assert.Equal("Apps_DpiScaling_SystemForcedNotYet", forced.Label);
        Assert.All(vm.DpiScalingChoices.Where(c => c.Scaling != DpiScaling.SystemForced), c => Assert.True(c.Available));
        // It is still the app's stored choice, and the page says why it works like System.
        var reader = vm.Rows.Single(r => r.Package == "com.example.reader");
        Assert.Equal(DpiScaling.SystemForced, vm.DpiScalingChoices[reader.DpiScalingIndex].Scaling);
        Assert.Equal("Apps_DpiScaling_SystemForcedTip", reader.DpiScalingTip);
        Assert.Null(vm.Rows.Single(r => r.Package == "com.example.notes").DpiScalingTip);
    }

    [Fact]
    public async Task Fr4_system_forced_is_offered_plainly_once_android_can_do_it()
    {
        var vm = Make(new FakeCli()
            .Answer("apps list --json", List)
            .Answer("app-window --json", "{\"apps\":[],\"system_forced_available\":true}"));
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        var forced = vm.DpiScalingChoices.Single(c => c.Scaling == DpiScaling.SystemForced);
        Assert.True(forced.Available);
        Assert.Equal("Apps_DpiScaling_SystemForced", forced.Label);
    }

    [Fact]
    public async Task Fr4_youtube_scales_the_system_way_by_default()
    {
        var list = List.Replace("com.example.reader", "com.google.android.youtube");
        var vm = Make(new FakeCli()
            .Answer("apps list --json", list)
            .Answer("app-window --json",
                "{\"apps\":[],\"system_forced_available\":false," +
                "\"system_scaled_by_default\":[\"com.google.android.youtube\"]}"));
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        Assert.False(vm.HasError, vm.Error);
        // YouTube, with no choice of the user's, shows its automatic choice as System for it.
        var youtube = vm.Rows.Single(r => r.Package == "com.google.android.youtube");
        Assert.True(vm.SystemScaledByDefault(youtube.Package));
        var choices = youtube.DpiScalingChoices!;
        Assert.Equal(DpiScaling.Auto, choices[youtube.DpiScalingIndex].Scaling);
        Assert.Equal("Apps_DpiScaling_AutoSystem", choices[youtube.DpiScalingIndex].Label);
        // Any other app's automatic choice keeps its plain name.
        var notes = vm.Rows.Single(r => r.Package == "com.example.notes");
        Assert.False(vm.SystemScaledByDefault(notes.Package));
        Assert.Equal("Apps_DpiScaling_Auto", notes.DpiScalingChoices![notes.DpiScalingIndex].Label);
        // The other choices read the same for both.
        Assert.Equal(notes.DpiScalingChoices!.Skip(1), choices.Skip(1));
    }

    [Fact]
    public async Task Fr4_the_users_choice_wins_over_the_built_in_list()
    {
        var list = List.Replace("com.example.reader", "com.google.android.youtube");
        var cli = new FakeCli()
            .Answer("apps list --json", list)
            .Answer("app-window --json",
                "{\"apps\":[],\"system_scaled_by_default\":[\"com.google.android.youtube\"]}")
            .Answer("app-window com.google.android.youtube --fixed-size off --start-size auto --dpi-scaling app --fullscreen default --immersive default --json",
                "{\"apps\":[{\"package\":\"com.google.android.youtube\",\"fixed_size\":false,\"start_size\":null,\"dpi_scaling\":\"app\"}]," +
                "\"system_scaled_by_default\":[\"com.google.android.youtube\"]}");
        var vm = Make(cli);
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        var app = vm.DpiScalingChoices.ToList().FindIndex(c => c.Scaling == DpiScaling.App);
        await vm.SetDpiScalingAsync("com.google.android.youtube", app);
        Assert.False(vm.HasError, vm.Error);
        var youtube = vm.Rows.Single(r => r.Package == "com.google.android.youtube");
        Assert.Equal(DpiScaling.App, youtube.DpiScalingChoices![youtube.DpiScalingIndex].Scaling);
    }

    [Fact]
    public async Task Fr4_a_dsa_that_names_no_built_in_list_scales_every_app_its_own_way()
    {
        var vm = Make(new FakeCli()
            .Answer("apps list --json", List)
            .Answer("app-window --json", "{\"apps\":[]}"));
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        Assert.All(vm.Rows, row => Assert.Equal("Apps_DpiScaling_Auto", row.DpiScalingChoices![0].Label));
    }

    [Fact]
    public void Fr4_dpi_scaling_is_passed_to_dsa_as_it_takes_it()
    {
        Assert.Equal(new[] { "auto", "app", "system", "system-forced" }, DpiScalings.All.Select(DpiScalings.Code));
        Assert.All(DpiScalings.All, s => Assert.Equal(s, DpiScalings.Parse(DpiScalings.Code(s))));
        Assert.Equal(DpiScaling.Auto, DpiScalings.Parse(null));
        Assert.Equal(DpiScaling.Auto, DpiScalings.Parse("enhanced"));
    }

    [Fact]
    public async Task Fr30_an_apps_full_screen_choice_wins_over_the_default()
    {
        var cli = new FakeCli()
            .Answer("apps list --json", List)
            .Answer("app-window --json",
                "{\"apps\":[{\"package\":\"com.example.reader\",\"fixed_size\":false,\"start_size\":null," +
                "\"dpi_scaling\":\"auto\",\"fullscreen\":\"window\",\"immersive\":\"default\"}]," +
                "\"defaults\":{\"fullscreen\":\"screen\",\"immersive\":\"window\"}}")
            .Answer("app-window com.example.notes --fixed-size off --start-size auto --dpi-scaling auto --fullscreen default --immersive screen --json",
                "{\"apps\":[{\"package\":\"com.example.notes\",\"fixed_size\":false,\"start_size\":null," +
                "\"dpi_scaling\":\"auto\",\"fullscreen\":\"default\",\"immersive\":\"screen\"}]}");
        var vm = Make(cli);
        await vm.RefreshAsync();
        await vm.LoadWindowSettingsAsync();
        Assert.False(vm.HasError, vm.Error);
        // An app without its own choice follows the default; one with a choice shows it.
        var notes = vm.Rows.Single(r => r.Package == "com.example.notes");
        Assert.Equal(FullScreenPlace.Default, vm.FullScreenChoices[notes.FullScreenIndex].Place);
        Assert.Equal(FullScreenPlace.Default, vm.FullScreenChoices[notes.ImmersiveIndex].Place);
        var reader = vm.Rows.Single(r => r.Package == "com.example.reader");
        Assert.Equal(FullScreenPlace.Window, vm.FullScreenChoices[reader.FullScreenIndex].Place);
        Assert.Equal(new[] { "Apps_FullScreen_Default", "Apps_FullScreen_Screen", "Apps_FullScreen_Window" },
            vm.FullScreenChoices.Select(c => c.Label));

        var screen = vm.FullScreenChoices.ToList().FindIndex(c => c.Place == FullScreenPlace.Screen);
        await vm.SetImmersiveAsync("com.example.notes", screen);
        Assert.False(vm.HasError, vm.Error);
        Assert.Equal(screen, vm.Rows.Single(r => r.Package == "com.example.notes").ImmersiveIndex);
        // Choosing what the app has already sends nothing.
        var calls = cli.Calls.Count;
        await vm.SetImmersiveAsync("com.example.notes", screen);
        Assert.Equal(calls, cli.Calls.Count);
    }

    [Fact]
    public void Fr30_full_screen_places_are_passed_to_dsa_as_it_takes_them()
    {
        Assert.Equal(new[] { "default", "screen", "window" }, FullScreenPlaces.All.Select(FullScreenPlaces.Code));
        Assert.All(FullScreenPlaces.All, p => Assert.Equal(p, FullScreenPlaces.Parse(FullScreenPlaces.Code(p))));
        Assert.Equal(FullScreenPlace.Default, FullScreenPlaces.Parse(null));
        Assert.Equal(FullScreenPlace.Window, FullScreenPlaces.Parse("monitor", FullScreenPlace.Window));
    }

    [Fact]
    public void Fr30_a_start_size_is_passed_to_dsa_as_it_takes_it()
    {
        Assert.Equal("auto", AppWindowSetting.Default("com.example.notes").StartSizeArgument);
        Assert.Equal("412x915", new AppWindowSetting("com.example.notes", false, 412, 915).StartSizeArgument);
    }
}
