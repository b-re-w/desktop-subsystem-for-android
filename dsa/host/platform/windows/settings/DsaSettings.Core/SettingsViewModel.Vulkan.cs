using System.Globalization;

namespace Dsa.Settings.Core;

/// <summary>The Display page's Vulkan driver: a native driver, or Dozen (Vulkan on Direct3D 12)
/// from Microsoft's Compatibility Pack where no GPU has one, and what the renderer runs on
/// now.</summary>
public sealed partial class SettingsViewModel
{
    /// <summary>The values of the setting, in the order the list shows them.</summary>
    public static readonly IReadOnlyList<string> VulkanDrivers = new[] { "auto", "native", "dozen" };

    /// <summary>Where the Microsoft Store shows the Compatibility Pack.</summary>
    public const string CompatibilityPackStoreUri = "ms-windows-store://pdp/?ProductId=9NQPSL29BFFF";

    /// <summary>The pack's winget id, for the command line.</summary>
    public const string CompatibilityPackWingetId = "Microsoft.OpenCLGLVulkanCompatibilityPack";

    public string VulkanDriver => _settings?.VulkanDriver ?? "auto";

    /// <summary>The three choices, in the user's language.</summary>
    public IReadOnlyList<GpuChoice> VulkanDriverChoices => new[]
    {
        new GpuChoice(_text.Get("Vulkan_Auto"), "auto"),
        new GpuChoice(_text.Get("Vulkan_Native"), "native"),
        new GpuChoice(_text.Get("Vulkan_Dozen"), "dozen"),
    };

    public int VulkanDriverIndex
    {
        get
        {
            for (var i = 0; i < VulkanDrivers.Count; i++)
            {
                if (string.Equals(VulkanDrivers[i], VulkanDriver, StringComparison.OrdinalIgnoreCase))
                {
                    return i;
                }
            }
            return 0;
        }
    }

    public GraphicsState Graphics => _settings?.Graphics ?? GraphicsState.Unknown;

    /// <summary>What the renderer runs on now, in the user's language, with the window
    /// manager's own words after it.</summary>
    public string GraphicsStatusText
    {
        get
        {
            var key = Graphics switch
            {
                GraphicsState.Native => "Graphics_Native",
                GraphicsState.Dozen => "Graphics_Dozen",
                GraphicsState.DozenCpu => "Graphics_DozenCpu",
                GraphicsState.Unavailable => "Graphics_Unavailable",
                _ => "Graphics_Unknown",
            };
            var detail = _settings?.GraphicsDetail;
            return string.IsNullOrEmpty(detail)
                ? _text.Get(key)
                : string.Format(CultureInfo.CurrentCulture, _text.Get("Graphics_WithDetail"), _text.Get(key), detail);
        }
    }

    /// <summary>Whether the page offers the Compatibility Pack: nothing to draw on, or Dozen
    /// asked for.</summary>
    public bool OffersCompatibilityPack =>
        Graphics == GraphicsState.Unavailable || string.Equals(VulkanDriver, "dozen", StringComparison.OrdinalIgnoreCase);

    /// <summary>Whether the CPU draws, so the page warns that games are slow.</summary>
    public bool DrawsOnCpu => Graphics == GraphicsState.DozenCpu;

    public Task SetVulkanDriverAsync(int index)
    {
        if (index < 0 || index >= VulkanDrivers.Count)
        {
            return Task.CompletedTask;
        }
        var value = VulkanDrivers[index];
        if (string.Equals(value, VulkanDriver, StringComparison.OrdinalIgnoreCase) && Loaded)
        {
            return Task.CompletedTask;
        }
        return ChangeAsync(() => _service.SetVulkanDriverAsync(value));
    }
}
