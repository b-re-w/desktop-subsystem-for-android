#nullable enable
using System;
using System.Collections.Generic;
using System.Globalization;

namespace Dsa.Settings.Core;

/// <summary>
/// The languages DSA's windows are written in, and which of them a system language gets. The
/// display language picks it: the culture is tried, then each of its parents, so a regional
/// culture such as fr-CA finds French. Traditional Chinese (zh-Hant, zh-TW, zh-HK, zh-MO) gets
/// zh-TW and Simplified Chinese (zh-Hans, zh-CN, zh-SG, plain zh) gets zh-CN; Portuguese in
/// every region gets Brazilian Portuguese; Indonesian answers to its old code <c>in</c> too.
/// Anything else gets English.
///
/// The settings app, the APK Installer and the setup window share this file (setup compiles it
/// in, being a .NET Framework program), so all three pick the same language on one machine.
/// </summary>
public sealed class UiLanguage
{
    public static readonly UiLanguage English = new("en", 0);
    public static readonly UiLanguage Korean = new("ko", 1);
    public static readonly UiLanguage Japanese = new("ja", 2);
    public static readonly UiLanguage ChineseSimplified = new("zh-CN", 3);
    public static readonly UiLanguage ChineseTraditional = new("zh-TW", 4);
    public static readonly UiLanguage Spanish = new("es", 5);
    public static readonly UiLanguage French = new("fr", 6);
    public static readonly UiLanguage German = new("de", 7);
    public static readonly UiLanguage PortugueseBrazil = new("pt-BR", 8);
    public static readonly UiLanguage Russian = new("ru", 9);
    public static readonly UiLanguage Italian = new("it", 10);
    public static readonly UiLanguage Vietnamese = new("vi", 11);
    public static readonly UiLanguage Indonesian = new("id", 12);
    public static readonly UiLanguage Turkish = new("tr", 13);

    private static readonly UiLanguage[] Languages =
    {
        English, Korean, Japanese, ChineseSimplified, ChineseTraditional, Spanish, French, German,
        PortugueseBrazil, Russian, Italian, Vietnamese, Indonesian, Turkish,
    };

    private CultureInfo? _culture;

    private UiLanguage(string tag, int index)
    {
        Tag = tag;
        Index = index;
    }

    /// <summary>Every language, English first, in the order of <see cref="Index"/>.</summary>
    public static IReadOnlyList<UiLanguage> All => Languages;

    /// <summary>The language's tag, which also names its string table
    /// (<c>Strings.&lt;tag&gt;.resx</c>): en, ko, ja, zh-CN, zh-TW, es, fr, de, pt-BR, ru, it,
    /// vi, id or tr.</summary>
    public string Tag { get; }

    /// <summary>The language's place in <see cref="All"/>.</summary>
    public int Index { get; }

    /// <summary>The culture the string tables are read in for this language. It names the
    /// table's own culture, so a pt-PT desktop reads the pt-BR table and a zh-HK one the zh-TW
    /// table, which .NET's own fallback from those cultures would not find.</summary>
    public CultureInfo Culture
    {
        get
        {
            if (_culture is null)
            {
                try
                {
                    _culture = CultureInfo.GetCultureInfo(Tag);
                }
                catch (CultureNotFoundException)
                {
                    // A system without culture data reads the English table.
                    _culture = CultureInfo.InvariantCulture;
                }
            }
            return _culture;
        }
    }

    /// <summary>The language of the user's display language.</summary>
    public static UiLanguage Current => Resolve(CultureInfo.CurrentUICulture);

    /// <summary>The language with exactly this tag (any case), or null.</summary>
    public static UiLanguage? FromTag(string? tag)
    {
        foreach (var language in Languages)
        {
            if (string.Equals(language.Tag, tag, StringComparison.OrdinalIgnoreCase))
            {
                return language;
            }
        }
        return null;
    }

    /// <summary>The language for a culture: the culture's own name decides when it names one of
    /// the languages, else the nearest parent that does, else English.</summary>
    public static UiLanguage Resolve(CultureInfo? culture)
    {
        // The parents end at the invariant culture, which is its own parent; the count keeps a
        // custom culture whose parents loop from going round for ever.
        for (var depth = 0; culture is not null && depth < 8; depth++)
        {
            if (Match(culture.Name) is { } language)
            {
                return language;
            }
            if (culture.Name.Length == 0 || ReferenceEquals(culture.Parent, culture))
            {
                break;
            }
            culture = culture.Parent;
        }
        return English;
    }

    /// <summary>The language for a culture name such as <c>pt-PT</c> or <c>zh-Hant-HK</c>. A
    /// name .NET does not know is still read by its language, script and region.</summary>
    public static UiLanguage Resolve(string? cultureName)
    {
        if (string.IsNullOrWhiteSpace(cultureName))
        {
            return English;
        }
        if (Match(cultureName!) is { } language)
        {
            return language;
        }
        try
        {
            return Resolve(CultureInfo.GetCultureInfo(cultureName!.Trim()));
        }
        catch (CultureNotFoundException)
        {
            return English;
        }
    }

    /// <summary>The language a culture name names by its language subtag (and, for Chinese, its
    /// script or region), or null.</summary>
    private static UiLanguage? Match(string name)
    {
        var parts = name.Trim().Split(new[] { '-', '_' }, StringSplitOptions.RemoveEmptyEntries);
        if (parts.Length == 0)
        {
            return null;
        }
        switch (parts[0].ToLowerInvariant())
        {
            case "zh":
                // The script says it when there is one (zh-Hant-..., or Windows' old zh-CHT);
                // else the region; plain zh is Simplified.
                for (var i = 1; i < parts.Length; i++)
                {
                    switch (parts[i].ToUpperInvariant())
                    {
                        case "HANT":
                        case "CHT":
                        case "TW":
                        case "HK":
                        case "MO":
                            return ChineseTraditional;
                        case "HANS":
                        case "CHS":
                        case "CN":
                        case "SG":
                            return ChineseSimplified;
                    }
                }
                return ChineseSimplified;
            case "pt":
                return PortugueseBrazil;
            case "in":
                return Indonesian;
            case var code:
                foreach (var language in Languages)
                {
                    if (language.Tag == code)
                    {
                        return language;
                    }
                }
                return null;
        }
    }

    public override string ToString() => Tag;
}
