using System.Globalization;
using System.Windows.Automation.Peers;
using System.Windows.Automation.Provider;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Markup;
using Dsa.Settings.Core;

namespace Dsa.Settings;

/// <summary>The name of a control in a list row, in XAML:
/// <c>AutomationProperties.Name="{l:ItemName AppsOpenButton.Content}"</c> names the button
/// "Open, Notes" in the row of Notes. A list repeats its buttons in every row, and a screen
/// reader that reads "Open" alone does not say whose it is. <see cref="Path"/> is the row's
/// property that names it (its <c>Label</c> unless said otherwise); with
/// <see cref="FromSelf"/> it is a property of the control itself, such as its <c>Tag</c>. A
/// button that several cards repeat, such as "Change as administrator", names its card with
/// <see cref="Item"/>, the key of the card's title.</summary>
[MarkupExtensionReturnType(typeof(object))]
public sealed class ItemNameExtension : MarkupExtension
{
    public ItemNameExtension()
    {
    }

    public ItemNameExtension(string key)
    {
        Key = key;
    }

    /// <summary>The text that says what the control does.</summary>
    [ConstructorArgument("key")]
    public string Key { get; set; } = "";

    /// <summary>The property that names the row.</summary>
    public string Path { get; set; } = "Label";

    /// <summary>Whether <see cref="Path"/> is a property of the control rather than of its row.</summary>
    public bool FromSelf { get; set; }

    /// <summary>The key of a fixed text that names the item, in place of a binding.</summary>
    public string Item { get; set; } = "";

    public override object ProvideValue(IServiceProvider serviceProvider)
    {
        if (Item.Length > 0)
        {
            return AccessibleText.ItemName(App.Text, Key, App.Text.Get(Item));
        }
        var binding = new Binding(Path)
        {
            Mode = BindingMode.OneWay,
            StringFormat = AccessibleText.ItemNameFormat(App.Text, Key),
        };
        if (FromSelf)
        {
            binding.RelativeSource = RelativeSource.Self;
        }
        return binding.ProvideValue(serviceProvider);
    }
}

/// <summary>What a screen reader reads for a row of the log: the time, the priority as a word,
/// the tag and the message, in the user's language.</summary>
public sealed class LogLineNameConverter : IValueConverter
{
    public object Convert(object value, Type targetType, object parameter, CultureInfo culture) =>
        value is LogLine line ? line.Spoken(App.Text) : value?.ToString() ?? "";

    public object ConvertBack(object value, Type targetType, object parameter, CultureInfo culture) =>
        Binding.DoNothing;
}

/// <summary>A thin strip that shows a step without a measure, with an indicator that slides
/// across it. A plain panel has no place in UI Automation's control view, so a screen reader
/// did not know the window was busy; this one reads as a progress bar with the name set on
/// it.</summary>
public class ProgressStrip : Grid
{
    protected override AutomationPeer OnCreateAutomationPeer() => new ProgressStripAutomationPeer(this);
}

/// <summary>A progress bar that has no measure: it reports no value, as an indeterminate WPF
/// progress bar does.</summary>
public sealed class ProgressStripAutomationPeer : FrameworkElementAutomationPeer, IRangeValueProvider
{
    public ProgressStripAutomationPeer(ProgressStrip owner) : base(owner)
    {
    }

    protected override string GetClassNameCore() => nameof(ProgressStrip);

    protected override AutomationControlType GetAutomationControlTypeCore() => AutomationControlType.ProgressBar;

    protected override bool IsControlElementCore() => true;

    protected override bool IsContentElementCore() => true;

    public override object GetPattern(PatternInterface patternInterface) =>
        patternInterface == PatternInterface.RangeValue ? this : base.GetPattern(patternInterface);

    // A list of children would expose the strip's two drawn borders as unnamed parts.
    protected override List<AutomationPeer>? GetChildrenCore() => null;

    double IRangeValueProvider.Value => 0;

    bool IRangeValueProvider.IsReadOnly => true;

    double IRangeValueProvider.Maximum => 100;

    double IRangeValueProvider.Minimum => 0;

    double IRangeValueProvider.LargeChange => 0;

    double IRangeValueProvider.SmallChange => 0;

    void IRangeValueProvider.SetValue(double value) => throw new InvalidOperationException("a progress strip is read only");
}
