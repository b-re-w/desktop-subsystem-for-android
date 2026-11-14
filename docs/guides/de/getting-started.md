# Erste Schritte

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · **Deutsch** · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™ (DSA) führt Android in einer schlanken virtuellen Maschine mit
eigenem Kernel aus. Jede Android-App öffnet sich in einem eigenen Fenster auf Ihrem Desktop.

## Stand nach Plattform

| Host | Status |
|---|---|
| Windows 11, x64 | Veröffentlicht. |
| Windows 11, ARM64 | Veröffentlicht. Android ist ebenfalls ARM64, daher laufen ARM-Apps ohne Übersetzung. |
| Linux, x64 und ARM64 | In Arbeit. Die Pakete `.deb` und `.rpm` werden in der CI gebaut; noch nicht veröffentlicht. |
| macOS, Apple silicon | In Arbeit. `DSA.app`, sein `.pkg` und sein `.dmg` werden in der CI gebaut; noch nicht veröffentlicht. |

## Unter Windows installieren

Sie benötigen Windows 11 auf einem x64- oder ARM64-PC, mit aktiviertem Windows-Feature
**VM-Plattform** (Virtual Machine Platform).

1. Aktivieren Sie die VM-Plattform. Öffnen Sie **Windows-Features aktivieren oder
   deaktivieren**, setzen Sie ein Häkchen bei **VM-Plattform**, wählen Sie **OK** und starten
   Sie neu. Oder führen Sie
   `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` in einem Terminal
   als Administrator aus. Das Setup aktiviert das Feature nicht für Sie: Es bricht mit einer
   Meldung ab, wenn das Feature ausgeschaltet ist.
2. Laden Sie das Setup-Programm und `SHA256SUMS.txt` vom
   [neuesten Release](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest)
   herunter: `DSA-v<version>-win-x64-setup.exe` auf einem x64-PC,
   `DSA-v<version>-win-arm64-setup.exe` auf einem ARM64-PC.
3. Prüfen Sie den Download. Der Hash, den dieser Befehl ausgibt, muss mit der Zeile in
   `SHA256SUMS.txt` übereinstimmen:

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. Führen Sie das Setup aus, wählen Sie **Install** (Installieren) und bestätigen Sie die
   Administratorabfrage.
5. Warten Sie auf das erste Android-Abbild. Wenn das Setup keines mitbringt, lädt der DSA-Dienst
   es im Hintergrund herunter. `dsa status` zeigt den Download.
6. Öffnen Sie eine App aus dem Startmenü, oder prüfen Sie es in einem Terminal:

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## Was das Setup installiert

- Den Dienst `DsaHost`, der das Android jedes Benutzers ausführt.
- Den Befehl `dsa`.
- **DSA Settings** im Startmenü.
- **Android** im Navigationsbereich des Explorers, das die Dateien von Android öffnet.
- Den APK Installer, der `.apk`-Dateien öffnet.
- Den DSA Window Manager (`dsa-wm`). Er startet bei Ihrer Anmeldung und startet dann Android,
  sodass die erste App, die Sie öffnen, nicht auf den Start von Android wartet.

Jeder Windows-Benutzer hat ein eigenes Android und einen eigenen Datenträger,
`%LOCALAPPDATA%\DSA\android.vhdx`. Andere Benutzer können ihn nicht lesen.

DSA verwendet einen eigenen Kernel und eine eigene virtuelle Maschine. Es verwendet oder ändert
weder Ihre WSL-Einstellungen noch Ihren WSL-Kernel oder Ihre Distributionen.

## Updates

DSA aktualisiert sich selbst und das Android-Abbild. Beide sind signiert, und DSA wendet nichts
an, dessen Signatur, Größe oder Hash nicht übereinstimmt.

- **DSA selbst** wird aktualisiert, wenn bei keinem Benutzer Android läuft. Bis dahin wartet das
  Update; `dsa update` installiert es sofort. DSA wechselt nie zu einer niedrigeren Version.
- **Release-Kanäle.** Ab 1.0 hat DSA einen stabilen Kanal und einen Vorabversionskanal für
  Versionen, die noch fertiggestellt werden. Ihr Kanal richtet sich zunächst nach dem
  verwendeten Installationsprogramm, und Upgrades behalten ihn bei. Ein Administrator ändert ihn
  mit **Get pre-release versions** (Vorabversionen erhalten) auf der Seite **Updates** von DSA
  Settings. Jedes Release vor 1.0 ist eine Vorabversion.
- **Das Android-Abbild** hängt nicht vom Kanal ab. Startet ein Abbild nicht, kehrt DSA zum
  vorherigen zurück, und `dsa status` meldet das.

Ein Administrator kann Updates auf der Seite **Updates** von DSA Settings ausschalten. Unter
[Einstellungen](settings.md) finden Sie jeden Schalter.

## Linux (in Arbeit, nicht veröffentlicht)

DSA für Linux wird als `.deb` (Debian, Ubuntu) und als `.rpm` (Fedora, openSUSE) gebaut. Es
benötigt KVM (`/dev/kvm`) und einen Wayland-Desktop. Android läuft in einer virtuellen Maschine
mit crosvm.

Die Installation eines Pakets fügt auch das signierte Paket-Repository von DSA hinzu, sodass
spätere Versionen über `apt`, `dnf`, `zypper` oder das Softwarecenter kommen. Unter Linux hat
DSA keinen eigenen Updater.

- Debian und Ubuntu: `/etc/apt/sources.list.d/dsa.sources`, mit dem Schlüssel in
  `/usr/share/keyrings/dsa-archive-keyring.gpg`.
- Fedora und openSUSE: `/etc/yum.repos.d/dsa.repo` (und `/etc/zypp/repos.d/dsa.repo`, wo zypper
  vorhanden ist), mit dem Schlüssel in `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`.

Das Repository soll unter
`https://thisisthepy.github.io/desktop-subsystem-for-android/` bereitgestellt werden. Es ist
noch nicht veröffentlicht.

Nach der Veröffentlichung sieht die erste Installation so aus:

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

Das Paket installiert den Systemdienst `dsa-hostd`, die Benutzerdienste `dsa-devices` und
`dsa-compositor`, den Befehl `dsa` in `/usr/bin`, den Menüeintrag **DSA Files** und die
Dateiaktionen **Open in an Android app**, **Share with an Android app** und **Install in
Android**.

## macOS (in Arbeit, nicht veröffentlicht)

DSA für macOS führt Android auf dem Virtualization Framework von Apple aus, auf Apple silicon.
Es wird als `DSA.app` gebaut, in einem Installationspaket (`DSA-<version>.pkg`) und einem
Datenträgerabbild (`DSA-<version>.dmg`). Die CI-Builds sind ad hoc signiert und laufen daher nur
auf dem Rechner, der sie gebaut hat. Ein mit einer Developer ID signiertes Release gibt es noch
nicht.

## Weiter

- [Apps und Fenster](apps.md)
- [APKs installieren](install-apks.md)

---

Android ist eine Marke von Google LLC. Windows ist eine Marke der Microsoft-Unternehmensgruppe.
macOS ist eine Marke von Apple Inc. DSA steht in keiner Verbindung zu Google, Microsoft oder
Apple und wird von ihnen weder unterstützt noch gesponsert.
