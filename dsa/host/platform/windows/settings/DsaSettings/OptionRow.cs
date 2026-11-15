using System.Windows;
using System.Windows.Controls;

namespace Dsa.Settings;

/// <summary>One setting in a card that opens, as the Windows Settings app lays one out: its name,
/// with the line that says what it does under it, on the left, and its control against the right
/// edge, centred on the text. Where the row is too narrow for both side by side (a narrow window,
/// a large text scale, a long list item), the control goes under the text, as WinUI's settings
/// card does below its breakpoint, so the name never shrinks to a word a line. The first child
/// is the text, the second the control. A panel has no automation peer, so a screen reader
/// reaches the control directly, with the name it gives itself.</summary>
public sealed class OptionRow : Panel
{
    /// <summary>The space between the text and the control beside it, as in WinUI's settings card.</summary>
    private const double Gap = 24;

    /// <summary>The space between the text and the control under it.</summary>
    private const double StackedGap = 8;

    /// <summary>The narrowest the text may be beside its control.</summary>
    private const double MinTextWidth = 200;

    /// <summary>The width the control asks for when nothing limits it.</summary>
    private double _controlWidth;

    private UIElement? Text => InternalChildren.Count > 0 ? InternalChildren[0] : null;

    private UIElement? Control => InternalChildren.Count > 1 ? InternalChildren[1] : null;

    private static bool SideBySide(double width, double control) =>
        double.IsInfinity(width) || width - control - Gap >= MinTextWidth;

    protected override Size MeasureOverride(Size available)
    {
        if (Text is not { } text)
        {
            return default;
        }
        if (Control is not { } control)
        {
            text.Measure(available);
            return text.DesiredSize;
        }
        control.Measure(new Size(double.PositiveInfinity, available.Height));
        _controlWidth = control.DesiredSize.Width;
        if (SideBySide(available.Width, _controlWidth))
        {
            var room = double.IsInfinity(available.Width)
                ? double.PositiveInfinity
                : available.Width - _controlWidth - Gap;
            text.Measure(new Size(room, available.Height));
            var width = double.IsInfinity(available.Width)
                ? text.DesiredSize.Width + Gap + _controlWidth
                : available.Width;
            return new Size(width, Math.Max(text.DesiredSize.Height, control.DesiredSize.Height));
        }
        text.Measure(available);
        control.Measure(available);
        return new Size(available.Width, text.DesiredSize.Height + StackedGap + control.DesiredSize.Height);
    }

    protected override Size ArrangeOverride(Size final)
    {
        if (Text is not { } text)
        {
            return final;
        }
        if (Control is not { } control)
        {
            text.Arrange(new Rect(final));
            return final;
        }
        if (SideBySide(final.Width, _controlWidth))
        {
            var controlWidth = Math.Min(_controlWidth, final.Width);
            var textWidth = Math.Max(0, final.Width - controlWidth - Gap);
            var textHeight = text.DesiredSize.Height;
            var controlHeight = control.DesiredSize.Height;
            text.Arrange(new Rect(0, Math.Max(0, (final.Height - textHeight) / 2), textWidth, textHeight));
            control.Arrange(new Rect(final.Width - controlWidth, Math.Max(0, (final.Height - controlHeight) / 2),
                controlWidth, controlHeight));
        }
        else
        {
            text.Arrange(new Rect(0, 0, final.Width, text.DesiredSize.Height));
            control.Arrange(new Rect(0, text.DesiredSize.Height + StackedGap,
                Math.Min(control.DesiredSize.Width, final.Width), control.DesiredSize.Height));
        }
        return final;
    }
}
