# Speicher und Verhalten im Leerlauf

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · [简体中文](../zh-CN/memory.md) · [繁體中文](../zh-TW/memory.md) · [Español](../es/memory.md) · [Français](../fr/memory.md) · **Deutsch** · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · [Tiếng Việt](../vi/memory.md) · [Bahasa Indonesia](../id/memory.md) · [Türkçe](../tr/memory.md)

Desktop Subsystem for Android™ (DSA) gibt Speicher an Ihren PC zurück, während Android läuft,
nicht erst, wenn es stoppt. Dafür müssen Sie nichts einrichten. Diese Seite erklärt, was
passiert und wie Sie es sehen.

## Sehen, wie viel Speicher Android belegt

Öffnen Sie die Seite **Memory** (Speicher) von DSA Settings, oder führen Sie `dsa memory` aus.
Beide zeigen:

- **On this PC** (auf diesem PC): was die virtuelle Maschine von Android, der Fenstermanager und
  jedes App-Fenster belegen.
- **Inside Android** (in Android): belegter Speicher, Cache, komprimierter Speicher und die
  größten Prozesse. Android meldet das einige Minuten nach dem Start.

## Doze, der Ruhezustand

Sind alle Android-Fenster seit 30 Sekunden geschlossen oder minimiert, schaltet sich der
Bildschirm von Android aus, und Android wechselt in Doze, wie ein Telefon in der Tasche.

- Benachrichtigungen, Wecker und Widgets funktionieren in Doze weiter. Dazwischen ruht die CPU.
- `dsa status` meldet `Dozing`. Die Seite **General** von DSA Settings meldet **Dozing (idle)**.
- Sobald ein Android-Fenster angezeigt wird, wacht Android sofort auf.

Android folgt auch dem Energiesparmodus Ihres PCs. Es speichert seinen Zustand, bevor der PC in
den Energiesparmodus wechselt, und stellt seine Uhr, wenn der PC aufwacht.

## Einfrieren (standardmäßig aus)

Das Einfrieren stoppt jeden Prozess in Android. Android verbraucht dann keine CPU, aber auch
Benachrichtigungen, Wecker und Widgets stoppen. Deshalb friert DSA Android nie ein, wenn Sie es
nicht verlangen.

- Sofort einfrieren: **Freeze** unter **Freeze Android now** auf der Seite **General**, oder
  `dsa freeze`.
- Nach 30 Minuten in Doze einfrieren: Schalten Sie **Freeze Android when idle** auf der Seite
  **Advanced** ein, oder `dsa settings set freeze on`.
- Wer ein Android-Fenster öffnet oder darin eine Eingabe macht, setzt Android fort.

## Wie Speicher zurückgegeben wird

- Die virtuelle Maschine von Android nimmt Ihrem PC erst dann Speicher, wenn Android ihn zum
  ersten Mal verwendet.
- Speicher, den Android freigibt, geht an Ihren PC zurück, während Android läuft.
- Schließt sich das letzte Fenster einer App, beendet Android die Prozesse dieser App, sodass ihr
  Speicher innerhalb von Sekunden zurückgeht. Eine App mit einem Widget auf dem Desktop oder eine,
  die noch im Hintergrund arbeitet (etwa ein Musikplayer), läuft weiter.
- Eine App, die einen Hintergrunddienst ohne Fenster, ohne Benachrichtigung, ohne Ton und ohne
  Widget laufen lässt, wird nach fünf Minuten gestoppt. Startet Android sie immer wieder, wartet
  DSA jedes Mal länger, bis zu einer Stunde.
- Solange kein App-Fenster geöffnet ist, verwirft Android seine Datei-Caches und verschiebt
  ungenutzten Speicher in komprimierten Speicher, damit mehr davon zurückgehen kann.
- Der Fenstermanager gibt den Speicher, den er nicht verwendet, 30 Sekunden nach dem Schließen
  des letzten App-Fensters zurück.

## Speicherlimit

Das Limit ist standardmäßig **Automatic** (automatisch), was kein festes Limit setzt. Android
startet mit 2 GB und wächst während der Laufzeit bis zum Speicher Ihres PCs abzüglich dessen, was
Windows behält (10 % des Speichers oder 2 GB, je nachdem, was größer ist). Android belegt nur, was
es verwendet.
Um ein Limit festzulegen, verwenden Sie **Memory limit** auf der Seite **Memory**, oder:

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # zurück zu automatisch
```

Ein neues Limit gilt ab dem nächsten Start von Android.

---

Android ist eine Marke von Google LLC. Windows ist eine Marke der Microsoft-Unternehmensgruppe.
DSA steht in keiner Verbindung zu Google oder Microsoft und wird von ihnen weder unterstützt noch
gesponsert.
