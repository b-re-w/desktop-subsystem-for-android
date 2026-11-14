# Per iniziare

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · **Italiano** · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™ (DSA) esegue Android in una macchina virtuale leggera con un
proprio kernel. Ogni app Android si apre in una propria finestra sul desktop.

## Stato per piattaforma

| Host | Stato |
|---|---|
| Windows 11, x64 | Rilasciato. |
| Windows 11, ARM64 | Rilasciato. Anche Android è ARM64, quindi le app ARM funzionano senza traduzione. |
| Linux, x64 e ARM64 | In sviluppo. I pacchetti `.deb` e `.rpm` vengono compilati nella CI; non ancora rilasciato. |
| macOS, Apple silicon | In sviluppo. `DSA.app`, il suo `.pkg` e il suo `.dmg` vengono compilati nella CI; non ancora rilasciato. |

## Installare su Windows

Serve Windows 11 su un PC x64 o ARM64, con la funzionalità di Windows **Piattaforma macchina
virtuale** attivata.

1. Attivi Piattaforma macchina virtuale. Apra **Attiva o disattiva funzionalità di Windows**,
   selezioni **Piattaforma macchina virtuale**, scelga **OK** e riavvii. Oppure esegua
   `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` in un terminale da
   amministratore. Il programma di installazione non la attiva al posto Suo: se la
   funzionalità è disattivata, si interrompe con un messaggio.
2. Scarichi il programma di installazione e `SHA256SUMS.txt`
   dall'[ultima versione](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest):
   `DSA-v<version>-win-x64-setup.exe` su un PC x64, `DSA-v<version>-win-arm64-setup.exe` su un
   PC ARM64.
3. Verifichi il file scaricato. L'hash stampato da questo comando deve corrispondere alla riga
   in `SHA256SUMS.txt`:

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. Esegua il programma di installazione, selezioni **Install** e approvi la richiesta di
   amministratore.
5. Attenda la prima immagine Android. Quando il programma di installazione non ne contiene
   una, il servizio DSA la scarica in background. `dsa status` mostra il download.
6. Apra un'app dal menu Start, oppure verifichi da un terminale:

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## Che cosa installa il programma di installazione

- Il servizio `DsaHost`, che esegue l'Android di ogni utente.
- Il comando `dsa`.
- **DSA Settings** nel menu Start.
- **Android** nel riquadro di spostamento di Esplora file, che apre i file di Android.
- L'APK Installer, che apre i file `.apk`.
- Il DSA Window Manager (`dsa-wm`). Si avvia quando accede a Windows e in quel momento avvia
  Android, così la prima app che apre non attende l'avvio di Android.

Ogni utente di Windows ha il proprio Android e il proprio disco dati,
`%LOCALAPPDATA%\DSA\android.vhdx`. Gli altri utenti non possono leggerlo.

DSA usa il proprio kernel e la propria macchina virtuale. Non usa né modifica le
impostazioni, il kernel o le distribuzioni di WSL.

## Aggiornamenti

DSA aggiorna se stesso e l'immagine Android. Entrambi sono firmati, e DSA non applica nulla la
cui firma, dimensione o hash non corrisponda.

- **DSA stesso** viene aggiornato quando l'Android di nessun utente è in esecuzione. Fino ad
  allora l'aggiornamento attende; `dsa update` lo installa subito. DSA non passa mai a una
  versione inferiore.
- **Canali di rilascio.** Dalla 1.0, DSA ha un canale stabile e un canale pre-release per le
  versioni ancora in via di completamento. Il Suo canale parte dal programma di installazione
  che ha usato, e gli aggiornamenti lo mantengono. Un amministratore lo cambia con **Get
  pre-release versions** nella pagina **Updates** di DSA Settings. Ogni versione precedente
  alla 1.0 è una pre-release.
- **L'immagine Android** non dipende dal canale. Se un'immagine non si avvia, DSA torna alla
  precedente, e `dsa status` lo segnala.

Un amministratore può disattivare gli aggiornamenti nella pagina **Updates** (Aggiornamenti)
di DSA Settings. Vedere [Impostazioni](settings.md) per tutti gli interruttori.

## Linux (in sviluppo, non rilasciato)

DSA per Linux viene creato come `.deb` (Debian, Ubuntu) e come `.rpm` (Fedora, openSUSE).
Richiede KVM (`/dev/kvm`) e un desktop Wayland. Android viene eseguito in una macchina
virtuale crosvm.

L'installazione di un pacchetto aggiunge anche il repository di pacchetti firmato di DSA,
così le versioni successive arrivano tramite `apt`, `dnf`, `zypper` o il centro software. Su
Linux, DSA non ha un proprio programma di aggiornamento.

- Debian e Ubuntu: `/etc/apt/sources.list.d/dsa.sources`, con la chiave in
  `/usr/share/keyrings/dsa-archive-keyring.gpg`.
- Fedora e openSUSE: `/etc/yum.repos.d/dsa.repo` (e `/etc/zypp/repos.d/dsa.repo` dove è
  presente zypper), con la chiave in `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`.

Il repository sarà servito da
`https://thisisthepy.github.io/desktop-subsystem-for-android/`. Non è ancora pubblicato.

Una volta rilasciato, la prima installazione sarà così:

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

Il pacchetto installa il servizio di sistema `dsa-hostd`, i servizi utente `dsa-devices` e
`dsa-compositor`, il comando `dsa` in `/usr/bin`, la voce di menu **DSA Files** e le azioni
sui file **Open in an Android app**, **Share with an Android app** e **Install in Android**.

## macOS (in sviluppo, non rilasciato)

DSA per macOS esegue Android sul Virtualization framework di Apple, su Apple silicon. Viene
creato come `DSA.app`, in un pacchetto di installazione (`DSA-<version>.pkg`) e in
un'immagine disco (`DSA-<version>.dmg`). Le build della CI hanno una firma ad hoc, quindi
funzionano solo sulla macchina che le ha create. Non esiste ancora una versione firmata con un
Developer ID.

## Passi successivi

- [App e finestre](apps.md)
- [Installare APK](install-apks.md)

---

Android è un marchio di Google LLC. Windows è un marchio del gruppo di società Microsoft.
macOS è un marchio di Apple Inc. DSA non è affiliato a Google, Microsoft o Apple, né approvato
o sponsorizzato da loro.
