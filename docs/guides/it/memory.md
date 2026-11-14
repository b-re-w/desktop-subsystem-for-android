# Memoria e comportamento in inattività

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · [简体中文](../zh-CN/memory.md) · [繁體中文](../zh-TW/memory.md) · [Español](../es/memory.md) · [Français](../fr/memory.md) · [Deutsch](../de/memory.md) · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · **Italiano** · [Tiếng Việt](../vi/memory.md) · [Bahasa Indonesia](../id/memory.md) · [Türkçe](../tr/memory.md)

Desktop Subsystem for Android™ (DSA) restituisce memoria al PC mentre Android è in esecuzione,
non solo quando si ferma. Non c'è nulla da configurare per questo. Questa pagina spiega che
cosa succede e come vederlo.

## Vedere quanta memoria usa Android

Apra la pagina **Memory** (Memoria) di DSA Settings, oppure esegua `dsa memory`. Entrambi
mostrano:

- **On this PC**: quanta memoria occupano la macchina virtuale di Android, il window manager e
  ogni finestra di app.
- **Inside Android**: memoria in uso, cache, memoria compressa e i processi più grandi.
  Android comunica questi dati qualche minuto dopo l'avvio.

## Doze, lo stato di inattività

Quando tutte le finestre Android sono chiuse o ridotte a icona da 30 secondi, lo schermo di
Android si spegne e Android entra in Doze, come fa un telefono in tasca.

- Notifiche, sveglie e widget continuano a funzionare in Doze. Nel frattempo la CPU riposa.
- `dsa status` indica `Dozing`. La pagina **General** (Generale) di DSA Settings indica
  **Dozing (idle)**.
- Mostrare una qualsiasi finestra Android riattiva subito Android.

Android segue anche la sospensione del PC. Salva il proprio stato prima che il PC vada in
sospensione e regola l'orologio quando il PC si riattiva.

## Congelamento (disattivato per impostazione predefinita)

Il congelamento ferma tutti i processi in Android. In quel momento non usa la CPU, ma si
fermano anche notifiche, sveglie e widget. Per questo DSA non congela mai Android a meno che
non lo chieda Lei.

- Per congelare subito: **Freeze** sotto **Freeze Android now** nella pagina **General**,
  oppure `dsa freeze`.
- Per congelare dopo 30 minuti in Doze: attivi **Freeze Android when idle** nella pagina
  **Advanced** (Avanzate), oppure `dsa settings set freeze on`.
- Aprire una finestra Android, o un input in una di esse, riprende Android.

## Come torna la memoria

- La macchina virtuale di Android prende memoria dal PC solo quando Android la usa per la
  prima volta.
- La memoria che Android libera viene restituita al PC mentre Android è in esecuzione.
- Quando l'ultima finestra di un'app si chiude, Android termina i processi di quell'app, così
  la loro memoria torna indietro in pochi secondi. Un'app con un widget sul desktop, o una che
  sta ancora lavorando in background (come un lettore musicale), resta in esecuzione.
- Un'app che mantiene in esecuzione un servizio in background senza finestra, notifica, suono
  o widget viene fermata dopo cinque minuti. Se Android la riavvia più e più volte, DSA attende
  ogni volta più a lungo, fino a un'ora.
- Mentre nessuna finestra di app è aperta, Android scarta le proprie cache dei file e sposta
  la memoria inattiva nella memoria compressa, così ne può tornare indietro di più.
- Il window manager restituisce la memoria che non usa 30 secondi dopo la chiusura
  dell'ultima finestra di app.

## Limite di memoria

Il limite è **Automatic** per impostazione predefinita, cioè senza un limite fisso. Android
parte da 2 GB e cresce durante l'esecuzione, fino alla memoria del PC meno quella che Windows
conserva (il 10% della memoria o 2 GB, il valore maggiore). Android occupa solo ciò che usa.
Per impostare un limite, usi **Memory limit** nella pagina **Memory**, oppure:

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # torna ad automatico
```

Un nuovo limite si applica al successivo avvio di Android.

---

Android è un marchio di Google LLC. Windows è un marchio del gruppo di società Microsoft. DSA
non è affiliato a Google o Microsoft, né approvato o sponsorizzato da loro.
