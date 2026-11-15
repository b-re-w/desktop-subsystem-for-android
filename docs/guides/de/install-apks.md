# APKs installieren

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · [简体中文](../zh-CN/install-apks.md) · [繁體中文](../zh-TW/install-apks.md) · [Español](../es/install-apks.md) · [Français](../fr/install-apks.md) · **Deutsch** · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · [Tiếng Việt](../vi/install-apks.md) · [Bahasa Indonesia](../id/install-apks.md) · [Türkçe](../tr/install-apks.md)

Desktop Subsystem for Android™ (DSA) liefert keinen App-Store mit. Sie installieren Apps aus
APK-Dateien, mit dem APK Installer oder dem Befehl `dsa`. Diese Seite beschreibt DSA unter
Windows.

## Mit dem APK Installer installieren

1. Doppelklicken Sie im Explorer auf eine `.apk`-Datei. Sie öffnet sich im **APK Installer**.
2. Prüfen Sie, was die APK ist. Das Fenster zeigt es, ohne Android zu starten:
   - Name, Symbol, Paket und Version der App;
   - den Herausgeber, entnommen aus dem Signaturzertifikat, weil eine APK keinen Herausgeber
     nennt;
   - die Quelle: den Host, von dem die Datei heruntergeladen wurde, oder ihren Ordner;
   - unter **Capabilities** (Fähigkeiten) die ersten Berechtigungen, die sie anfordert. **More**
     (mehr) zeigt alle, die Android-Versionen, für die sie gemacht ist, ihre Größe, ihre
     Prozessortypen und den SHA-256 ihres Zertifikats.

   Enthält die APK nur Code für einen anderen Prozessortyp, warnt Sie das Fenster.
3. Wählen Sie **Install** (Installieren). Ist die App bereits installiert, lautet die
   Schaltfläche **Update** (Aktualisieren) oder **Install again** (erneut installieren). Läuft
   Android nicht, startet es zuerst.
4. Mit dem Häkchen bei **Launch when ready** (nach Abschluss starten, der Standard) öffnet sich
   die App, sobald sie installiert ist, und das Installationsprogramm schließt sich. Andernfalls
   wählen Sie **Launch** (Starten), oder öffnen Sie die App später aus dem Startmenü.

Während Android läuft, gibt das Fenster außerdem an, ob die Datei eine Neuinstallation, ein
Update, dieselbe Version noch einmal oder etwas ist, das Android ablehnt: eine ältere Version
oder dieselbe App, mit einem anderen Schlüssel signiert.

Sie können APK-Dateien auch auf dem APK Installer, auf DSA Settings oder auf dem Fenster einer
App ablegen. Jede APK öffnet sich in einem eigenen Installationsfenster.

## Über die Befehlszeile installieren

```
dsa apk-info app.apk            # was die APK ist; liest die Datei, ohne Android zu starten
dsa install app.apk             # installieren; startet Android, wenn es nicht läuft
dsa install app.apk --progress  # zusätzlich jeden Schritt ausgeben
```

Lehnt Android die APK ab, endet `dsa install` mit dem Code 1, und seine Meldung beginnt mit
`not installed (<reason>):`.

## Warum eine Installation abgelehnt wird

| Grund | Bedeutung | Was zu tun ist |
|---|---|---|
| `incompatible` | Die App benötigt einen anderen Prozessortyp, eine neuere Android-Version oder eine Funktion, die diesem Gerät fehlt | Siehe [Apps nur für ARM](apps.md#apps-nur-für-arm) oder [Android-Versionen](android-versions.md) |
| `conflicting-signature` | Die App ist bereits installiert, mit einem anderen Schlüssel signiert | Deinstallieren Sie zuerst die installierte App |
| `downgrade` | Eine neuere Version der App ist installiert | Deinstallieren Sie zuerst die installierte App |
| `invalid-apk` | Die Datei ist keine signierte APK, die Android lesen kann | Besorgen Sie die APK erneut |
| `insufficient-storage` | Android hat keinen Platz für die App | Geben Sie Speicherplatz in Android frei |
| `blocked` | Eine Richtlinie dieses Android lässt die App nicht zu | |
| `other` | Alles andere; die eigene Meldung von Android folgt | |

Eine APK kann bis zu 2 GiB groß sein. Installationen laufen nacheinander: Eine zweite wartet auf
die erste.

## Eine App deinstallieren

Verwenden Sie eine dieser Möglichkeiten:

- Windows **Einstellungen** > **Apps** > **Installierte Apps**.
- Die Seite **Apps** von DSA Settings: **Uninstall** (Deinstallieren) neben der App.
- `dsa apps uninstall <package>`.

Apps, die zu Android gehören, lassen sich nicht deinstallieren.

---

Android ist eine Marke von Google LLC. Windows ist eine Marke der Microsoft-Unternehmensgruppe.
DSA steht in keiner Verbindung zu Google oder Microsoft und wird von ihnen weder unterstützt noch
gesponsert.
