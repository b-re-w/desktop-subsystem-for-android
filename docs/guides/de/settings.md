# Einstellungen

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · [Français](../fr/settings.md) · **Deutsch** · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings** ist die Einstellungs-App von Desktop Subsystem for Android™ (DSA) unter Windows.
Öffnen Sie sie aus dem Startmenü. Sie erscheint auf Englisch oder Koreanisch, je nach Ihrer
Windows-Anzeigesprache, und folgt Ihrem Windows-Design. Sie funktioniert mit der Tastatur und mit
Bildschirmleseprogrammen.

Jede Änderung, die DSA Settings vornimmt, läuft über den Befehl `dsa`, sodass die Befehlszeile
dieselben Werte sieht:

```
dsa settings                        # jede Einstellung und ihr Wert
dsa settings set <name> <value>     # eine ändern; Schalter nehmen on oder off
```

Manche Einstellungen gelten für jeden Benutzer des PCs, daher kann nur ein Administrator sie
ändern. DSA Settings kennzeichnet sie mit einem Schildsymbol und fragt nach der Zustimmung eines
Administrators, wenn Sie eine davon ändern. Unten sind sie mit „(Admin)“ gekennzeichnet.

## Seiten

| Seite | Was Sie dort finden | Befehlszeile |
|---|---|---|
| **General** | Zustand von Android, **Start**, **Stop**, **Freeze Android now**, das letzte Problem, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** | **Memory limit**, die Größe der VM von Android, was jeder Teil belegt | `memory-limit`, `dsa memory` |
| **Apps** | Installierte Apps: **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. Für jede App: **Resizable**, die Größe, mit der sich Fenster öffnen, **Scaling**, **Full screen request**, **When only the bars hide**, **Run as ARM**, **Root access**. Eine Zeile für den Root-Zugriff von **adb shell** | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** | Apps, die ohne Fenster laufen: **Open**, **Stop**; ob vor dem Beenden gefragt wird | `dsa background`, `stop-without-asking` |
| **Widgets** | Widgets auf Ihrem Desktop und **Add to desktop** | `dsa widgets` |
| **Display & GPU** | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (nur Windows 11, standardmäßig ein), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** | Zuerst dieser Computer, schreibgeschützt: sein Name, Hersteller und Modell sowie **Device specifications** mit **Copy**. Dann Hersteller, Modell und Build-Fingerabdruck, die Android den Apps meldet | `dsa settings device`, `device.<field>` |
| **Network** | Wie Android das Netzwerk erreicht, seine Adresse, wer seine Namen beantwortet (DNS) und sein Gateway, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** | **Android**: die laufende Version und ihr Build, ihre Sicherheitspatch-Ebene, die Version, der sie folgt, das Android-Update mit seinem Fortschritt und **Check now** (Admin), **Restart Android into updates by itself**, **Update the Android image automatically** (Admin), **Version to use** (Admin). **DSA**: **Update DSA automatically** (Admin), **Get pre-release versions** (Admin), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** | Zwischenablage, Dateien, Weblinks, Einfrieren im Leerlauf, Sicherung, Zurücksetzen, ARM-Apps, Google Play, geschützte Inhalte, Root-Zugriff, Traces (Admin), Protokollordner | siehe unten |
| **Logs** | Das Live-Protokoll von Android, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** | Versionen, Lizenzen, Quellcode | |

## Die Seite Advanced

| Einstellung | Standard | Befehlszeile |
|---|---|---|
| **Share the clipboard with Android** | Ein | `clipboard` |
| **Share Windows folders with Android** | Ein | `file-sharing` |
| **Shared folders** | Alle sechs | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Aus | `freeze` |
| **Open web links in Android apps** | Aus | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Aus | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (nicht auf ARM64) | | `dsa arm-translation fetch google` |
| **Use the default translator** (nicht auf ARM64) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Nicht installiert | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (unter **Protected content**) | Nicht installiert | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (Admin) | Aus | `root-elevation` |
| **Input trace** (Admin) | Aus | `input-trace` |
| **Resize trace** (Admin) | Aus | `resize-trace` |

Die Sicherung bewahrt eine verschlüsselte Kopie der Daten Ihrer Apps in Ihrem Benutzerordner auf,
mit einem Schlüssel, den Windows für Sie verwahrt. Um sie auf einem anderen PC oder nach einer
Neuinstallation von Windows wiederherzustellen, benötigen Sie den Wiederherstellungsschlüssel.
Notieren Sie ihn daher an einem Ort außerhalb des PCs. Google Play, Widevine und Root-Zugriff
sind unter [Apps und Fenster](apps.md) beschrieben.

## Wann eine Änderung wirksam wird

- **Render GPU**: wenn der Fenstermanager erneut startet, bei Ihrer nächsten Anmeldung oder wenn
  Sie **Restart the window manager** wählen.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for
  ARM**, Google Play, Widevine, das Einschalten des Root-Zugriffs und die Android-Version: beim
  nächsten Start von Android. Um Android neu zu starten, stoppen Sie es auf der Seite **General**
  und öffnen Sie dann eine App.
