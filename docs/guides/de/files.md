# Dateien

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · [简体中文](../zh-CN/files.md) · [繁體中文](../zh-TW/files.md) · [Español](../es/files.md) · [Français](../fr/files.md) · **Deutsch** · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · [Italiano](../it/files.md) · [Tiếng Việt](../vi/files.md) · [Bahasa Indonesia](../id/files.md) · [Türkçe](../tr/files.md)

Mit Desktop Subsystem for Android™ (DSA) öffnen Sie die Dateien von Android im Explorer, teilen
Ihre Windows-Ordner mit Android-Apps und senden eine Datei an eine Android-App. Diese Seite
beschreibt DSA unter Windows.

## Die Dateien von Android im Explorer öffnen

Verwenden Sie eine dieser Möglichkeiten. Jede startet Android, wenn es nicht läuft.

- **Android** im Navigationsbereich des Explorers, direkt unter **Linux** von WSL.
- **Android files** (Android-Dateien) auf der Seite **Advanced** (Erweitert) von DSA Settings.
- `dsa files open`. `dsa files` gibt an, wo sich der Speicher befindet, ohne ihn zu öffnen.

Sie sehen den gemeinsamen Speicher von Android: die Dateien, die Android-Apps gespeichert haben,
etwa Fotos und Downloads. Sie können Dateien wie in jedem Ordner hinein- und herauskopieren. Der
Explorer erreicht den Speicher über den WebDAV-Client von Windows. Antwortet der Speicher nicht,
zeigt DSA einen Fehler mit dem Grund.

## Ihre Windows-Ordner mit Android-Apps teilen

Ihre Ordner Dokumente, Downloads, Bilder, Musik, Videos und Desktop erscheinen im gemeinsamen
Speicher von Android unter `Host`, etwa `Host/Pictures`. Android-Apps können dort Dateien öffnen
und speichern. Eine App benötigt zuerst die Speicherberechtigung von Android, und Windows lässt
sie nur das tun, was Ihr eigenes Konto darf.

Das Teilen ist standardmäßig eingeschaltet. Um es zu ändern, öffnen Sie die Seite **Advanced**
von DSA Settings:

- Schalten Sie **Share Windows folders with Android** aus, um keinen Ordner zu teilen.
- Entfernen Sie unter **Shared folders** (geteilte Ordner) das Häkchen bei einem Ordner, um ihn
  nicht mehr zu teilen. Er verschwindet sofort aus Android.

Über die Befehlszeile:

```
dsa settings set shared-folders Documents,Pictures   # nur diese; auch all oder none
dsa settings set file-sharing off                     # keinen Ordner teilen
```

Frühere Versionen verwendeten statt `Host` einen Ordner namens `Windows`. Android entfernt den
alten Ordner, sobald er leer ist, sodass Dateien, die Sie dort gespeichert haben, erhalten
bleiben.

## Eine Datei an eine Android-App senden

- **Ziehen und Ablegen.** Ziehen Sie eine Datei aus dem Explorer auf das Fenster einer App. Die
  Datei geht an diese App. Eine APK geht stattdessen an den APK Installer. Ordner werden nicht
  gesendet.
- **Kontextmenü.** Klicken Sie mit der rechten Maustaste auf eine Datei und wählen Sie **Open in
  an Android app** oder **Share with an Android app**. Unter Windows 11 finden Sie sie unter
  **Weitere Optionen anzeigen**. Android fragt dann, welche App die Datei übernimmt.
- **Befehlszeile.** `dsa open <file>` und `dsa share <file>`.

Damit senden Sie Dateien bis 64 MiB. Eine größere Datei wird abgelehnt, bevor etwas gesendet
wird. Um einer App eine größere Datei zu geben, legen Sie sie stattdessen in einen geteilten
Ordner.

## Eine Datei aus Android holen

- Speichern Sie sie in Android unter `Host/<folder>`. Dann liegt sie in diesem Ordner auf Ihrem
  PC.
- Oder öffnen Sie **Android** im Explorer und kopieren Sie die Datei heraus.

---

Android ist eine Marke von Google LLC. Windows ist eine Marke der Microsoft-Unternehmensgruppe.
DSA steht in keiner Verbindung zu Google oder Microsoft und wird von ihnen weder unterstützt noch
gesponsert.
