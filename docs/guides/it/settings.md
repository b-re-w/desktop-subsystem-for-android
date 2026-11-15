# Impostazioni

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · **Italiano** · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings** è l'app delle impostazioni di Desktop Subsystem for Android™ (DSA) su
Windows. La apra dal menu Start. È disponibile in inglese o in coreano, secondo la lingua di
visualizzazione di Windows, e segue il tema di Windows. Funziona con la tastiera e con le
utilità per la lettura dello schermo.

Ogni modifica fatta da DSA Settings passa dal comando `dsa`, quindi la riga di comando vede gli
stessi valori:

```
dsa settings                        # tutte le impostazioni e i loro valori
dsa settings set <name> <value>     # ne cambia una; gli interruttori accettano on o off
```

Alcune impostazioni valgono per tutti gli utenti del PC, quindi solo un amministratore può
cambiarle. DSA Settings le contrassegna con l'icona di uno scudo e chiede l'approvazione di un
amministratore quando ne cambia una. Qui sotto sono contrassegnate con "(admin)".

## Pagine

| Pagina | Che cosa contiene | Riga di comando |
|---|---|---|
| **General** | Lo stato di Android, **Start**, **Stop**, **Freeze Android now**, l'ultimo problema, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** | **Memory limit**, le dimensioni della VM di Android, quanto usa ciascuna parte | `memory-limit`, `dsa memory` |
| **Apps** | App installate: **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. Per ogni app: **Resizable**, le dimensioni con cui si aprono le finestre, **Scaling**, **Full screen request**, **When only the bars hide**, **Run as ARM**, **Root access**. Una riga per l'accesso root di **adb shell** | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** | App in esecuzione senza finestra: **Open**, **Stop**; se chiedere prima di fermarle | `dsa background`, `stop-without-asking` |
| **Widgets** | I widget sul desktop, e **Add to desktop** | `dsa widgets` |
| **Display & GPU** | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (solo Windows 11, attivo per impostazione predefinita), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** | Prima questo computer, in sola lettura: il nome, il produttore e il modello, e **Device specifications** con **Copy**. Poi il produttore, il modello e l'impronta della build che Android comunica alle app | `dsa settings device`, `device.<field>` |
| **Network** | Come Android raggiunge la rete, il suo indirizzo, chi risponde ai suoi nomi (DNS) e il suo gateway, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** | **Android**: la versione e la build in esecuzione, il livello della patch di sicurezza, la versione che segue, l'aggiornamento di Android con il suo avanzamento e **Check now** (admin), **Restart Android into updates by itself**, **Update the Android image automatically** (admin), **Version to use** (admin). **DSA**: **Update DSA automatically** (admin), **Get pre-release versions** (admin), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** | Appunti, file, link web, congelamento in caso di inattività, backup, ripristino dei dati, app ARM, Google Play, contenuti protetti, accesso root, tracce (admin), cartelle dei log | vedere sotto |
| **Logs** | Il log in tempo reale di Android, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** | Versioni, licenze, codice sorgente | |

## La pagina Advanced

| Impostazione | Predefinito | Riga di comando |
|---|---|---|
| **Share the clipboard with Android** | Attivo | `clipboard` |
| **Share Windows folders with Android** | Attivo | `file-sharing` |
| **Shared folders** | Tutte e sei | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Disattivato | `freeze` |
| **Open web links in Android apps** | Disattivato | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Disattivato | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (non su ARM64) | | `dsa arm-translation fetch google` |
| **Use the default translator** (non su ARM64) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Non installato | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (sotto **Protected content**) | Non installato | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (admin) | Disattivato | `root-elevation` |
| **Input trace** (admin) | Disattivato | `input-trace` |
| **Resize trace** (admin) | Disattivato | `resize-trace` |

Il backup conserva una copia crittografata dei dati delle app nella Sua cartella utente, con
una chiave che Windows custodisce per Lei. Per ripristinarla su un altro PC, o dopo una
reinstallazione di Windows, serve la chiave di ripristino, quindi la annoti e la conservi
lontano dal PC. Google Play, Widevine e l'accesso root sono descritti in
[App e finestre](apps.md).

## Quando si applica una modifica

- **Render GPU**: quando il window manager si riavvia, al successivo accesso a Windows o quando
  seleziona **Restart the window manager**.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for
  ARM**, Google Play, Widevine, l'attivazione dell'accesso root e la versione di Android: al
  successivo avvio di Android. Per riavviare Android, lo fermi nella pagina **General**, poi
  apra un'app.
