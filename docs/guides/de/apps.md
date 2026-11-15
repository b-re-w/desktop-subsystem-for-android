# Apps und Fenster

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · [简体中文](../zh-CN/apps.md) · [繁體中文](../zh-TW/apps.md) · [Español](../es/apps.md) · [Français](../fr/apps.md) · **Deutsch** · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · [Tiếng Việt](../vi/apps.md) · [Bahasa Indonesia](../id/apps.md) · [Türkçe](../tr/apps.md)

Jeder Task einer Android-App öffnet sich als eigenes Desktop-Fenster mit eigener Schaltfläche in
der Taskleiste. Es gibt keinen Emulatorrahmen um einen Telefonbildschirm. Diese Seite beschreibt
Desktop Subsystem for Android™ (DSA) unter Windows.

## Eine App öffnen

- **Startmenü.** Jede installierte App hat einen Startmenüeintrag mit eigenem Namen und Symbol,
  sodass die Windows-Suche sie findet. Beim Öffnen startet zuerst Android, wenn Android nicht
  läuft.
- **DSA Settings.** Wählen Sie auf der Seite **Apps** neben der App **Open** (Öffnen).
- **Befehlszeile.** `dsa apps list` listet die installierten Apps mit ihren Paketen auf.
  `dsa launch <package>` öffnet eine davon, zum Beispiel `dsa launch com.android.settings`.

Hat der Startmenüeintrag eines anderen Programms bereits den Namen der App, ergänzt DSA den Namen
um das Paket, etwa `Clock (com.example.clock)`. DSA ändert oder entfernt nur die Einträge, die es
selbst angelegt hat.

## Desktopverknüpfungen und Sprunglisten

App-Symbole funktionieren wie die von Windows-Apps, nicht wie auf einem Android-Startbildschirm.

- **Desktopverknüpfung.** Wählen Sie auf der Seite **Apps** von DSA Settings neben der App
  **Create desktop shortcut** (Desktopverknüpfung erstellen). Sie können auch mit der rechten
  Maustaste auf die Taskleistenschaltfläche der App klicken und **Create desktop shortcut**
  wählen. Über die Befehlszeile: `dsa apps desktop-shortcut <package>`. Die Verknüpfung öffnet
  die App so wie ihr Startmenüeintrag.
- **App-Verknüpfungen.** Die Verknüpfungen, die eine Android-App anbietet, etwa ein neuer Chat in
  einem Messenger, erscheinen, wenn Sie mit der rechten Maustaste auf die Taskleistenschaltfläche
  oder den Startmenüeintrag der App klicken. Eine davon öffnet den entsprechenden Bildschirm der
  App in einem Fenster und startet zuerst Android, wenn es nicht läuft. Ändert die App ihre
  Verknüpfungen, folgt die Liste. Über die Befehlszeile: `dsa apps shortcut <package> <id>`;
  `dsa apps info <package> --json` listet die IDs auf.
- **Verknüpfungen, die eine App anfordert.** Möchte eine App eine ihrer Verknüpfungen auf Ihrem
  Desktop ablegen, fragt DSA Sie zuerst und legt sie nur an, wenn Sie zustimmen.

Ändert eine App ihren Namen, etwa nach einem Update, folgen ihr Startmenüeintrag, ihre
Desktopverknüpfungen und ihre Anheftung an die Taskleiste dem neuen Namen und funktionieren
weiter. Eine Desktopverknüpfung, die Sie gelöscht haben, wird nicht neu angelegt. Wer eine App
deinstalliert, entfernt auch ihre Desktopverknüpfungen.

## Mit App-Fenstern arbeiten

- Jeder Task hat ein eigenes Fenster, eine eigene Taskleistenschaltfläche und einen eigenen
  Eintrag bei Alt+Tab, mit Name und Symbol der App. Der Task-Manager führt jede App unter ihrem
  eigenen Namen.
