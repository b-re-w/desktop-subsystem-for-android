namespace Dsa.Settings.Core;

/// <summary>The Display page's choices of where an app's full screen shows, for every app
/// without its own: a clear request (a video's full screen button) on the whole screen by
/// default, and an app hiding both its bars alone inside its window by default. An app's own
/// choice on the Apps page wins over them.</summary>
public sealed partial class SettingsViewModel
{
    /// <summary>The two places an app's full screen can show for every app, in the order of
    /// <see cref="AppFullScreenPlaces"/>.</summary>
    public IReadOnlyList<FullScreenChoice> AppFullScreenChoices => AppFullScreenPlaces
        .Select(place => new FullScreenChoice(_text.Get($"Display_FullScreen_{place}"), place))
        .ToList();

    /// <summary>The places the choice for every app offers: it is the default itself.</summary>
    public static readonly IReadOnlyList<FullScreenPlace> AppFullScreenPlaces =
        new[] { FullScreenPlace.Screen, FullScreenPlace.Window };

    /// <summary>Where a clear full screen request shows now, among <see cref="AppFullScreenChoices"/>.</summary>
    public int AppFullScreenIndex =>
        IndexOf(_settings?.AppFullScreen ?? FullScreenPlace.Screen, FullScreenPlace.Screen);

    /// <summary>Where an app hiding both its bars alone shows now.</summary>
    public int AppImmersiveIndex =>
        IndexOf(_settings?.AppImmersive ?? FullScreenPlace.Window, FullScreenPlace.Window);

    private static int IndexOf(FullScreenPlace place, FullScreenPlace builtin)
    {
        var index = AppFullScreenPlaces.ToList().IndexOf(place);
        return index >= 0 ? index : AppFullScreenPlaces.ToList().IndexOf(builtin);
    }

    /// <summary>Shows every app's clear full screen request at <paramref name="index"/> of
    /// <see cref="AppFullScreenChoices"/>. The windows open already follow it at once.</summary>
    public Task SetAppFullScreenAsync(int index)
    {
        if (index < 0 || index >= AppFullScreenPlaces.Count || (index == AppFullScreenIndex && Loaded))
        {
            return Task.CompletedTask;
        }
        var place = AppFullScreenPlaces[index];
        return ChangeAsync(() => _service.SetAppFullScreenAsync(place));
    }

    /// <summary>Shows every app that hides both its bars alone at <paramref name="index"/> of
    /// <see cref="AppFullScreenChoices"/>.</summary>
    public Task SetAppImmersiveAsync(int index)
    {
        if (index < 0 || index >= AppFullScreenPlaces.Count || (index == AppImmersiveIndex && Loaded))
        {
            return Task.CompletedTask;
        }
        var place = AppFullScreenPlaces[index];
        return ChangeAsync(() => _service.SetAppImmersiveAsync(place));
    }
}