- Le impostazioni della finestra di un'app: dalla finestra successiva che l'app apre.
- Tutto il resto, compresa la disattivazione dell'accesso root: subito.

## Come Android risolve i nomi

Il computer risponde a ogni nome cercato dalle app Android, con lo stesso resolver usato dalle
app Windows (su Linux e macOS, il resolver del sistema). Così il file hosts, le regole dei nomi
della Sua azienda, i server DNS di una VPN e un filtro DNS si applicano alle app Android come
al computer. Android non interroga direttamente un server DNS.

- Lo stesso vale con **Advanced networking** attivo: il computer si trova sulla stessa rete e
  ne conosce i nomi, e una VPN o un filtro DNS su di esso continua ad applicarsi.
- Il **DNS privato** di Android è disattivato per impostazione predefinita, perché
  interrogherebbe un server DNS aggirando il computer. Se sceglie un server DNS privato nelle
  impostazioni di Android, Android lo usa.
- Se DSA non può rispondere per un certo tempo, Android interroga i server DNS della rete
  finché DSA non torna a rispondere.
- La riga **DNS** della pagina **Network**, e `dsa network`, indicano chi risponde in quel
  momento: il resolver del computer oppure i server DNS della rete, che vengono elencati.

## Valori accettati dalla riga di comando

- `render-gpu`: `display` (la GPU del monitor principale, il predefinito), `integrated`,
  `discrete`, oppure parte del nome di una GPU come la elenca `dsa gpus`.
- `memory-limit`: `auto` (il predefinito: nessun limite fisso, fino alla memoria del PC meno
  quella che Windows conserva), oppure una dimensione come `4g` o `3072` (MiB).
- `shared-folders`: `all`, `none`, oppure nomi separati da virgole, come
  `Documents,Pictures`.
- `adb-port`: una porta TCP da 1 a 65535. Il predefinito è 58526.
- `app-fullscreen`: `screen` (il predefinito) o `window`, per la richiesta esplicita di
  schermo intero di un'app. `app-immersive`: `screen` o `window` (il predefinito), per un'app
  che nasconde soltanto le proprie barre. Vedere [Schermo intero](apps.md#schermo-intero).
- `touch-gestures-off`: `on` (il predefinito) disattiva i movimenti touch a tre e quattro dita
  di Windows 11 mentre una finestra di app DSA è in primo piano, così le app ricevono fino a
  dieci dita, e ripristina la Sua impostazione quando un'altra finestra passa in primo piano.
  Se Windows rifiuta, l'impostazione non si può usare e l'app delle impostazioni ne spiega il
  motivo; può comunque disattivare i movimenti per tutte le app nelle Impostazioni di Windows,
  in Bluetooth e dispositivi > Tocco.
- `host-certificates`: `on` (il predefinito) permette alle app Android di considerare
  attendibili i certificati radice aggiunti a questo PC, come quelli della Sua azienda o di un
  programma di sicurezza.
- `adb-auto-connect`: `on` (il predefinito) mostra Android in `adb devices` e in Android
  Studio mentre la modalità sviluppatore è attiva, senza `adb connect`.
- `image-auto-restart`: `on` (il predefinito) riavvia subito Android in un aggiornamento
  dell'immagine scaricato quando nessuna finestra di app è aperta. Altrimenti DSA chiede prima,
  oppure attende che le chiuda.
- `prerelease-updates`: `on` prende le pre-release oltre alle versioni stabili, `off` prende
  solo le versioni stabili. Il valore iniziale dipende dal programma di installazione usato: un
  programma di installazione pre-release lo attiva, uno stabile lo lascia disattivato, e gli
  aggiornamenti mantengono la Sua scelta. Ogni versione precedente alla 1.0 è una pre-release.
  Disattivarlo non riporta mai DSA a una versione precedente; DSA attende una versione stabile
  più recente di quella installata.
- `root-elevation`: `on` permette alle app autorizzate nella pagina **Apps** di chiedere root.
  Il livello di ogni app si imposta con `dsa apps root <package> off|ask|remember`.
- `device.<field>`: `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint`, oppure una parte dell'impronta come `fingerprint.release`.
  `dsa settings reset device` ripristina tutti i valori.

---

Android è un marchio di Google LLC. Windows è un marchio del gruppo di società Microsoft. DSA
non è affiliato a Google o Microsoft, né approvato o sponsorizzato da loro.
