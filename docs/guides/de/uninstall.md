# Deinstallation

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · **Deutsch** · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

Deinstallieren Sie Desktop Subsystem for Android™ (DSA) auf eine dieser Arten:

- Windows **Einstellungen** > **Apps** > **Installierte Apps**: Suchen Sie Desktop Subsystem for
  Android und wählen Sie dann **Deinstallieren**.
- Führen Sie das Setup erneut aus und wählen Sie **Uninstall** (Deinstallieren).

Die Deinstallation entfernt Folgendes, für jeden Benutzer des PCs, ob angemeldet oder nicht:

- das Programm, den Dienst `DsaHost`, den Befehl `dsa`, DSA Settings und den APK Installer;
- die heruntergeladenen Android-Abbilder und Updates;
- die Startmenüeinträge der Android-Apps und ihre Einträge unter **Installierte Apps**;
- **Android** im Explorer, die Dateizuordnung für `.apk` und die Einträge im Kontextmenü;
- die Einstellungen und Protokolle von DSA.

Sie **behält** den Android-Datenträger jedes Benutzers, `%LOCALAPPDATA%\DSA\android.vhdx`,
sodass eine spätere Installation Ihre Apps und Daten wiederfindet. Um auch Ihre Daten zu
entfernen, führen Sie vor der Deinstallation `dsa reset-data --yes` aus, oder löschen Sie diese
Datei danach.

Hat eine Ordnerumleitung das Startmenü eines Benutzers verschoben, bleiben die Einträge der
Android-Apps dieses Benutzers erhalten, weil das Deinstallationsprogramm nur am Standardort sucht.
Löschen Sie sie von Hand.

Ihre WSL-Einrichtung bleibt unberührt, weil DSA sie nie ändert.

## Linux (noch nicht veröffentlicht)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

Das Entfernen des Pakets entfernt auch den Eintrag für das Paket-Repository, den es hinzugefügt
hat.

---

Android ist eine Marke von Google LLC. Windows ist eine Marke der Microsoft-Unternehmensgruppe.
DSA steht in keiner Verbindung zu Google oder Microsoft und wird von ihnen weder unterstützt noch
gesponsert.
