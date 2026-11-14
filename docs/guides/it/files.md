# File

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · [简体中文](../zh-CN/files.md) · [繁體中文](../zh-TW/files.md) · [Español](../es/files.md) · [Français](../fr/files.md) · [Deutsch](../de/files.md) · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · **Italiano** · [Tiếng Việt](../vi/files.md) · [Bahasa Indonesia](../id/files.md) · [Türkçe](../tr/files.md)

Desktop Subsystem for Android™ (DSA) Le permette di aprire i file di Android in Esplora file,
di condividere le cartelle di Windows con le app Android e di inviare un file a un'app
Android. Questa pagina descrive DSA su Windows.

## Aprire i file di Android in Esplora file

Usi uno di questi modi. Ciascuno avvia Android, se non è in esecuzione.

- **Android** nel riquadro di spostamento di Esplora file, subito sotto **Linux** di WSL.
- **Android files** nella pagina **Advanced** (Avanzate) di DSA Settings.
- `dsa files open`. `dsa files` indica dove si trova lo spazio di archiviazione, senza aprirlo.

Vede lo spazio di archiviazione condiviso di Android: i file salvati dalle app Android, come
foto e download. Può copiare file in entrata e in uscita come in qualsiasi cartella. Esplora
file raggiunge lo spazio di archiviazione tramite il client WebDAV di Windows. Se lo spazio di
archiviazione non risponde, DSA mostra un errore con il motivo.

## Condividere le cartelle di Windows con le app Android

Le cartelle Documenti, Download, Immagini, Musica, Video e Desktop compaiono nello spazio di
archiviazione condiviso di Android sotto `Host`, come `Host/Pictures`. Le app Android possono
aprire e salvare file lì. Un'app ha prima bisogno dell'autorizzazione di archiviazione di
Android, e Windows le permette di fare solo ciò che può fare il Suo account.

La condivisione è attiva per impostazione predefinita. Per cambiarla, apra la pagina
**Advanced** di DSA Settings:

- Disattivi **Share Windows folders with Android** per non condividere alcuna cartella.
- In **Shared folders**, deselezioni una cartella per smettere di condividerla. Scompare
  subito da Android.

Dalla riga di comando:

```
dsa settings set shared-folders Documents,Pictures   # solo queste; anche all o none
dsa settings set file-sharing off                     # nessuna cartella condivisa
```

Le versioni precedenti usavano una cartella chiamata `Windows` invece di `Host`. Android
rimuove la vecchia cartella quando è vuota, quindi i file salvati lì restano.

## Inviare un file a un'app Android

- **Trascinamento.** Trascini un file da Esplora file sulla finestra di un'app. Il file va a
  quell'app. Un APK va invece all'APK Installer. Le cartelle non vengono inviate.
- **Menu contestuale.** Faccia clic con il pulsante destro su un file e selezioni **Open in an
  Android app** o **Share with an Android app**. In Windows 11 si trovano sotto **Mostra altre
  opzioni**. Android chiede poi quale app riceve il file.
- **Riga di comando.** `dsa open <file>` e `dsa share <file>`.

Questi metodi inviano file fino a 64 MiB. Un file più grande viene rifiutato prima che venga
inviato qualsiasi cosa. Per dare a un'app un file più grande, lo metta invece in una cartella
condivisa.

## Prendere un file da Android

- Lo salvi in Android sotto `Host/<folder>`. Si troverà quindi in quella cartella sul PC.
- Oppure apra **Android** in Esplora file e copi il file all'esterno.

---

Android è un marchio di Google LLC. Windows è un marchio del gruppo di società Microsoft. DSA
non è affiliato a Google o Microsoft, né approvato o sponsorizzato da loro.
