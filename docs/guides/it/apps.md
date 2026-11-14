# App e finestre

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · [简体中文](../zh-CN/apps.md) · [繁體中文](../zh-TW/apps.md) · [Español](../es/apps.md) · [Français](../fr/apps.md) · [Deutsch](../de/apps.md) · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · **Italiano** · [Tiếng Việt](../vi/apps.md) · [Bahasa Indonesia](../id/apps.md) · [Türkçe](../tr/apps.md)

Ogni attività di un'app Android si apre come una finestra del desktop a sé, con un proprio
pulsante sulla barra delle applicazioni. Non c'è una cornice da emulatore intorno a uno schermo
di telefono. Questa pagina descrive Desktop Subsystem for Android™ (DSA) su Windows.

## Aprire un'app

- **Menu Start.** Ogni app installata ha una voce nel menu Start con il proprio nome e la
  propria icona, quindi la ricerca di Windows la trova. Aprirla avvia prima Android, se non è
  in esecuzione.
- **DSA Settings.** Nella pagina **Apps** (App), selezioni **Open** accanto all'app.
- **Riga di comando.** `dsa apps list` elenca le app installate con i relativi pacchetti.
  `dsa launch <package>` ne apre una, per esempio `dsa launch com.android.settings`.

Quando la voce del menu Start di un altro programma ha già il nome dell'app, DSA aggiunge il
pacchetto al nome, come `Clock (com.example.clock)`. DSA modifica o rimuove solo le voci che ha
creato.

## Collegamenti sul desktop e jump list

Le icone delle app funzionano come quelle delle app Windows, non come quelle di una schermata
Home di Android.

- **Collegamento sul desktop.** Nella pagina **Apps** di DSA Settings, selezioni **Create
  desktop shortcut** accanto all'app. Può anche fare clic con il pulsante destro sul pulsante
  dell'app nella barra delle applicazioni e selezionare **Create desktop shortcut**. Dalla riga
  di comando: `dsa apps desktop-shortcut <package>`. Il collegamento apre l'app come la sua
  voce nel menu Start.
- **Scorciatoie dell'app.** Le scorciatoie offerte da un'app Android, come una nuova chat in
  un'app di messaggistica, compaiono quando fa clic con il pulsante destro sul pulsante
  dell'app nella barra delle applicazioni o sulla sua voce nel menu Start. Selezionarne una
  apre quella schermata dell'app in una finestra, avviando prima Android se non è in
  esecuzione. Quando l'app cambia le proprie scorciatoie, l'elenco si aggiorna. Dalla riga di
  comando: `dsa apps shortcut <package> <id>`; `dsa apps info <package> --json` elenca gli ID.
- **Scorciatoie richieste da un'app.** Quando un'app chiede di aggiungere una delle sue
  scorciatoie al desktop, DSA chiede prima a Lei e la aggiunge solo se è d'accordo.

Quando un'app cambia nome, per esempio dopo un aggiornamento, la sua voce nel menu Start, i
suoi collegamenti sul desktop e la sua aggiunta alla barra delle applicazioni seguono il nuovo
nome e continuano a funzionare. Un collegamento sul desktop che ha eliminato non viene
ricreato. La disinstallazione di un'app rimuove anche i suoi collegamenti sul desktop.

## Lavorare con le finestre delle app

- Ogni attività ha una propria finestra, un proprio pulsante sulla barra delle applicazioni e
  una propria voce in Alt+Tab, con il nome e l'icona dell'app. Gestione attività elenca ogni
  app con il suo nome.
- La barra del titolo fa parte dell'app: i colori dell'app arrivano fino al bordo superiore
  della finestra, e i pulsanti della barra del titolo di Windows 11 stanno sopra di essa.
  L'icona e il nome dell'app sono a sinistra. Quando l'app può tornare indietro, l'icona
  diventa una freccia indietro; la selezioni per tornare indietro.
