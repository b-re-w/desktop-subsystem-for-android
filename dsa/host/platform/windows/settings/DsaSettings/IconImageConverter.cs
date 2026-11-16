using System.Globalization;
using System.IO;
using System.Runtime.CompilerServices;
using System.Windows;
using System.Windows.Data;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using Dsa.Settings.Core;

namespace Dsa.Settings;

/// <summary>Turns an app's PNG icon into an image decoded at the size it is shown
/// (<see cref="IconDecodeSize"/>), the shown size in device independent pixels given as the
/// converter parameter. Without it WPF decodes each icon at its own size, 256 px for most, and
/// again every time a list is rebuilt; here each icon is decoded once for each size.</summary>
public sealed class IconImageConverter : IValueConverter
{
    private static readonly ConditionalWeakTable<byte[], Dictionary<int, ImageSource>> Decoded = new();

    public object? Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is not byte[] { Length: > 0 } png)
        {
            return null;
        }
        var shown = double.TryParse(parameter as string, NumberStyles.Float, CultureInfo.InvariantCulture, out var dip)
            ? dip
            : 32;
        try
        {
            var pixels = IconDecodeSize.Pixels(shown, Scale(), SourceWidth(png));
            var sizes = Decoded.GetOrCreateValue(png);
            if (sizes.TryGetValue(pixels, out var image))
            {
                return image;
            }
            var bitmap = new BitmapImage();
            bitmap.BeginInit();
            bitmap.CacheOption = BitmapCacheOption.OnLoad;
            bitmap.DecodePixelWidth = pixels;
            bitmap.StreamSource = new MemoryStream(png);
            bitmap.EndInit();
            bitmap.Freeze();
            sizes[pixels] = bitmap;
            return bitmap;
        }
        catch (Exception e) when (e is NotSupportedException or IOException or ArgumentException or FileFormatException
                                      or InvalidOperationException)
        {
            // An icon Windows cannot decode shows as no icon, as before.
            return null;
        }
    }

    public object ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture) =>
        throw new NotSupportedException();

    /// <summary>The icon's own width, read from its header without decoding it.</summary>
    private static int SourceWidth(byte[] png)
    {
        using var stream = new MemoryStream(png);
        var decoder = BitmapDecoder.Create(stream, BitmapCreateOptions.DelayCreation, BitmapCacheOption.None);
        return decoder.Frames.Count > 0 ? decoder.Frames[0].PixelWidth : 0;
    }

    /// <summary>The main window's scale on its monitor; 1 before it is shown.</summary>
    private static double Scale() =>
        Application.Current?.MainWindow is { } window
        && PresentationSource.FromVisual(window)?.CompositionTarget is { } target
            ? target.TransformToDevice.M11
            : 1.0;
}
