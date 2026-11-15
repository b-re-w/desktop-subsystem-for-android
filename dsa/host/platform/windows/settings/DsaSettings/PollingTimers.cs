using System.Windows;
using System.Windows.Controls;
using System.Windows.Threading;
using Dsa.Settings.Core;

namespace Dsa.Settings;

/// <summary>Runs a page's polling timers only while the page is loaded and its window is shown
/// and not minimised (<see cref="PagePolling"/>). Each tick starts a <c>dsa</c> process, so a
/// minimised settings window kept polling every few seconds for nobody.</summary>
internal static class PollingTimers
{
    public static void Attach(Page page, params DispatcherTimer[] timers)
    {
        Window? window = null;
        var loaded = false;

        void Update()
        {
            var run = PagePolling.ShouldPoll(loaded, window?.IsVisible ?? false,
                window?.WindowState == WindowState.Minimized);
            foreach (var timer in timers)
            {
                if (run && !timer.IsEnabled)
                {
                    timer.Start();
                }
                else if (!run)
                {
                    timer.Stop();
                }
            }
        }

        EventHandler stateChanged = (_, _) => Update();
        DependencyPropertyChangedEventHandler visibleChanged = (_, _) => Update();
        page.Loaded += (_, _) =>
        {
            loaded = true;
            window = Window.GetWindow(page);
            if (window is not null)
            {
                window.StateChanged += stateChanged;
                window.IsVisibleChanged += visibleChanged;
            }
            Update();
        };
        page.Unloaded += (_, _) =>
        {
            loaded = false;
            if (window is not null)
            {
                window.StateChanged -= stateChanged;
                window.IsVisibleChanged -= visibleChanged;
            }
            Update();
            window = null;
        };
    }
}
