# Versioni di Android

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · [简体中文](../zh-CN/android-versions.md) · [繁體中文](../zh-TW/android-versions.md) · [Español](../es/android-versions.md) · [Français](../fr/android-versions.md) · [Deutsch](../de/android-versions.md) · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · **Italiano** · [Tiếng Việt](../vi/android-versions.md) · [Bahasa Indonesia](../id/android-versions.md) · [Türkçe](../tr/android-versions.md)

Desktop Subsystem for Android™ (DSA) supporta le due versioni principali più recenti di
Android: oggi Android 16 e Android 17. Per impostazione predefinita esegue la più recente.
Alcune app funzionano su una sola delle due, quindi può scegliere.

## Vedere quale versione è in esecuzione

Apra la pagina **Updates** (Aggiornamenti) di DSA Settings. Sotto **Android** mostra la
versione, la build, il livello della patch di sicurezza, la versione che segue e lo stato del
suo aggiornamento, con l'avanzamento del download. **Check now** chiede al servizio di cercare
subito un aggiornamento. Oppure esegua:

```
dsa android version   # la versione in esecuzione e quelle che si possono scegliere
```

## Cambiare la versione

Tutti gli utenti del PC eseguono la stessa versione di Android, quindi solo un amministratore
può cambiarla.

1. Apra la pagina **Updates** di DSA Settings.
2. In **Version to use**, scelga una versione, oppure **Always the newest**.
3. Selezioni **Change** e approvi la richiesta di amministratore.

La versione scelta viene scaricata e usata dal successivo avvio di Android. Da un terminale da
amministratore:

```
dsa android use 16
dsa android use latest
```

### Passare a una versione precedente cancella i dati

Android non può leggere i dati scritti da una versione più recente. Passare a una versione
principale precedente cancella quindi i dati Android di tutti gli utenti del PC: le app che
hanno installato, i loro account e tutti i dati delle app. Le impostazioni di DSA restano.
L'operazione non si può annullare.

DSA Settings Le chiede di confermare con **Erase data and change**. Dalla riga di comando,
aggiunga `--erase-data`:

```
dsa android use 16 --erase-data
```

Passare a una versione più recente mantiene le app e i dati.

## Aggiornamenti

- La versione scelta continua a ricevere gli aggiornamenti di sicurezza.
- Quando una versione esce dal supporto, DSA passa alla successiva e mantiene le app.
- DSA non passa mai da solo a una versione principale precedente.
- Ogni immagine viene verificata rispetto a un elenco firmato prima di essere usata.
  Un'immagine che non si avvia viene sostituita dalla precedente.

Un amministratore può disattivare gli aggiornamenti dell'immagine con **Update the Android
image automatically** nella pagina **Updates** di DSA Settings, oppure con
`dsa settings set image-updates off`.

---

Android è un marchio di Google LLC. Windows è un marchio del gruppo di società Microsoft. DSA
non è affiliato a Google o Microsoft, né approvato o sponsorizzato da loro.
