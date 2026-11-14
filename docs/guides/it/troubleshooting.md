# Risoluzione dei problemi e log

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · **Italiano** · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

Questa pagina aiuta a capire perché Desktop Subsystem for Android™ (DSA) non funziona, a
leggere il log di Android e a inviare una segnalazione di problema. Riguarda Windows, e Linux
dove indicato.

## Prima di tutto, controllare lo stato

```
dsa status          # stato di Android, eventuali problemi ed eventuali aggiornamenti in attesa
dsa start --wait    # avvia Android e attende che abbia terminato l'avvio
```

La pagina **General** (Generale) di DSA Settings mostra lo stesso stato.

| `dsa status` | DSA Settings | Significato |
|---|---|---|
| `Active` | **Running** | Android ha terminato l'avvio e l'agente di DSA al suo interno è connesso |
| `Dozing` | **Dozing (idle)** | Android è inattivo. Vedere [Memoria](memory.md) |
| `Frozen` | **Frozen** | Android è congelato finché non apre una delle sue finestre |
| `Stalled` | **Not ready** | Android è in esecuzione ma impiega troppo tempo. La riga successiva spiega perché e che cosa fare |

## Problemi comuni

| Problema | Che cosa fare |
|---|---|
| Il programma di installazione indica che Piattaforma macchina virtuale è disattivata | Attivi la funzionalità di Windows, riavvii ed esegua di nuovo il programma di installazione. Vedere [Per iniziare](getting-started.md) |
| Non si apre alcuna finestra di app | Esca da Windows e acceda di nuovo. Il window manager si avvia all'accesso |
| Un'app si apre con dimensioni sbagliate | `dsa app-window <package> reset` |
| Android non si avvia dopo un aggiornamento dell'immagine | Nulla. DSA torna da solo all'immagine precedente, e `dsa status` lo segnala |
| Android non ha rete | Apra la pagina **Network** di DSA Settings, oppure esegua `dsa network`. Mostra la modalità, l'indirizzo, i server DNS ed eventuali problemi |
| Android è guasto e nient'altro aiuta | Ripristini i dati di Android (vedere sotto) |

### Ripristinare i dati di Android

Il ripristino cancella tutte le app installate, gli account e tutti i dati delle app. Android
si avvia quindi come appena installato. L'immagine Android e le impostazioni di DSA restano.

- In DSA Settings: pagina **Advanced** (Avanzate) > **Reset Android's data** > **Reset**.
- Dalla riga di comando: `dsa reset-data` indica che cosa andrebbe perso.
  `dsa reset-data --yes` lo cancella.

## Leggere il log di Android

Il log di Android resta in memoria in Android e non viene mai scritto su un file. Raggiunge il
PC solo mentre qualcosa lo legge.

- In DSA Settings: la pagina **Logs** (Log). Può filtrare per tag o per testo, e mettere in
  pausa.
- Dalla riga di comando:

  ```
  dsa logcat                        # segue il log; Ctrl+C lo termina
  dsa logcat -d -b all *:W          # ciò che il log contiene ora, avvisi e livelli superiori
  dsa logcat ActivityManager:I *:S  # un solo tag
  ```

## Inviare una segnalazione di problema

1. Salvi la diagnostica. Nella pagina **Logs** selezioni **Save diagnostics**, oppure esegua:

   ```
   dsa logs collect -o diag.zip
   ```

   Richiede un minuto o più.
2. Alleghi il file zip a una
   [issue](https://github.com/thisisthepy/desktop-subsystem-for-android/issues).

L'archivio contiene l'intero log di Android, i suoi registri dei crash (tombstone, tracce ANR,
dropbox, pstore), un riepilogo dei servizi di sistema di Android e i log di DSA su questo PC
che La riguardano, senza le righe degli altri utenti. `summary.txt` nell'archivio indica che
cosa non è stato possibile raccogliere.

## Dove si trovano i log su Windows

| Cartella | File |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (il window manager), `dsa-app-window.log` (le finestre delle app) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (il servizio), `android-<SID>.log` (la console Android di ogni utente, al massimo 2 MiB per avvio), `setup-<version>.log` (aggiornamenti automatici) |

Per aprirli, usi **Open my log folder** e **Open the service's log folder (administrators)**
nella pagina **Advanced** di DSA Settings.

## Dove si trovano i log su Linux (non ancora rilasciato)

I servizi scrivono nel journal di systemd:

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## Tutti i comandi

`dsa` senza argomenti stampa tutti i comandi. Aggiunga `--json` per un output leggibile dai
programmi.

---

Android è un marchio di Google LLC. Windows è un marchio del gruppo di società Microsoft. DSA
non è affiliato a Google o Microsoft, né approvato o sponsorizzato da loro.
