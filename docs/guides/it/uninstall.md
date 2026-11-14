# Disinstallazione

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · **Italiano** · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

Disinstalli Desktop Subsystem for Android™ (DSA) in uno di questi due modi:

- **Impostazioni** di Windows > **App** > **App installate**: trovi Desktop Subsystem for
  Android, poi selezioni **Disinstalla**.
- Esegua di nuovo il programma di installazione e selezioni **Uninstall**.

La disinstallazione rimuove questi elementi, per tutti gli utenti del PC, connessi o no:

- il programma, il servizio `DsaHost`, il comando `dsa`, DSA Settings e l'APK Installer;
- le immagini Android e gli aggiornamenti scaricati;
- le voci delle app Android nel menu Start e le loro voci in **App installate**;
- **Android** in Esplora file, l'associazione dei file `.apk` e le voci del menu contestuale;
- le impostazioni e i log di DSA.

**Conserva** il disco Android di ogni utente, `%LOCALAPPDATA%\DSA\android.vhdx`, così una
successiva installazione ritrova le app e i dati. Per rimuovere anche i dati, esegua
`dsa reset-data --yes` prima della disinstallazione, oppure elimini quel file dopo.

Se il reindirizzamento delle cartelle ha spostato il menu Start di un utente, le voci delle
app Android di quell'utente restano, perché il programma di disinstallazione cerca solo nella
posizione predefinita. Le elimini a mano.

La configurazione di WSL resta intatta, perché DSA non la modifica mai.

## Linux (non ancora rilasciato)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

La rimozione del pacchetto rimuove anche la voce del repository di pacchetti che aveva
aggiunto.

---

Android è un marchio di Google LLC. Windows è un marchio del gruppo di società Microsoft. DSA
non è affiliato a Google o Microsoft, né approvato o sponsorizzato da loro.
