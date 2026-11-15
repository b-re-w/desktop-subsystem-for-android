using System.Windows;
using System.Windows.Controls;
using Dsa.Settings.Core;

namespace Dsa.Settings.Pages;

public partial class DisplayPage : Page
{
    private static SettingsViewModel ViewModel => App.Settings;

    public DisplayPage()
    {
        InitializeComponent();
        DataContext = ViewModel;
        Loaded += async (_, _) => await ViewModel.LoadAsync();
    }

    private async void OnGpuChosen(object sender, SelectionChangedEventArgs e)
    {
        // The list is rebuilt whenever the settings change, which also lands here; only a
        // choice that differs from the setting is sent.
        if (GpuBox.SelectedItem is GpuChoice choice && ViewModel.Loaded && !ViewModel.IsBusy)
        {
            await ViewModel.SetRenderGpuAsync(choice.Value);
        }
    }

    private async void OnVulkanDriverChosen(object sender, SelectionChangedEventArgs e)
    {
        // The list is set again whenever the settings change; only the user's choice is sent.
        if (VulkanDriverBox.SelectedIndex >= 0 && ViewModel.Loaded && !ViewModel.IsBusy)
        {
            await ViewModel.SetVulkanDriverAsync(VulkanDriverBox.SelectedIndex);
        }
    }

    /// <summary>Opens the Microsoft Store at the Compatibility Pack, which carries Dozen.</summary>
    private void OnOpenCompatibilityPack(object sender, RoutedEventArgs e)
    {
        try
        {
            System.Diagnostics.Process.Start(
                new System.Diagnostics.ProcessStartInfo(SettingsViewModel.CompatibilityPackStoreUri) { UseShellExecute = true });
        }
        catch (System.ComponentModel.Win32Exception)
        {
            // No Store on this computer: the text names the winget id too.
        }
    }

    private async void OnAppFullScreen(object sender, SelectionChangedEventArgs e)
    {
        // The list is set again whenever the settings change; only the user's choice is sent.
        if (AppFullScreenBox.SelectedIndex >= 0 && ViewModel.Loaded && !ViewModel.IsBusy)
        {
            await ViewModel.SetAppFullScreenAsync(AppFullScreenBox.SelectedIndex);
        }
    }

    private async void OnAppImmersive(object sender, SelectionChangedEventArgs e)
    {
        if (AppImmersiveBox.SelectedIndex >= 0 && ViewModel.Loaded && !ViewModel.IsBusy)
        {
            await ViewModel.SetAppImmersiveAsync(AppImmersiveBox.SelectedIndex);
        }
    }

    private async void OnTouchGesturesToggled(object sender, RoutedEventArgs e) =>
        await ViewModel.SetTouchGesturesOffAsync(TouchGesturesSwitch.IsChecked == true);

    /// <summary>Opens Windows Settings at its touch page, where the gestures are changed for
    /// every app.</summary>
    private void OnOpenTouchSettings(object sender, RoutedEventArgs e)
    {
        try
        {
            System.Diagnostics.Process.Start(
                new System.Diagnostics.ProcessStartInfo(SettingsViewModel.TouchSettingsUri) { UseShellExecute = true });
        }
        catch (System.ComponentModel.Win32Exception)
        {
            // No Settings app opens that page on this computer, so nothing opens.
        }
    }

    private async void OnRestartWindowManager(object sender, RoutedEventArgs e)
    {
        var confirmed = await Shell.Confirm(App.Text.Get("RestartWindowManagerTitle"),
            App.Text.Get("RestartWindowManagerBody"), App.Text.Get("RestartWindowManagerAction"));
        if (!confirmed)
        {
            return;
        }
        var problem = await Shell.RestartWindowManager();
        if (problem is not null)
        {
            await Shell.Tell(App.Text.Get("RestartWindowManagerFailed"), problem);
        }
    }
}