- Funzionano i layout di ancoraggio (passando il puntatore sul pulsante di ingrandimento), il
  trascinamento della barra del titolo, il doppio clic su di essa per ingrandire e il
  ridimensionamento in tempo reale. Il ridimensionamento non riavvia l'app. Una finestra
  ingrandita riempie l'area di lavoro del monitor e lascia visibile la barra delle
  applicazioni.
- Non c'è una barra di navigazione Android. La barra inferiore di un'app sta sul bordo
  inferiore della finestra, senza una fascia vuota sotto.
- Mouse (con hover), tocco, penna (pressione, gomma e pulsante laterale) e tastiera
  funzionano tutti. Il testo arriva dal metodo di input di Windows, compresi il coreano e
  altre lingue; la composizione compare direttamente nell'editor Android, e la finestra dei
  candidati sta accanto al cursore. Non c'è un'app tastiera Android.
- Trascini file da Esplora file sulla finestra di un'app, oppure trascini file da un'app in
  Esplora file.
- Le utilità per la lettura dello schermo come Assistente vocale (Narrator) e NVDA leggono
  l'app tramite UI Automation.
- Chiudere una finestra chiede ad Android di terminare quell'attività.

## Il menu della finestra

Faccia clic con il pulsante destro sulla barra del titolo, selezioni l'icona dell'app oppure
prema Alt+Spazio. Dopo le voci consuete di Windows, il menu contiene:

- **Full screen**: riempie l'intero schermo, come F11. Lo selezioni di nuovo, o prema F11, per
  uscire.
- **New window**: apre un'altra finestra dell'app, per le app che ne supportano più di una.
- **Screenshot**: copia negli appunti la schermata dell'app, senza la barra del titolo, e la
  salva nella cartella Screenshot della cartella Immagini.
- **Aspect ratio**: apre l'impostazione di Android per le proporzioni dell'app, per le app che
  appaiono stirate in una finestra grande.
- **Open by default**: sceglie se i link supportati dall'app si aprono nell'app o nel browser.
- **App info**: apre la voce dell'app nella pagina **Apps** di DSA Settings.

## Schermo intero

Quando un'app chiede lo schermo intero, la sua finestra lo mostra in uno di due modi:

- **Sull'intero schermo**: la finestra riempie il monitor e la barra del titolo si nasconde.
  Tocchi con il puntatore il bordo superiore dello schermo per mostrarla, con riduci a icona e
  chiudi; il pulsante centrale esce dallo schermo intero.
- **All'interno della finestra**: la finestra mantiene le sue dimensioni e la barra del
  titolo, e l'app la riempie.

Le app lo chiedono in due tipi di modi, e ciascun tipo ha il proprio valore predefinito:

| Tipo di richiesta | Esempio | Predefinito |
|---|---|---|
| Una richiesta esplicita di schermo intero | Il pulsante schermo intero di un lettore video | Sull'intero schermo |
| Solo nascondere le barre | Un feed di video brevi | All'interno della finestra |

Cambi i valori predefiniti in **App full screen** e **Apps that hide their bars** nella pagina
**Display & GPU** (Schermo e GPU) di DSA Settings, oppure per una singola app in **Full
screen** e **Bars hidden** nella pagina **Apps**. La scelta dell'app stessa ha la precedenza.
F11 e **Full screen** nel menu della finestra attivano o disattivano in qualsiasi momento lo
schermo intero sull'intero schermo, qualunque sia l'impostazione.

Dalla riga di comando:

