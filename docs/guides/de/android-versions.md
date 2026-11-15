# Android-Versionen

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · [简体中文](../zh-CN/android-versions.md) · [繁體中文](../zh-TW/android-versions.md) · [Español](../es/android-versions.md) · [Français](../fr/android-versions.md) · **Deutsch** · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · [Tiếng Việt](../vi/android-versions.md) · [Bahasa Indonesia](../id/android-versions.md) · [Türkçe](../tr/android-versions.md)

Desktop Subsystem for Android™ (DSA) unterstützt die zwei neuesten Android-Hauptversionen:
derzeit Android 16 und Android 17. Standardmäßig führt es die neueste aus. Manche Apps laufen nur
auf einer der beiden, deshalb können Sie wählen.

## Sehen, welche Version läuft

Öffnen Sie die Seite **Updates** von DSA Settings. Unter **Android** zeigt sie die Version, ihren
Build, ihre Sicherheitspatch-Ebene, die Version, der sie folgt, und den Stand ihres Updates, mit
dem Fortschritt des Downloads. **Check now** (jetzt prüfen) bittet den Dienst, sofort nach einem
Update zu suchen. Oder führen Sie aus:

```
dsa android version   # die laufende Version und die wählbaren Versionen
```

## Die Version ändern

Alle Benutzer des PCs führen dieselbe Android-Version aus, daher kann nur ein Administrator sie
ändern.

1. Öffnen Sie die Seite **Updates** von DSA Settings.
2. Wählen Sie unter **Version to use** (zu verwendende Version) eine Version oder **Always the
   newest** (immer die neueste).
3. Wählen Sie **Change** (Ändern) und bestätigen Sie die Administratorabfrage.

Die gewählte Version wird heruntergeladen und ab dem nächsten Start von Android verwendet. In
einem Terminal als Administrator:

```
dsa android use 16
dsa android use latest
```

### Der Wechsel zu einer älteren Version löscht Daten

Android kann keine Daten lesen, die eine neuere Version geschrieben hat. Der Wechsel zu einer
älteren Hauptversion löscht daher die Android-Daten aller Benutzer des PCs: die von ihnen
installierten Apps, ihre Konten und alle App-Daten. Die Einstellungen von DSA bleiben erhalten.
Das lässt sich nicht rückgängig machen.

DSA Settings bittet Sie, mit **Erase data and change** (Daten löschen und ändern) zu bestätigen.
Ergänzen Sie in der Befehlszeile `--erase-data`:

```
dsa android use 16 --erase-data
```

Der Wechsel zu einer neueren Version behält Ihre Apps und Daten.

## Updates

- Die gewählte Version erhält weiterhin Sicherheitsupdates.
- Verliert eine Version die Unterstützung, wechselt DSA zur nächsten und behält Ihre Apps.
- DSA wechselt nie von sich aus zu einer älteren Hauptversion.
- Jedes Abbild wird vor der Verwendung anhand einer signierten Liste geprüft. Ein Abbild, das
  nicht startet, wird durch das vorherige ersetzt.

Ein Administrator kann Abbild-Updates mit **Update the Android image automatically** auf der
Seite **Updates** von DSA Settings oder mit `dsa settings set image-updates off` ausschalten.

---

Android ist eine Marke von Google LLC. Windows ist eine Marke der Microsoft-Unternehmensgruppe.
DSA steht in keiner Verbindung zu Google oder Microsoft und wird von ihnen weder unterstützt noch
gesponsert.