- Die Titelleiste gehört zur App: Die Farben der App reichen bis an den oberen Fensterrand, und
  die Titelleistenschaltflächen von Windows 11 liegen darüber. Symbol und Name der App stehen
  links. Kann die App zurückgehen, wird das Symbol zu einem Zurück-Pfeil; wählen Sie ihn, um
  zurückzugehen.
- Andocklayouts (Zeiger über die Maximieren-Schaltfläche halten), Ziehen der Titelleiste,
  Doppelklick darauf zum Maximieren und Größenänderung in Echtzeit funktionieren. Eine
  Größenänderung startet die App nicht neu. Ein maximiertes Fenster füllt den Arbeitsbereich des
  Monitors und lässt die Taskleiste sichtbar.
- Es gibt keine Android-Navigationsleiste. Die untere Leiste einer App sitzt am unteren
  Fensterrand, ohne leeren Streifen darunter.
- Maus (mit Hover), Toucheingabe, Stift (Druck, Radierer und Zylindertaste) und Tastatur
  funktionieren. Text kommt von Ihrer Windows-Eingabemethode, auch für Koreanisch und andere
  Sprachen; die Zeichenzusammensetzung erscheint direkt im Android-Editor, und das
  Kandidatenfenster sitzt neben dem Cursor. Es gibt keine Android-Tastatur-App.
- Ziehen Sie Dateien aus dem Explorer auf das Fenster einer App, oder ziehen Sie Dateien aus
  einer App in den Explorer.
- Bildschirmleseprogramme wie die Sprachausgabe (Narrator) und NVDA lesen die App über UI
  Automation.
- Wer ein Fenster schließt, bittet Android, diesen Task zu beenden.

## Das Fenstermenü

Klicken Sie mit der rechten Maustaste auf die Titelleiste, wählen Sie das Symbol der App, oder
drücken Sie Alt+Leertaste. Nach den üblichen Windows-Einträgen enthält das Menü:

- **Full screen** (Vollbild): füllt den ganzen Bildschirm, wie F11. Wählen Sie es erneut oder
  drücken Sie F11, um es zu verlassen.
- **New window** (neues Fenster): öffnet ein weiteres Fenster der App, für Apps, die mehrere
  unterstützen.
- **Screenshot**: kopiert den Bildschirm der App ohne Titelleiste in die Zwischenablage und
  speichert ihn im Ordner Screenshots in Ihren Bildern.
- **Aspect ratio** (Seitenverhältnis): öffnet die Android-Einstellung für das Seitenverhältnis
  der App, für Apps, die in einem großen Fenster gestreckt aussehen.
- **Open by default** (standardmäßig öffnen): legt fest, ob die Links, die die App unterstützt,
  in der App oder in Ihrem Browser geöffnet werden.
- **App info** (App-Info): öffnet den Eintrag der App auf der Seite **Apps** von DSA Settings.

## Vollbild

Fordert eine App Vollbild an, zeigt ihr Fenster es auf eine von zwei Arten:

- **Auf dem ganzen Bildschirm**: Das Fenster füllt den Monitor, und die Titelleiste wird
  ausgeblendet. Berühren Sie mit dem Mauszeiger den oberen Bildschirmrand, um sie mit Minimieren
  und Schließen einzublenden; die mittlere Schaltfläche verlässt das Vollbild.
- **Im Fenster**: Das Fenster behält seine Größe und Titelleiste, und die App füllt es aus.

Apps fordern es auf zwei Arten an, und jede Art hat ihren eigenen Standard:

| Art der Anforderung | Beispiel | Standard |
|---|---|---|
| Eine klare Vollbildanforderung | Die Vollbild-Schaltfläche eines Videoplayers | Auf dem ganzen Bildschirm |
| Nur Ausblenden der Leisten | Ein Feed mit kurzen Videos | Im Fenster |

Ändern Sie die Standards unter **App full screen** und **Apps that hide their bars** auf der
Seite **Display & GPU** von DSA Settings, oder für eine einzelne App unter **Full screen request** und
**When only the bars hide** auf der Seite **Apps**. Die eigene Wahl einer App hat Vorrang. F11 und **Full
screen** im Fenstermenü schalten jederzeit das Vollbild auf dem ganzen Bildschirm ein und aus,
unabhängig von der Einstellung.

