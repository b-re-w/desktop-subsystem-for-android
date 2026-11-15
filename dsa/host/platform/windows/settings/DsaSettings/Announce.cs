using System.ComponentModel;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Automation.Peers;
using System.Windows.Threading;
using Dsa.Settings.Core;
using InfoBar = Wpf.Ui.Controls.InfoBar;
using InfoBarSeverity = Wpf.Ui.Controls.InfoBarSeverity;

namespace Dsa.Settings;

/// <summary>Tells a screen reader what changed on the page, through a UI Automation
/// notification, as WinUI's InfoBar does. A notice bar opens or changes its words without moving
/// the keyboard focus, so a user who does not see the screen heard nothing when an error or a
/// result appeared. Every notice bar takes <c>Announce.Changes</c> from its style in
/// App.xaml: it reads out its title and message when it opens, and again when they change while
/// it is open. Bars already open when their page shows are read with the page, not announced.</summary>
public static class Announce
{
    public static readonly DependencyProperty ChangesProperty = DependencyProperty.RegisterAttached(
        "Changes", typeof(bool), typeof(Announce), new PropertyMetadata(false, OnChangesChanged));

    public static bool GetChanges(DependencyObject element) => (bool)element.GetValue(ChangesProperty);

    public static void SetChanges(DependencyObject element, bool value) => element.SetValue(ChangesProperty, value);

    /// <summary>What the bar last told, so a binding that sets the same words again is quiet.</summary>
    private static readonly DependencyProperty SaidProperty = DependencyProperty.RegisterAttached(
        "Said", typeof(string), typeof(Announce), new PropertyMetadata(null));

    /// <summary>Whether a check of the bar is already queued: its title, message and state
    /// change one binding at a time, and they are read once, together.</summary>
    private static readonly DependencyProperty QueuedProperty = DependencyProperty.RegisterAttached(
        "Queued", typeof(bool), typeof(Announce), new PropertyMetadata(false));

    private static readonly DependencyProperty[] Watched =
    {
        InfoBar.IsOpenProperty, InfoBar.TitleProperty, InfoBar.MessageProperty,
    };

    private static void OnChangesChanged(DependencyObject element, DependencyPropertyChangedEventArgs e)
    {
        if (element is not InfoBar bar)
        {
            return;
        }
        bar.Loaded -= OnLoaded;
        bar.Unloaded -= OnUnloaded;
        Watch(bar, false);
        if ((bool)e.NewValue)
        {
            bar.Loaded += OnLoaded;
            bar.Unloaded += OnUnloaded;
            if (bar.IsLoaded)
            {
                Start(bar);
            }
        }
    }

    private static void OnLoaded(object sender, RoutedEventArgs e)
    {
        if (sender is InfoBar bar)
        {
            Start(bar);
        }
    }

    // The watch is dropped while the bar is off screen: the property descriptors hold the bars
    // they watch, and a page that is navigated away from would otherwise never be freed.
    private static void OnUnloaded(object sender, RoutedEventArgs e)
    {
        if (sender is InfoBar bar)
        {
            Watch(bar, false);
        }
    }

    private static void Start(InfoBar bar)
    {
        bar.SetValue(SaidProperty, bar.IsOpen ? Words(bar) : null);
        Watch(bar, false);
        Watch(bar, true);
    }

    private static void Watch(InfoBar bar, bool on)
    {
        foreach (var property in Watched)
        {
            var descriptor = DependencyPropertyDescriptor.FromProperty(property, typeof(InfoBar));
            if (on)
            {
                descriptor.AddValueChanged(bar, OnBarChanged);
            }
            else
            {
                descriptor.RemoveValueChanged(bar, OnBarChanged);
            }
        }
    }

    private static void OnBarChanged(object? sender, EventArgs e)
    {
        if (sender is not InfoBar bar || (bool)bar.GetValue(QueuedProperty))
        {
            return;
        }
        bar.SetValue(QueuedProperty, true);
        bar.Dispatcher.InvokeAsync(() =>
        {
            bar.SetValue(QueuedProperty, false);
            Tell(bar);
        }, DispatcherPriority.Background);
    }

    private static void Tell(InfoBar bar)
    {
        if (!bar.IsLoaded || !bar.IsOpen)
        {
            // A bar that closes is quiet, and says its words again when it opens again.
            bar.SetValue(SaidProperty, null);
            return;
        }
        var words = Words(bar);
        if (words.Length == 0 || words == (string?)bar.GetValue(SaidProperty))
        {
            return;
        }
        bar.SetValue(SaidProperty, words);
        Say(bar, words, bar.Severity == InfoBarSeverity.Error);
    }

    private static string Words(InfoBar bar) => AccessibleText.Notice(App.Text, bar.Title, bar.Message);

    /// <summary>Has a screen reader say <paramref name="words"/>, as from
    /// <paramref name="source"/>. An important message (an error) is read at once, in place of
    /// what the reader was saying; any other waits its turn.</summary>
    public static void Say(UIElement source, string words, bool important = false)
    {
        if (string.IsNullOrWhiteSpace(words))
        {
            return;
        }
        // An element without a peer of its own speaks through its window's.
        var peer = UIElementAutomationPeer.FromElement(source) ?? UIElementAutomationPeer.CreatePeerForElement(source);
        if (peer is null && Window.GetWindow(source) is { } window)
        {
            peer = UIElementAutomationPeer.FromElement(window) ?? UIElementAutomationPeer.CreatePeerForElement(window);
        }
        peer?.RaiseNotificationEvent(
            AutomationNotificationKind.Other,
            important ? AutomationNotificationProcessing.ImportantMostRecent : AutomationNotificationProcessing.MostRecent,
            words,
            "DsaSettingsNotice");
    }
}
