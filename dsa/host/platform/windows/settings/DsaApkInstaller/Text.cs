using System.Globalization;
using System.Resources;
using Dsa.Settings.Core;

namespace Dsa.ApkInstaller;

/// <summary>The window's texts in the user's language, from the settings app's string tables
/// (Strings.resx and its translations), which this app embeds as its own. The language is the
/// one of DSA's fourteen that the display language resolves to (<see cref="UiLanguage"/>), as
/// in the settings app, so both windows speak the same language.</summary>
internal sealed class ResourceText : ITextSource
{
    private static readonly ResourceManager Manager =
        new("Dsa.ApkInstaller.Strings", typeof(ResourceText).Assembly);

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
