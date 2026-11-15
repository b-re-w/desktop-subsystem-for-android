# Installare APK

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · [简体中文](../zh-CN/install-apks.md) · [繁體中文](../zh-TW/install-apks.md) · [Español](../es/install-apks.md) · [Français](../fr/install-apks.md) · [Deutsch](../de/install-apks.md) · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · **Italiano** · [Tiếng Việt](../vi/install-apks.md) · [Bahasa Indonesia](../id/install-apks.md) · [Türkçe](../tr/install-apks.md)

Desktop Subsystem for Android™ (DSA) non include uno store di app. Le app si installano da file
APK, con l'APK Installer o con il comando `dsa`. Questa pagina descrive DSA su Windows.

## Installare con l'APK Installer

1. Faccia doppio clic su un file `.apk` in Esplora file. Si apre nell'**APK Installer**.
2. Controlli di che APK si tratta. La finestra lo mostra senza avviare Android:
   - il nome, l'icona, il pacchetto e la versione dell'app;
   - l'editore, ricavato dal certificato di firma, perché un APK non indica alcun editore;
   - la provenienza: l'host da cui il file è stato scaricato, oppure la sua cartella;
   - sotto **Capabilities**, le prime autorizzazioni che richiede. **More** le mostra tutte,
     insieme alle versioni di Android per cui è fatto, alle dimensioni, ai tipi di processore e
     all'SHA-256 del certificato.

   Quando l'APK contiene codice solo per un altro tipo di processore, la finestra lo segnala.
3. Selezioni **Install**. Quando l'app è già installata, il pulsante riporta **Update** o
   **Install again**. Android si avvia prima, se non è in esecuzione.
4. Con **Launch when ready** selezionato (il predefinito), l'app si apre appena è installata e
   il programma di installazione si chiude. Altrimenti selezioni **Launch**, oppure apra l'app
   più tardi dal menu Start.

Mentre Android è in esecuzione, la finestra indica anche se il file è una nuova installazione,
un aggiornamento, la stessa versione di nuovo, oppure qualcosa che Android rifiuta: una
versione precedente, o la stessa app firmata con un'altra chiave.

Può anche trascinare file APK sull'APK Installer, su DSA Settings o sulla finestra di un'app.
Ogni APK si apre in una propria finestra del programma di installazione.

## Installare dalla riga di comando

```
dsa apk-info app.apk            # di che APK si tratta; legge il file senza avviare Android
dsa install app.apk             # lo installa; avvia Android se non è in esecuzione
dsa install app.apk --progress  # stampa anche ogni passaggio
```

Quando Android rifiuta l'APK, `dsa install` termina con il codice 1 e il suo messaggio inizia
con `not installed (<reason>):`.

## Perché un'installazione viene rifiutata

| Motivo | Significato | Che cosa fare |
|---|---|---|
| `incompatible` | L'app richiede un altro tipo di processore, una versione più recente di Android o una funzionalità che questo dispositivo non ha | Vedere [App create solo per ARM](apps.md#app-create-solo-per-arm) o [Versioni di Android](android-versions.md) |
| `conflicting-signature` | L'app è già installata, firmata con un'altra chiave | Disinstalli prima l'app installata |
| `downgrade` | È installata una versione più recente dell'app | Disinstalli prima l'app installata |
| `invalid-apk` | Il file non è un APK firmato che Android possa leggere | Si procuri di nuovo l'APK |
| `insufficient-storage` | Android non ha spazio per l'app | Liberi spazio in Android |
| `blocked` | Un criterio di questo Android non consente l'app | |
| `other` | Qualsiasi altro caso; segue il messaggio di Android | |

Un APK può arrivare fino a 2 GiB. Le installazioni avvengono una alla volta: una seconda
attende la prima.

## Disinstallare un'app

Usi uno di questi modi:

- **Impostazioni** di Windows > **App** > **App installate**.
- La pagina **Apps** di DSA Settings: **Uninstall** accanto all'app.
- `dsa apps uninstall <package>`.

Le app che fanno parte di Android non si possono disinstallare.

---

Android è un marchio di Google LLC. Windows è un marchio del gruppo di società Microsoft. DSA
non è affiliato a Google o Microsoft, né approvato o sponsorizzato da loro.