Über die Befehlszeile:

```
dsa settings set app-fullscreen screen|window           # Standard für klare Anforderungen
dsa settings set app-immersive screen|window            # Standard für ausgeblendete Leisten
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## Fenstergröße einer App festlegen

Auf der Seite **Apps** von DSA Settings:

- Schalten Sie **Resizable** (Größe änderbar) aus, damit die Fenster der App die Größe behalten,
  mit der sie sich öffnen.
- Wählen Sie die Größe, mit der sich die Fenster der App öffnen: die Größe, die Android wählt,
  oder eine Telefon- oder Tablet-Größe im Hoch- oder Querformat.

Die Änderung gilt für das nächste Fenster, das die App öffnet. Über die Befehlszeile:

```
dsa app-window <package> --fixed-size on        # Größe nicht änderbar, nicht maximierbar
dsa app-window <package> --start-size 1280x800  # öffnet in dieser Größe; auto lässt Android wählen
dsa app-window <package> --dpi-scaling system   # Skalierung auf einem anderen Monitor; siehe unten
dsa app-window <package> reset                  # zurück zu den Standards
```

## Eine App unter der Titelleiste beginnen lassen

Eine App zeichnet bis zum oberen Rand ihres Fensters, unter die Titelleiste. Einige Apps
quetschen dann ihre obere Leiste oder verdecken einen Teil davon unter der Titelleiste.
Schalten Sie für eine solche App auf der Seite **Apps** von DSA Settings **Start below the
title bar** (unter der Titelleiste beginnen) ein. Der Inhalt der App beginnt dann unter der Titelleiste, und die
Titelleiste nimmt die Farbe an, die die App dort zeigt. Standardmäßig ist es aus, und es gilt
ab dem nächsten Öffnen des App-Fensters. In der Befehlszeile:

```
dsa app-window <package> --below-title-bar on   # off kehrt zum Standard zurück
```

## Eine App auf einen Monitor mit anderer Skalierung verschieben

Android startet mit der Skalierung Ihres Hauptmonitors, sodass Apps dort in der richtigen
Größe öffnen. Verschieben Sie ein Fenster
auf einen Monitor mit anderer Skalierung, behält die App ihre Größe auf dem Bildschirm. Jede App
tut das auf eine dieser Arten, die Sie unter **Scaling** (Skalierung) auf der Seite **Apps** oder mit `dsa app-window <package> --dpi-scaling`
wählen:

- **Automatic** (`auto`, der Standard): **System** für die Apps auf der eingebauten Liste von
  DSA mit Apps, die fehlerhaft werden, wenn sich ihre Skalierung zur Laufzeit ändert, und
  **App** für jede andere App. Die Liste enthält YouTube: Es behält sein Layout für die
  Skalierung, mit der es gestartet ist, sodass seine unteren Registerkarten und der Miniplayer
  nach einem Verschieben bis zu einem Neustart fehlerhaft blieben. Die Einstellungen zeigen den
  Standard einer solchen App als **Automatic (system, recommended for this app)**. Ihre eigene
  Wahl für die App hat Vorrang.
- **App** (`app`): Während Sie das Fenster ziehen, skaliert DSA es auf den neuen Monitor. Lassen
  Sie los, zeichnet sich die App in der Skalierung des neuen Monitors neu, und ihr Text bleibt
  scharf: Android erstellt ihren Bildschirm einmal neu und stellt ihren Zustand wieder her
  (Bildlaufposition, eingegebener Text). Sie zeichnet sich daher nach jedem Verschieben kurz neu,
  wie ein Windows-Programm mit der Skalierung „Anwendung“. Das gilt sogar für eine App, die
  angibt, eine Änderung der Skalierung selbst zu verarbeiten, weil manche dieser Apps nur einen
  Teil ihres Bildschirms in der neuen Skalierung neu zeichnen.
- **System** (`system`): Die App behält ihre Skalierung, und DSA skaliert ihr Fenster auf den
  neuen Monitor. Die App wird nie neu erstellt. Vergrößert kann Text etwas weich aussehen.
- **System (forced)** (`system-forced`): Die App erhält die neue Skalierung ohne Neustart, auch
  wenn sie nicht angibt, eine solche zu verarbeiten. Noch nicht verfügbar: Dafür ist eine
  Änderung an Android nötig, die diese Version nicht hat. Bis dahin verhält es sich wie
  **System**.

Text in Apps folgt der Textgröße von Windows (Einstellungen > Barrierefreiheit > Textgröße).

## Benachrichtigungen

Android-Benachrichtigungen erscheinen als Windows-Benachrichtigungen, mit Name und Symbol der
App.

- Wählen Sie eine Benachrichtigung, um die App zu öffnen.
- Verwerfen Sie sie, wird sie auch in Android verworfen, es sei denn, die App lässt sie
  angezeigt.
- Benachrichtigungen kommen weiter an, während Android in Doze ist. Siehe [Speicher](memory.md).

## Apps im Hintergrund

Eine App, die weiterläuft, nachdem ihr Fenster geschlossen wurde, etwa ein Musikplayer, erscheint
als Symbol im Infobereich. Ihre QuickInfo ist die Benachrichtigung der App.

- Wählen Sie das Symbol, um die App zu öffnen.
- Ein Rechtsklick darauf bietet **Open** (Öffnen), die eigenen Aktionen der Benachrichtigung und
  **Stop** (Beenden).
- **Stop** fragt zuerst. Setzen Sie ein Häkchen bei **Do not ask again** (nicht mehr fragen), um
  die Frage künftig zu überspringen. Um wieder gefragt zu werden, schalten Sie **Ask before
  stopping an app from the notification area** auf der Seite **Running in the background** von
  DSA Settings ein.
- Eine App, die Medien abspielt, funktioniert auch mit den Medientasten und den
  Windows-Mediensteuerelementen.

Die Seite **Running in the background** (im Hintergrund ausgeführt) von DSA Settings listet
diese Apps auf, mit **Open** und **Stop** für jede. Über die Befehlszeile: `dsa background`,
`dsa background open <package>` und `dsa background stop <package>`.

## Widgets auf dem Desktop

Ein Android-Widget kann in einem eigenen Fenster auf Ihrem Desktop liegen. Es bleibt unter
anderen Fenstern, bleibt sichtbar, wenn Sie den Desktop anzeigen, und erscheint weder in der
Taskleiste noch bei Alt+Tab.

1. Öffnen Sie die Seite **Widgets** von DSA Settings.
2. Wählen Sie unter **Widgets your apps offer** (Widgets Ihrer Apps) neben einem Widget **Add to
   desktop** (zum Desktop hinzufügen). Bittet die App Sie, das Widget einzurichten, öffnet sich
   ihre Einrichtung in einem Fenster.
3. Um ein Widget zu verschieben, drücken Sie an einer beliebigen Stelle darauf und halten eine
   halbe Sekunde still. Es hebt sich an, mit einem Rahmen und einem Griff an jeder Kante, entlang
   der sich seine Größe ändern lässt. Ziehen Sie es, um es zu verschieben, oder ziehen Sie einen
   Griff, um seine Größe zu ändern. Ein schnelles Ziehen bleibt beim Widget selbst, sodass seine
   Listen weiterhin scrollen. Sie können auch eine Kante des Widgets ziehen, um seine Größe zu
   ändern, ohne es zuerst anzuheben; das geht nur an den Kanten, entlang der sich seine Größe
   ändern lässt. DSA merkt sich, wo Sie es abgelegt haben und wie groß es ist.
4. Klicken Sie mit der rechten Maustaste auf das Widget, oder lassen Sie nach dem langen Drücken
   los, für **Widget settings** (Widget-Einstellungen, wenn sich das Widget erneut einrichten
   lässt), **Resize** (Größe ändern), um eine der erlaubten Größen zu wählen, und **Remove**
   (Entfernen). Die Menütaste und Umschalt+F10 öffnen das Menü ebenfalls. Die Pfeiltasten, die
   Eingabetaste und Esc funktionieren in diesem Menü, und ein Buchstabe wählt einen Eintrag: R
   für **Resize** und M für **Remove**.

Über die Befehlszeile: `dsa widgets list`, `dsa widgets add <provider>` und
`dsa widgets remove <id>`.

## Links und die Zwischenablage

- Ein Weblink, den Sie in einer Android-App antippen, öffnet sich in Ihrem Standardbrowser. Ein
  Link, den eine installierte App als ihren eigenen bestätigt hat, öffnet sich in dieser App.
- Um stattdessen jeden Weblink in Android-Apps zu öffnen, schalten Sie **Open web links in
  Android apps** auf der Seite **Advanced** (Erweitert) von DSA Settings ein
  (`dsa settings set links-in-android on`).
- Text, den Sie kopieren, wird in beide Richtungen zwischen Windows und Android geteilt. Um das
  zu beenden, schalten Sie **Share the clipboard with Android** auf der Seite **Advanced** aus
  (`dsa settings set clipboard off`).

## Google Play und geschützte Videos

DSA liefert keine Google-Software mit. Wenn Sie sie möchten, lädt DSA sie für Sie von ihrer
Quelle herunter, nachdem es Ihnen die Quelle und die Bedingungen gezeigt hat. Beides gilt ab dem
nächsten Start von Android, und beides lässt sich wieder entfernen.

- **Google Play.** Wählen Sie auf der Seite **Advanced** von DSA Settings unter **Google Play**
  die Option **Install**, oder führen Sie `dsa gapps install` aus. DSA lädt das öffentliche Paket
  LiteGapps Core (Google Play-Dienste, Play Store und Kontosynchronisierung) für Ihre
  Android-Version herunter. Dieses Android ist nicht von Google zertifiziert, daher verweigert
  Google möglicherweise die Anmeldung, bis Sie das Gerät registrieren: **Register this device
  with Google** zeigt die Android-ID und öffnet die Registrierungsseite. `dsa gapps status` zeigt
  den Zustand, und `dsa gapps remove` entfernt die Apps von Google, auch während Android gestoppt
  ist.
- **Widevine.** Streaming-Apps benötigen das Widevine-Modul von Google, um geschützte Videos
  abzuspielen. Wählen Sie auf der Seite **Advanced** unter **Protected content** (geschützte
  Inhalte) die Option **Get**, oder führen Sie `dsa drm widevine install` aus. DSA lädt das
  Android-Emulator-Abbild von Google für Ihre Android-Version herunter (etwa 2 GB), behält nur
  das Widevine-Modul, unverändert, und löscht den Rest. Widevine hat hier die Sicherheitsstufe
  L3, daher senden Streamingdienste höchstens Standardauflösung; für hohe Auflösung verwenden Sie
  einen Windows-Browser oder eine Windows-App. `dsa drm widevine remove` entfernt es.

## Apps nur für ARM

Auf einem x64-PC läuft eine App, die nur Code für ARM-Prozessoren enthält, über einen
ARM-Übersetzer. Android auf einem x64-PC bringt einen mit, den Open-Source-Übersetzer Digitalis,
und verwendet ihn ab dem ersten Start. Sie können stattdessen einen anderen verwenden:

- **Der ARM-Übersetzer von Google.** Wählen Sie auf der Seite **Advanced** neben **Google's ARM
  translator** die Option **Get**, oder führen Sie `dsa arm-translation fetch google` aus. DSA
  zeigt zuerst, woher er stammt, und die Bedingungen von Google (die
  [Android SDK License](https://developer.android.com/studio/terms)) und lädt nichts herunter,
  bevor Sie zustimmen. Ihr PC lädt dann das Android-Emulator-Abbild herunter, das Google für Ihre
  Android-Version anbietet (etwa 2 GB), behält nur den Übersetzer und löscht den Rest. DSA liefert
  den Übersetzer von Google weder mit noch gibt es ihn weiter.
- **Ein Paket, das Sie selbst bauen oder anderswo beziehen**, etwa eines, das aus dem
  Open-Source-Übersetzer Digitalis erstellt wurde: `dsa arm-translation install <folder>`.

Starten Sie dann Android neu: Stoppen Sie es auf der Seite **General** (Allgemein) von DSA
Settings und öffnen Sie dann eine App.

Um zum Übersetzer zurückzukehren, der mit Android geliefert wird, wählen Sie auf der Seite
**Advanced** neben **Use the default translator** die Option **Use**, oder führen Sie
`dsa arm-translation use-default` aus. Das entfernt das von Ihnen installierte Paket, ab dem
nächsten Start von Android.

`dsa arm-translation status` gibt an, welchen Übersetzer Android verwendet. Der Schalter **Run
apps built only for ARM** auf der Seite **Advanced** schaltet die Übersetzung ein oder aus, ohne
etwas zu entfernen. Für eine App mit ARM- und x64-Code führt **Run as ARM** auf der Seite **Apps**
(`dsa apps run-as-arm <package> on`) stattdessen ihren ARM-Code aus. Nur 64-Bit-ARM-Apps
(`arm64-v8a`) können laufen. Auf einem ARM64-PC führt Android ARM-Apps selbst aus und braucht
keinen Übersetzer.

## Root-Zugriff

Manche Apps, etwa Sicherungswerkzeuge, benötigen Root. DSA gewährt ihn so, wie `sudo` es auf
einem Linux-PC tut: nur wenn ein Administrator des PCs zustimmt, eine Anfrage nach der anderen.
Er ist standardmäßig ausgeschaltet, und Apps, die nicht zugelassen sind, sehen `su` überhaupt
nicht.

1. Ein Administrator schaltet **Let Android apps ask for root** unter **Root access** auf der
   Seite **Advanced** von DSA Settings ein (`dsa settings set root-elevation on`). Das gilt ab
   dem nächsten Start von Android.
2. Legen Sie auf der Seite **Apps** den **Root access** (Root-Zugriff) der App fest:
   - **Root: not allowed** (nicht erlaubt, der Standard): Die App hat kein `su`.
   - **Root: ask each time** (jedes Mal fragen): Jedes Mal, wenn die App `su` verwendet, zeigt
     Windows seine Administratorabfrage (Benutzerkontensteuerung, UAC) unter dem eigenen Namen
     und Symbol der App. Ein Standardbenutzer gibt das Kennwort eines Administrators ein. Eine
     Zustimmung gilt für diese App 15 Minuten.
   - **Root: remember (no asking)** (merken, ohne Nachfrage): Die App verwendet `su`, ohne zu
     fragen. Diese Wahl erfordert die Zustimmung eines Administrators.

   Über die Befehlszeile: `dsa apps root <package> off|ask|remember`.

Bei eingeschaltetem Entwicklermodus kann auch `adb shell` `su` verwenden. Dafür gibt es auf der
Seite **Apps** eine eigene Zeile, **adb shell**, die standardmäßig jedes Mal fragt
(`dsa apps root adb ask`). Ihre Abfrage heißt **adb shell** und zeigt den Befehl.

Fragt ein Teil von Android selbst, nennt die Abfrage ihn: **Android system**, **Android phone
services**, **Android Bluetooth**, **Android NFC** oder **Android system service**. Apps, die
eine Benutzer-ID teilen, werden als eine Gruppe zugelassen, und die Abfrage führt sie alle auf.

Jede Anfrage wird im Dienstprotokoll festgehalten, mit der App, der Uhrzeit, dem Befehl und ob
sie genehmigt wurde. Wer die Funktion ausschaltet, verbirgt `su` sofort vor jeder App, ohne
Neustart.

---

Android ist eine Marke von Google LLC. Windows ist eine Marke der Microsoft-Unternehmensgruppe.
DSA steht in keiner Verbindung zu Google oder Microsoft und wird von ihnen weder unterstützt noch
gesponsert.