- Die Fenstereinstellungen einer App: ab dem nächsten Fenster, das die App öffnet.
- Alles andere, einschließlich des Ausschaltens des Root-Zugriffs: sofort.

## Wie Android Namen auflöst

Ihr Computer beantwortet jeden Namen, den die Apps von Android nachschlagen, mit demselben
Resolver, den Windows-Apps verwenden (unter Linux und macOS mit dem Resolver des Systems). Ihre
hosts-Datei, die Namensregeln Ihres Unternehmens, die eigenen DNS-Server eines VPN und ein
DNS-Filter gelten daher für die Apps von Android genauso wie für den Computer. Android fragt
selbst keinen DNS-Server.

- Das gilt auch bei eingeschaltetem **Advanced networking**: Ihr Computer ist im selben Netzwerk
  und kennt dessen Namen, und ein VPN oder DNS-Filter auf ihm gilt weiterhin.
- Das eigene **Private DNS** von Android ist standardmäßig ausgeschaltet, weil es einen DNS-Server
  an Ihrem Computer vorbei fragen würde. Wählen Sie in den Einstellungen von Android einen
  privaten DNS-Server, verwendet Android ihn.
- Kann DSA eine Weile nicht antworten, fragt Android die DNS-Server des Netzwerks, bis DSA wieder
  antworten kann.
- Die Zeile **DNS** auf der Seite **Network** und `dsa network` geben an, wer gerade antwortet:
  der Resolver Ihres Computers oder die DNS-Server des Netzwerks, die sie dann auflisten.

## Werte, die die Befehlszeile annimmt

- `render-gpu`: `display` (die GPU des primären Monitors, der Standard), `integrated`,
  `discrete` oder ein Teil des Namens einer GPU, wie `dsa gpus` ihn auflistet.
- `memory-limit`: `auto` (der Standard: kein festes Limit, bis zum Speicher des PCs abzüglich
  dessen, was Windows behält) oder eine Größe wie `4g` oder `3072` (MiB).
- `shared-folders`: `all`, `none` oder durch Kommas getrennte Namen, etwa
  `Documents,Pictures`.
- `adb-port`: ein TCP-Port von 1 bis 65535. Der Standard ist 58526.
- `app-fullscreen`: `screen` (der Standard) oder `window`, für die klare Vollbildanforderung
  einer App. `app-immersive`: `screen` oder `window` (der Standard), für eine App, die nur ihre
  Leisten ausblendet. Siehe [Vollbild](apps.md#vollbild).
- `touch-gestures-off`: `on` (der Standard) schaltet die Touchgesten von Windows 11 mit drei und
  vier Fingern aus, solange ein DSA-App-Fenster im Vordergrund ist, damit Apps bis zu zehn Finger
  erhalten, und stellt Ihre eigene Einstellung wieder her, wenn ein anderes Fenster in den
  Vordergrund kommt. Lehnt Windows das ab, lässt sich die Einstellung nicht verwenden, und die
  Einstellungs-App nennt den Grund. Sie können die Gesten dennoch für jede App in den
  Windows-Einstellungen unter Bluetooth und Geräte > Toucheingabe ausschalten.
- `host-certificates`: `on` (der Standard) lässt Android-Apps Stammzertifikaten vertrauen, die
  diesem PC hinzugefügt wurden, etwa denen Ihres Unternehmens oder eines Sicherheitsprogramms.
- `adb-auto-connect`: `on` (der Standard) zeigt Android in `adb devices` und Android Studio,
  solange der Entwicklermodus eingeschaltet ist, ohne `adb connect`.
- `image-auto-restart`: `on` (der Standard) startet Android sofort in ein heruntergeladenes
  Abbild-Update neu, wenn kein App-Fenster geöffnet ist. Andernfalls fragt DSA zuerst oder wartet,
  bis Sie sie schließen.
- `prerelease-updates`: `on` nimmt Vorabversionen und stabile Releases, `off` nur stabile
  Releases. Der Wert richtet sich zunächst nach dem verwendeten Installationsprogramm: Ein
  Installationsprogramm einer Vorabversion schaltet ihn ein, ein stabiles lässt ihn aus, und
  Upgrades behalten Ihre Wahl. Jedes Release vor 1.0 ist eine Vorabversion. Das Ausschalten
  bringt DSA nie auf eine ältere Version zurück; DSA wartet auf ein stabiles Release, das neuer
  ist als das installierte.
- `root-elevation`: `on` lässt Apps, die Sie auf der Seite **Apps** zulassen, nach Root fragen.
  Die Stufe jeder App wird mit `dsa apps root <package> off|ask|remember` festgelegt.
- `device.<field>`: `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint` oder ein Teil des Fingerabdrucks wie `fingerprint.release`.
  `dsa settings reset device` setzt jeden Wert zurück.

---

Android ist eine Marke von Google LLC. Windows ist eine Marke der Microsoft-Unternehmensgruppe.
DSA steht in keiner Verbindung zu Google oder Microsoft und wird von ihnen weder unterstützt noch
gesponsert.
