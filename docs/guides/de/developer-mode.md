# Entwicklermodus und adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · **Deutsch** · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

Mit dem Entwicklermodus kann sich die Android Debug Bridge (adb) auf Ihrem PC mit Ihrem Android
in Desktop Subsystem for Android™ (DSA) verbinden. Er ist standardmäßig ausgeschaltet.

adb gehört nicht zu DSA. Sie erhalten es mit den Android SDK Platform-Tools.

## adb verbinden

1. Öffnen Sie die Seite **Network** (Netzwerk) von DSA Settings und schalten Sie **Developer
   mode** (Entwicklermodus) ein. Oder führen Sie `dsa settings set developer-mode on` aus.
2. Starten Sie Android, falls es nicht läuft: `dsa start --wait`.
3. Verbinden Sie sich:

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android fragt nicht, ob Sie die Verbindung zulassen. Der Port antwortet nur auf diesem PC,
   daher können sich andere Computer nicht verbinden.
4. Wenn Sie fertig sind, schalten Sie **Developer mode** wieder aus, oder führen Sie
   `dsa settings set developer-mode off` aus.

**Connect adb** (adb verbinden) auf der Seite **Network** zeigt den genauen Befehl, und
`dsa settings` gibt ihn in der Zeile `Developer mode` aus.

## Einen anderen Port verwenden

Der Standardport ist 58526. Um ihn zu ändern, bearbeiten Sie **Port** unter **Connect adb**,
oder führen Sie aus:

```
dsa settings set adb-port 5555
```

## Was der Entwicklermodus nicht ändert

DSA verwendet adb nie für seine eigene Arbeit. Fenster, Eingabe und Installationen nutzen den
eigenen Steuerkanal von DSA, daher funktioniert alles gleich, wenn der Entwicklermodus
ausgeschaltet ist.

Um das Protokoll von Android zu lesen, brauchen Sie kein adb. Siehe
[Fehlerbehebung](troubleshooting.md).

---

Android ist eine Marke von Google LLC. DSA steht in keiner Verbindung zu Google und wird von
Google weder unterstützt noch gesponsert.
