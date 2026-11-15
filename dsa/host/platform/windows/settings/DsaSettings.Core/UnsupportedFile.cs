using System.IO.Compression;
using System.Text.RegularExpressions;

namespace Dsa.Settings.Core;

/// <summary>What a file the APK Installer cannot install is.</summary>
public enum UnsupportedKind
{
    /// <summary>An app split into several APKs and packed together (<c>.xapk</c>, <c>.apks</c>
    /// or <c>.apkm</c>) that is damaged, or is not one app.</summary>
    SplitBundle,
    /// <summary>An Android App Bundle (<c>.aab</c>), the format apps are uploaded to stores in.</summary>
    AppBundle,
    /// <summary>A bundle whose APKs are encrypted, as some older <c>.apkm</c> files are.</summary>
    EncryptedBundle,
    /// <summary>A bundle with a split signed with another certificate than its base APK.</summary>
    SplitSignatureMismatch,
    /// <summary>An app whose native code is none DSA's Android runs, which is 64-bit only: it
    /// has 32-bit code only, or is a build made to run as 32-bit code.</summary>
    UnsupportedAbi,
    /// <summary>Anything else that is not an APK.</summary>
    NotApk,
}

/// <summary>
/// Says in plain words why a file cannot be installed. dsa names the reason with a stable
/// code when it refuses a bundle; otherwise what the file is comes from what is in it where it
/// can be read, so a renamed bundle is still named for what it is, and from its extension
/// otherwise. The sentences are in the string tables, in the user's language; dsa's own words
/// on what is wrong with a bundle are put in as dsa wrote them.
/// </summary>
public static class UnsupportedFile
{
    private static readonly Regex Refusal = new(@"^(?:dsa: )?not installed \(([a-z-]+)\): [^\r\n]*?: (.*)$", RegexOptions.Singleline);

    public static UnsupportedKind Classify(string path)
    {
        if (FromContent(path) is { } kind)
        {
            return kind;
        }
        return System.IO.Path.GetExtension(path).ToLowerInvariant() switch
        {
            ".xapk" or ".apks" or ".apkm" => UnsupportedKind.SplitBundle,
            ".aab" => UnsupportedKind.AppBundle,
            _ => UnsupportedKind.NotApk,
        };
    }

    /// <summary>What dsa's refusal of a file before Android saw it says it is, and dsa's own
    /// words on why; null when the line is no such refusal.</summary>
    public static (UnsupportedKind Kind, string Detail)? FromRefusal(string error)
    {
        var match = Refusal.Match(error.Trim());
        if (!match.Success)
        {
            return null;
        }
        UnsupportedKind? kind = match.Groups[1].Value switch
        {
            "app-bundle" => UnsupportedKind.AppBundle,
            "encrypted-bundle" => UnsupportedKind.EncryptedBundle,
            "split-signature-mismatch" => UnsupportedKind.SplitSignatureMismatch,
            "invalid-bundle" => UnsupportedKind.SplitBundle,
            "unsupported-abi" => UnsupportedKind.UnsupportedAbi,
            _ => null,
        };
        return kind is { } known ? (known, match.Groups[2].Value.Trim()) : null;
    }

    /// <summary>What the archive holds says what it is: an App Bundle has its BundleConfig.pb
    /// and its base module; a split bundle holds APKs, with toc.pb (.apks), info.json (.apkm)
    /// or manifest.json (.xapk) beside them. Null when the file is not an archive that says.</summary>
    private static UnsupportedKind? FromContent(string path)
    {
        try
        {
            using var zip = ZipFile.OpenRead(path);
            var names = zip.Entries.Select(e => e.FullName.Replace('\\', '/')).ToList();
            if (names.Any(n => n == "BundleConfig.pb" || n == "base/manifest/AndroidManifest.xml"))
            {
                return UnsupportedKind.AppBundle;
            }
            var bundled = names.Any(n => n.EndsWith(".apk", StringComparison.OrdinalIgnoreCase));
            return bundled || names.Contains("toc.pb") ? UnsupportedKind.SplitBundle : null;
        }
        catch (Exception e) when (e is InvalidDataException or IOException or UnauthorizedAccessException
            or NotSupportedException or ArgumentException)
        {
            return null;
        }
    }

    /// <summary>The sentence the window shows for <paramref name="fileName"/>, in the user's
    /// language. A damaged bundle's sentence ends with what is wrong with it, when dsa said.</summary>
    public static string Message(ITextSource text, UnsupportedKind kind, string fileName, string? detail = null)
    {
        var said = string.IsNullOrWhiteSpace(detail) ? null : detail!.Trim().TrimEnd('.');
        var (key, withDetail) = kind switch
        {
            UnsupportedKind.SplitBundle when said is not null => ("ApkWindow_File_SplitBundleDetail", true),
            UnsupportedKind.SplitBundle => ("ApkWindow_File_SplitBundle", false),
            UnsupportedKind.AppBundle => ("ApkWindow_File_AppBundle", false),
            UnsupportedKind.EncryptedBundle => ("ApkWindow_File_EncryptedBundle", false),
            UnsupportedKind.SplitSignatureMismatch => ("ApkWindow_File_SplitSignature", false),
            UnsupportedKind.UnsupportedAbi when said is not null => ("ApkWindow_File_AbiDetail", true),
            UnsupportedKind.UnsupportedAbi => ("ApkWindow_File_Abi", false),
            _ => ("ApkWindow_File_NotApk", false),
        };
        return withDetail
            ? string.Format(System.Globalization.CultureInfo.CurrentCulture, text.Get(key), fileName, said)
            : string.Format(System.Globalization.CultureInfo.CurrentCulture, text.Get(key), fileName);
    }

    /// <summary>Every text <see cref="Message"/> can show, with how many values it takes.</summary>
    public static readonly IReadOnlyDictionary<string, int> AllKeys = new Dictionary<string, int>
    {
        ["ApkWindow_File_SplitBundleDetail"] = 2,
        ["ApkWindow_File_SplitBundle"] = 1,
        ["ApkWindow_File_AppBundle"] = 1,
        ["ApkWindow_File_EncryptedBundle"] = 1,
        ["ApkWindow_File_SplitSignature"] = 1,
        ["ApkWindow_File_AbiDetail"] = 2,
        ["ApkWindow_File_Abi"] = 1,
        ["ApkWindow_File_NotApk"] = 1,
    };
}
