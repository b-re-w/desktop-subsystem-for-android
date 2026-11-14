using System.Globalization;
using System.Resources;
using System.Windows.Markup;
using Dsa.Settings.Core;

namespace Dsa.Settings;

/// <summary>The texts in the user's language, from Strings.resx (English) and
/// Strings.&lt;language&gt;.resx. The language is the one of DSA's fourteen that the display
/// language resolves to (<see cref="UiLanguage"/>), so a pt-PT desktop reads the pt-BR table,
/// a zh-HK one the zh-TW table, and any language DSA is not written in reads English.</summary>
internal sealed class ResourceText : ITextSource
{
    private static readonly ResourceManager Manager =
        new("Dsa.Settings.Strings", typeof(ResourceText).Assembly);

    public string Get(string key)
    {
        try
        {
            return Manager.GetString(key, UiLanguage.Resolve(CultureInfo.CurrentUICulture).Culture) ?? key;
        }
        catch (MissingManifestResourceException)
        {
            return key;
        }
    }
}

/// <summary>A text in the user's language, in XAML: <c>{l:Text NavGeneral}</c>.</summary>
[MarkupExtensionReturnType(typeof(string))]
public sealed class TextExtension : MarkupExtension
{
    public TextExtension()
    {
    }

    public TextExtension(string key)
    {
        Key = key;
    }

    [ConstructorArgument("key")]
    public string Key { get; set; } = "";

    public override object ProvideValue(IServiceProvider serviceProvider) => App.Text.Get(Key);
}
