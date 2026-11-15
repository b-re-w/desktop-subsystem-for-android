# Modalità sviluppatore e adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · **Italiano** · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

La modalità sviluppatore permette all'Android Debug Bridge (adb) del PC di connettersi al Suo
Android in Desktop Subsystem for Android™ (DSA). È disattivata per impostazione predefinita.

adb non fa parte di DSA. Lo ottenga dagli Android SDK Platform-Tools.

## Connettere adb

1. Apra la pagina **Network** (Rete) di DSA Settings e attivi **Developer mode**. Oppure
   esegua `dsa settings set developer-mode on`.
2. Avvii Android, se non è in esecuzione: `dsa start --wait`.
3. Si connetta:

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android non chiede di consentire la connessione. La porta risponde solo su questo PC,
   quindi gli altri computer non possono connettersi.
4. Quando ha finito, disattivi di nuovo **Developer mode**, oppure esegua
   `dsa settings set developer-mode off`.

**Connect adb** nella pagina **Network** mostra il comando esatto, e `dsa settings` lo stampa
sulla riga `Developer mode`.

## Usare un'altra porta

La porta predefinita è 58526. Per cambiarla, modifichi **Port** sotto **Connect adb**, oppure
esegua:

```
dsa settings set adb-port 5555
```

## Che cosa la modalità sviluppatore non cambia

DSA non usa mai adb per il proprio lavoro. Finestre, input e installazioni usano il canale di
controllo di DSA, quindi tutto funziona allo stesso modo con la modalità sviluppatore
disattivata.

Per leggere il log di Android non serve adb. Vedere
[Risoluzione dei problemi](troubleshooting.md).

---

Android è un marchio di Google LLC. DSA non è affiliato a Google, né approvato o
sponsorizzato da Google.