```
dsa settings set app-fullscreen screen|window           # predefinito per le richieste esplicite
dsa settings set app-immersive screen|window            # predefinito per le barre nascoste
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## Impostare le dimensioni della finestra di un'app

Nella pagina **Apps** di DSA Settings:

- Disattivi **Resizable** per mantenere le finestre dell'app alle dimensioni con cui si aprono.
- Scelga le dimensioni con cui si aprono le finestre dell'app: quelle scelte da Android,
  oppure dimensioni da telefono o da tablet, in verticale o in orizzontale.

La modifica si applica alla finestra successiva che l'app apre. Dalla riga di comando:

```
dsa app-window <package> --fixed-size on        # non può essere ridimensionata né ingrandita
dsa app-window <package> --start-size 1280x800  # si apre a queste dimensioni; auto lascia scegliere Android
dsa app-window <package> --dpi-scaling system   # come si scala su un altro monitor; vedere sotto
dsa app-window <package> reset                  # torna ai valori predefiniti
```

## Spostare un'app su un monitor con un'altra scala

Un'app si apre con la scala del monitor su cui si apre la sua finestra, fin dal primo
fotogramma, quindi non viene ricreata subito dopo l'apertura. Quando sposta una finestra su un
monitor con un'altra scala, l'app mantiene le sue dimensioni sullo schermo. Ogni app lo fa in
uno di questi modi, che sceglie in **Scaling on another monitor** nella pagina **Apps**, oppure
con `dsa app-window <package> --dpi-scaling`:

- **Automatic** (`auto`, il predefinito): **System** per le app presenti nell'elenco integrato
  di DSA delle app che si guastano quando la scala cambia mentre sono in esecuzione, e **App**
  per tutte le altre. L'elenco contiene YouTube: mantiene il layout per la scala con cui si è
  avviato, quindi dopo uno spostamento le schede in basso e il miniplayer resterebbero guasti
  fino al riavvio. Settings mostra il valore predefinito di un'app di questo tipo come
  **Scaling: system (recommended for this app)**. La Sua scelta per l'app ha la precedenza.
- **App** (`app`): mentre trascina la finestra, DSA la scala per il nuovo monitor. Quando la
  rilascia, l'app si ridisegna con la scala del nuovo monitor e il testo resta nitido: Android
  ricrea la schermata una volta e ne ripristina lo stato (posizione di scorrimento, testo
  digitato), quindi l'app si ridisegna brevemente dopo ogni spostamento, come fa un programma
  Windows con il ridimensionamento "Applicazione". Questo vale anche per un'app che dichiara di
  gestire da sola un cambio di scala, perché alcune di queste app ridisegnano con la nuova
  scala solo una parte della schermata.
- **System** (`system`): l'app mantiene la sua scala, e DSA scala la finestra per il nuovo
  monitor. L'app non viene mai ricreata. Quando la scala aumenta, il testo può apparire un
  po' sfocato.
- **System (forced)** (`system-forced`): all'app viene data la nuova scala senza riavvio, anche
  se non dichiara di gestirla. Non ancora disponibile: richiede una modifica ad Android che
  questa versione non contiene, quindi fino ad allora funziona come **System**.

Il testo nelle app segue le dimensioni del testo di Windows (Impostazioni > Accessibilità >
Dimensioni testo).

## Notifiche

Le notifiche Android compaiono come notifiche di Windows, con il nome e l'icona dell'app.

- Selezioni una notifica per aprire l'app.
- Se la chiude, viene chiusa anche in Android, a meno che l'app non la mantenga visibile.
- Le notifiche continuano ad arrivare mentre Android è in Doze. Vedere [Memoria](memory.md).

## App in esecuzione in background

Un'app che resta in esecuzione dopo la chiusura della sua finestra, come un lettore musicale,
compare come icona nell'area di notifica. La sua descrizione comando è la notifica dell'app.

- Selezioni l'icona per aprire l'app.
- Faccia clic con il pulsante destro per **Open**, le azioni proprie della notifica e **Stop**.
- **Stop** chiede prima conferma. Selezioni **Do not ask again** per saltare la domanda da quel
  momento in poi. Per ricevere di nuovo la domanda, attivi **Ask before stopping an app from
  the notification area** nella pagina **Running in the background** (In esecuzione in
  background) di DSA Settings.
- Un'app che riproduce contenuti multimediali funziona anche con i tasti multimediali e con i
  controlli multimediali di Windows.

La pagina **Running in the background** di DSA Settings elenca queste app, con **Open** e
**Stop** per ciascuna. Dalla riga di comando: `dsa background`, `dsa background open <package>`
e `dsa background stop <package>`.

## Widget sul desktop

Un widget Android può stare sul desktop in una propria finestra. Resta sotto le altre
finestre, resta visibile quando mostra il desktop, e non compare nella barra delle
applicazioni né in Alt+Tab.

1. Apra la pagina **Widgets** (Widget) di DSA Settings.
2. In **Widgets your apps offer**, selezioni **Add to desktop** accanto a un widget. Se l'app
   chiede di configurare il widget, la sua configurazione si apre in una finestra.
3. Per spostare un widget, lo prema in un punto qualsiasi e resti fermo per mezzo secondo. Il
   widget si solleva, con un bordo e una maniglia su ogni lato lungo il quale si può
   ridimensionare. Lo trascini per spostarlo, oppure trascini una maniglia per
   ridimensionarlo. Un trascinamento rapido resta al widget, quindi i suoi elenchi continuano a
   scorrere. Può anche trascinare un bordo del widget per ridimensionarlo senza sollevarlo
   prima; lo fanno solo i bordi lungo i quali si può ridimensionare. DSA ricorda dove lo ha
   messo e quanto è grande.
4. Faccia clic con il pulsante destro sul widget, oppure rilasci dopo la pressione prolungata,
   per **Widget settings** (quando il widget si può configurare di nuovo), **Resize**, per
   scegliere una delle dimensioni consentite, e **Remove**. Anche il tasto menu e Maiusc+F10 lo
   aprono. In questo menu funzionano i tasti freccia, Invio ed Esc, e una lettera sceglie una
   voce: R per **Resize** e M per **Remove**.

Dalla riga di comando: `dsa widgets list`, `dsa widgets add <provider>` e
`dsa widgets remove <id>`.

## Link e appunti

- Un link web toccato in un'app Android si apre nel browser predefinito. Un link che un'app
  installata ha verificato come proprio si apre in quell'app.
- Per aprire invece tutti i link web nelle app Android, attivi **Open web links in Android
  apps** nella pagina **Advanced** (Avanzate) di DSA Settings
  (`dsa settings set links-in-android on`).
- Il testo copiato è condiviso in entrambe le direzioni tra Windows e Android. Per
  interrompere la condivisione, disattivi **Share the clipboard with Android** nella pagina
  **Advanced** (`dsa settings set clipboard off`).

## Google Play e video protetti

DSA non include software Google. Se lo desidera, DSA lo scarica per Lei dalla sua fonte, dopo
averLe mostrato la fonte e le condizioni. Entrambi si applicano dal successivo avvio di
Android, ed entrambi si possono rimuovere di nuovo.

- **Google Play.** Nella pagina **Advanced** di DSA Settings, selezioni **Install** sotto
  **Google Play**, oppure esegua `dsa gapps install`. DSA scarica il pacchetto pubblico
  LiteGapps Core (Google Play services, il Play Store e la sincronizzazione dell'account) per
  la Sua versione di Android. Questo Android non è certificato da Google, quindi Google
  potrebbe rifiutare l'accesso finché non registra il dispositivo: **Register this device with
  Google** mostra l'ID Android e apre la pagina di registrazione. `dsa gapps status` mostra lo
  stato, e `dsa gapps remove` rimuove le app di Google, anche mentre Android è fermo.
- **Widevine.** Le app di streaming hanno bisogno del modulo Widevine di Google per riprodurre
  video protetti. Nella pagina **Advanced**, selezioni **Get** sotto **Protected content**,
  oppure esegua `dsa drm widevine install`. DSA scarica l'immagine dell'emulatore Android di
  Google per la Sua versione di Android (circa 2 GB), conserva solo il modulo Widevine, senza
  modifiche, ed elimina il resto. Qui Widevine ha il livello di sicurezza L3, quindi i servizi
  di streaming inviano al massimo la definizione standard; per l'alta definizione, usi un
  browser o un'app di Windows. `dsa drm widevine remove` lo rimuove.

## App create solo per ARM

Su un PC x64, un'app con codice solo per processori ARM funziona tramite un traduttore ARM.
Android su un PC x64 ne include uno, il traduttore open source Digitalis, e lo usa dal primo
avvio. Può usarne un altro al suo posto:

- **Il traduttore ARM di Google.** Nella pagina **Advanced**, selezioni **Get** accanto a
  **Google's ARM translator**, oppure esegua `dsa arm-translation fetch google`. DSA mostra
  prima da dove proviene e le condizioni di Google (la
  [Android SDK License](https://developer.android.com/studio/terms)), e non scarica nulla
  finché non le accetta. Il PC scarica quindi l'immagine dell'emulatore Android che Google
  offre per la Sua versione di Android (circa 2 GB), conserva solo il traduttore ed elimina il
  resto. DSA non include né ridistribuisce il traduttore di Google.
- **Un pacchetto che compila o che ottiene altrove**, come uno creato a partire dal traduttore
  open source Digitalis: `dsa arm-translation install <folder>`.

Poi riavvii Android: lo fermi nella pagina **General** (Generale) di DSA Settings, poi apra
un'app.

Per tornare al traduttore incluso in Android, selezioni **Use** accanto a **Use the default
translator** nella pagina **Advanced**, oppure esegua `dsa arm-translation use-default`. In
questo modo viene rimosso il pacchetto che aveva installato, dal successivo avvio di Android.

`dsa arm-translation status` indica quale traduttore usa Android. L'interruttore **Run apps
built only for ARM** nella pagina **Advanced** attiva o disattiva la traduzione senza rimuovere
nulla. Per un'app che ha codice sia ARM sia x64, **Run as ARM** nella pagina **Apps**
(`dsa apps run-as-arm <package> on`) esegue invece il suo codice ARM. Possono funzionare solo
le app ARM a 64 bit (`arm64-v8a`). Su un PC ARM64, Android esegue le app ARM direttamente e non
ha bisogno di un traduttore.

## Accesso root

Alcune app, come gli strumenti di backup, hanno bisogno di root. DSA lo concede come fa `sudo`
su un PC Linux: solo quando un amministratore del PC approva, una richiesta alla volta. È
disattivato per impostazione predefinita, e le app non autorizzate non vedono affatto `su`.

1. Un amministratore attiva **Let Android apps ask for root** sotto **Root access** nella
   pagina **Advanced** di DSA Settings (`dsa settings set root-elevation on`). Si applica dal
   successivo avvio di Android.
2. Nella pagina **Apps**, imposti per l'app **Root access** (accesso root):
   - **Root: not allowed** (il predefinito): l'app non ha `su`.
   - **Root: ask each time**: ogni volta che l'app usa `su`, Windows mostra la sua richiesta di
     amministratore (UAC) con il nome e l'icona dell'app. Un utente standard inserisce la
     password di un amministratore. Un'approvazione dura 15 minuti per quell'app.
   - **Root: remember (no asking)**: l'app usa `su` senza chiedere. Questa scelta richiede
     l'approvazione di un amministratore.

   Dalla riga di comando: `dsa apps root <package> off|ask|remember`.

Con la modalità sviluppatore attiva, anche `adb shell` può usare `su`. Ha una propria riga
nella pagina **Apps**, **adb shell**, e per impostazione predefinita chiede ogni volta
(`dsa apps root adb ask`). La sua richiesta si chiama **adb shell** e mostra il comando.

Quando a chiedere è una parte di Android stesso, la richiesta la nomina: **Android system**,
**Android phone services**, **Android Bluetooth**, **Android NFC** o **Android system
service**. Le app che condividono un unico ID utente vengono approvate come un unico gruppo, e
la richiesta le elenca tutte.

Ogni richiesta viene conservata nel log del servizio con l'app, l'ora, il comando e l'esito
dell'approvazione. Disattivare la funzione nasconde `su` a tutte le app in una volta, senza
riavvio.

---

Android è un marchio di Google LLC. Windows è un marchio del gruppo di società Microsoft. DSA
non è affiliato a Google o Microsoft, né approvato o sponsorizzato da loro.
