# Fehlerbehebung und Protokolle

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · **Deutsch** · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

Diese Seite hilft Ihnen herauszufinden, warum Desktop Subsystem for Android™ (DSA) nicht
funktioniert, das Protokoll von Android zu lesen und einen Problembericht zu senden. Sie
behandelt Windows und, wo angegeben, Linux.

## Zuerst den Zustand prüfen

```
dsa status          # Zustand von Android, Probleme und ausstehende Updates
dsa start --wait    # Android starten und warten, bis es hochgefahren ist
```

Die Seite **General** (Allgemein) von DSA Settings zeigt denselben Zustand.

| `dsa status` | DSA Settings | Bedeutung |
|---|---|---|
| `Active` | **Running** | Android ist hochgefahren, und der DSA-Agent darin ist verbunden |
| `Dozing` | **Dozing (idle)** | Android ist im Leerlauf. Siehe [Speicher](memory.md) |
| `Frozen` | **Frozen** | Android ist eingefroren, bis Sie eines seiner Fenster öffnen |
| `Stalled` | **Not ready** | Android läuft, hat aber zu lange gebraucht. Die nächste Zeile nennt den Grund und was zu tun ist |

## Häufige Probleme

| Problem | Was zu tun ist |
|---|---|
| Das Setup meldet, dass die VM-Plattform (Virtual Machine Platform) ausgeschaltet ist | Aktivieren Sie das Windows-Feature, starten Sie neu und führen Sie das Setup erneut aus. Siehe [Erste Schritte](getting-started.md) |
| Es öffnet sich kein App-Fenster | Melden Sie sich ab und wieder an. Der Fenstermanager startet bei der Anmeldung |
| Eine App öffnet sich in einer falschen Größe | `dsa app-window <package> reset` |
| Android startet nach einem Abbild-Update nicht | Nichts. DSA kehrt von selbst zum vorherigen Abbild zurück, und `dsa status` meldet das |
| Android hat kein Netzwerk | Öffnen Sie die Seite **Network** von DSA Settings, oder führen Sie `dsa network` aus. Dort sehen Sie Modus, Adresse, DNS-Server und etwaige Probleme |
| Android ist defekt, und nichts anderes hilft | Setzen Sie die Daten von Android zurück (siehe unten) |

### Die Daten von Android zurücksetzen

Das Zurücksetzen löscht jede von Ihnen installierte App, Ihre Konten und alle App-Daten. Android
startet dann wie nach der ersten Installation. Das Android-Abbild und die Einstellungen von DSA
bleiben erhalten.

- In DSA Settings: Seite **Advanced** > **Reset Android's data** > **Reset**.
- In der Befehlszeile: `dsa reset-data` gibt an, was verloren ginge. `dsa reset-data --yes`
  löscht es.

## Das Protokoll von Android lesen

Das Protokoll von Android bleibt in Android im Speicher und wird nie in eine Datei geschrieben.
Es erreicht Ihren PC nur, solange etwas es liest.

- In DSA Settings: die Seite **Logs** (Protokolle). Sie können nach Tag oder Text filtern und
  anhalten.
- In der Befehlszeile:

  ```
  dsa logcat                        # dem Protokoll folgen; Strg+C beendet es
  dsa logcat -d -b all *:W          # was das Protokoll jetzt enthält, ab Warnungen
  dsa logcat ActivityManager:I *:S  # nur ein Tag
  ```

## Einen Problembericht senden

1. Speichern Sie die Diagnosedaten. Wählen Sie auf der Seite **Logs** die Option **Save
   diagnostics**, oder führen Sie aus:

   ```
   dsa logs collect -o diag.zip
   ```

   Das dauert eine Minute oder länger.
2. Hängen Sie die ZIP-Datei an ein
   [Issue](https://github.com/thisisthepy/desktop-subsystem-for-android/issues) an.

Das Archiv enthält das ganze Protokoll von Android, seine Absturzaufzeichnungen (tombstones,
ANR-Traces, dropbox, pstore), eine Zusammenfassung der Systemdienste von Android und die
DSA-Protokolle auf diesem PC, die Sie betreffen, ohne die Zeilen anderer Benutzer.
`summary.txt` im Archiv gibt an, was nicht gesammelt werden konnte.

## Wo die Protokolle unter Windows liegen

| Ordner | Dateien |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (der Fenstermanager), `dsa-app-window.log` (App-Fenster) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (der Dienst), `android-<SID>.log` (die Android-Konsole jedes Benutzers, höchstens 2 MiB pro Systemstart), `setup-<version>.log` (Selbstaktualisierungen) |

Um sie zu öffnen, verwenden Sie **Open my log folder** und **Open the service's log folder
(administrators)** auf der Seite **Advanced** von DSA Settings.

## Wo die Protokolle unter Linux liegen (noch nicht veröffentlicht)

Die Dienste schreiben in das systemd-Journal:

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## Jeder Befehl

`dsa` ohne Argumente gibt jeden Befehl aus. Ergänzen Sie `--json` für eine Ausgabe, die
Programme lesen können.

---

Android ist eine Marke von Google LLC. Windows ist eine Marke der Microsoft-Unternehmensgruppe.
DSA steht in keiner Verbindung zu Google oder Microsoft und wird von ihnen weder unterstützt noch
gesponsert.
