# Aplicaciones y ventanas

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · [简体中文](../zh-CN/apps.md) · [繁體中文](../zh-TW/apps.md) · **Español** · [Français](../fr/apps.md) · [Deutsch](../de/apps.md) · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · [Tiếng Việt](../vi/apps.md) · [Bahasa Indonesia](../id/apps.md) · [Türkçe](../tr/apps.md)

Cada tarea de una aplicación de Android se abre como una ventana de escritorio propia, con su
propio botón en la barra de tareas. No hay ningún marco de emulador alrededor de una pantalla
de teléfono. Esta página describe Desktop Subsystem for Android™ (DSA) en Windows.

## Abrir una aplicación

- **Menú Inicio.** Cada aplicación instalada tiene una entrada en el menú Inicio con su
  propio nombre e icono, así que la búsqueda de Windows la encuentra. Al abrirla, primero se
  inicia Android si no se está ejecutando.
- **DSA Settings.** En la página **Apps** (aplicaciones), seleccione **Open** (abrir) junto a
  la aplicación.
- **Línea de comandos.** `dsa apps list` muestra las aplicaciones instaladas con sus
  paquetes. `dsa launch <package>` abre una, por ejemplo `dsa launch com.android.settings`.

Cuando la entrada del menú Inicio de otro programa ya tiene el nombre de la aplicación, DSA
añade el paquete al nombre, como `Clock (com.example.clock)`. DSA solo cambia o quita las
entradas que creó.

## Accesos directos del escritorio y listas de accesos directos

Los iconos de las aplicaciones funcionan como los de las aplicaciones de Windows, no como en
la pantalla de inicio de Android.

- **Acceso directo en el escritorio.** En la página **Apps** de DSA Settings, seleccione
  **Create desktop shortcut** (crear acceso directo en el escritorio) junto a la aplicación.
  También puede hacer clic con el botón derecho en el botón de la aplicación en la barra de
  tareas y seleccionar **Create desktop shortcut**. Desde la línea de comandos:
  `dsa apps desktop-shortcut <package>`. El acceso directo abre la aplicación igual que su
  entrada del menú Inicio.
- **Accesos directos de la aplicación.** Los accesos directos que ofrece una aplicación de
  Android, como el chat nuevo de una aplicación de mensajería, aparecen al hacer clic con el
  botón derecho en el botón de la aplicación en la barra de tareas o en su entrada del menú
  Inicio. Al seleccionar uno se abre esa pantalla de la aplicación en una ventana, e Android
  se inicia antes si no se está ejecutando. Cuando la aplicación cambia sus accesos
  directos, la lista también cambia. Desde la línea de comandos:
  `dsa apps shortcut <package> <id>`; `dsa apps info <package> --json` muestra los ID.
- **Accesos directos que pide una aplicación.** Cuando una aplicación pide añadir uno de sus
  accesos directos a su escritorio, DSA se lo pregunta antes y solo lo añade si usted está
  de acuerdo.

Cuando una aplicación cambia de nombre, por ejemplo después de una actualización, su entrada
del menú Inicio, sus accesos directos del escritorio y su anclaje en la barra de tareas
adoptan el nuevo nombre y siguen funcionando. Un acceso directo del escritorio que usted
eliminó no se vuelve a crear. Al desinstalar una aplicación también se quitan sus accesos
directos del escritorio.

## Trabajar con las ventanas de las aplicaciones

- Cada tarea tiene su propia ventana, su botón en la barra de tareas y su entrada en
  Alt+Tab, con el nombre y el icono de la aplicación. El Administrador de tareas muestra
  cada aplicación con su propio nombre.
- La barra de título forma parte de la aplicación: los colores de la aplicación llegan hasta
  el borde superior de la ventana, y los botones de título de Windows 11 se sitúan encima.
  El icono y el nombre de la aplicación aparecen a la izquierda. Cuando la aplicación puede
  volver atrás, el icono se convierte en una flecha de retroceso; selecciónela para volver.
- Los diseños de ajuste (al pasar el puntero sobre el botón de maximizar), el arrastre de la
  barra de título, el doble clic en ella para maximizar y el cambio de tamaño en vivo
  funcionan. Cambiar el tamaño no reinicia la aplicación. Una ventana maximizada ocupa el
  área de trabajo del monitor y deja visible la barra de tareas.
- No hay barra de navegación de Android. La barra inferior de una aplicación se sitúa en el
  borde inferior de la ventana, sin ninguna franja vacía debajo.
- Funcionan el ratón (con desplazamiento del puntero por encima), la entrada táctil, el lápiz
  (presión, borrador y botón lateral) y el teclado. El texto procede de su método de entrada
  de Windows, incluidos el coreano y otros idiomas; la composición se muestra directamente
  en el editor de Android, y la ventana de candidatos aparece junto al cursor. No hay ninguna
  aplicación de teclado de Android.
- Arrastre archivos desde el Explorador de archivos a la ventana de una aplicación, o
  arrastre archivos desde una aplicación al Explorador de archivos.
- Los lectores de pantalla como Narrador (Narrator) y NVDA leen la aplicación a través de UI
  Automation (automatización de la interfaz de usuario).
- Al cerrar una ventana se pide a Android que finalice esa tarea.

## El menú de la ventana

Haga clic con el botón derecho en la barra de título, seleccione el icono de la aplicación o
pulse Alt+Espacio. Después de los elementos habituales de Windows, el menú contiene:

- **Full screen** (pantalla completa): ocupa toda la pantalla, como F11. Selecciónelo de
  nuevo, o pulse F11, para salir.
- **New window** (ventana nueva): abre otra ventana de la aplicación, en las aplicaciones que
  admiten varias.
- **Screenshot** (captura de pantalla): copia la pantalla de la aplicación, sin la barra de
  título, en el portapapeles y la guarda en la carpeta Capturas de pantalla de su carpeta
  Imágenes.
- **Aspect ratio** (relación de aspecto): abre la opción de Android para la relación de
  aspecto de la aplicación, para las aplicaciones que se ven estiradas en una ventana grande.
- **Open by default** (abrir de forma predeterminada): elige si los enlaces que admite la
  aplicación se abren en la aplicación o en su navegador.
- **App info** (información de la aplicación): abre la entrada de la aplicación en la página
  **Apps** de DSA Settings.

## Pantalla completa

Cuando una aplicación pide la pantalla completa, su ventana la muestra de una de estas dos
formas:

- **En toda la pantalla**: la ventana ocupa el monitor y la barra de título se oculta. Toque
  el borde superior de la pantalla con el puntero para mostrarla, con minimizar y cerrar; el
  botón central sale de la pantalla completa.
- **Dentro de la ventana**: la ventana conserva su tamaño y su barra de título, y la
  aplicación la llena.

Las aplicaciones la piden de dos maneras, y cada una tiene su propio valor predeterminado:

| Tipo de petición | Ejemplo | Valor predeterminado |
|---|---|---|
| Una petición clara de pantalla completa | El botón de pantalla completa de un reproductor de vídeo | En toda la pantalla |
| Solo ocultar las barras | Un feed de vídeos cortos | Dentro de la ventana |

Cambie los valores predeterminados en **App full screen** y **Apps that hide their bars** de
la página **Display & GPU** (pantalla y GPU) de DSA Settings, o para una sola aplicación en
**Full screen request** y **When only the bars hide** de la página **Apps**. La elección propia de una aplicación
tiene prioridad. F11 y **Full screen** en el menú de la ventana alternan en cualquier momento
la pantalla completa en toda la pantalla, sea cual sea la configuración.

Desde la línea de comandos:

```
dsa settings set app-fullscreen screen|window           # predeterminado para peticiones claras
dsa settings set app-immersive screen|window            # predeterminado para barras ocultas
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## Definir el tamaño de la ventana de una aplicación

En la página **Apps** de DSA Settings:

- Desactive **Resizable** (redimensionable) para que las ventanas de la aplicación conserven
  el tamaño con el que se abren.
- Elija el tamaño con el que se abren las ventanas de la aplicación: el que elige Android, o
  un tamaño de teléfono o de tableta en vertical u horizontal.

El cambio se aplica a la siguiente ventana que abra la aplicación. Desde la línea de
comandos:

```
dsa app-window <package> --fixed-size on        # no se puede redimensionar ni maximizar
dsa app-window <package> --start-size 1280x800  # se abre con este tamaño; auto deja elegir a Android
dsa app-window <package> --dpi-scaling system   # cómo se escala en otro monitor; vea más abajo
dsa app-window <package> reset                  # vuelve a los valores predeterminados
```

## Hacer que una aplicación empiece debajo de la barra de título

Una aplicación dibuja hasta el borde superior de su ventana, bajo la barra de título. Algunas
aplicaciones aplastan entonces su barra superior u ocultan parte de ella bajo la barra de
título. Para una aplicación así, active **Start below the title bar** (empezar debajo de la barra de título) en la página
**Apps** de DSA Settings. El contenido de la aplicación empieza entonces debajo de la barra de
título, y la barra de título toma el color que la aplicación muestra ahí. Está desactivado de
forma predeterminada y se aplica la próxima vez que se abra la ventana de la aplicación. Desde
la línea de comandos:

```
dsa app-window <package> --below-title-bar on   # off vuelve al valor predeterminado
```

## Mover una aplicación a un monitor con otra escala

Android se inicia con la escala de su monitor principal, así que las aplicaciones se abren
allí con el tamaño correcto. Cuando mueve una
ventana a un monitor con otra escala, la aplicación conserva su tamaño en la pantalla. Cada
aplicación lo hace de una de estas formas, que usted elige en **Scaling**
(escala en otro monitor) de la página **Apps**, o con
`dsa app-window <package> --dpi-scaling`:

- **Automatic** (`auto`, el valor predeterminado): **System** para las aplicaciones de la
  lista integrada de DSA de aplicaciones que fallan cuando su escala cambia mientras se
  ejecutan, y **App** para todas las demás. La lista contiene YouTube: mantiene el diseño de
  la escala con la que se inició, así que después de un movimiento sus pestañas inferiores y
  su minirreproductor quedarían mal hasta que se reinicie. La configuración muestra el valor
  predeterminado de una aplicación así como **Automatic (system, recommended for this app)**.
  Su propia elección para la aplicación tiene prioridad.
- **App** (`app`): mientras arrastra la ventana, DSA la escala al nuevo monitor. Cuando la
  suelta, la aplicación se vuelve a dibujar con la escala del nuevo monitor y su texto sigue
  nítido: Android vuelve a crear su pantalla una vez y restaura su estado (posición de
  desplazamiento, texto escrito), así que se vuelve a dibujar brevemente después de cada
  movimiento, como hace un programa de Windows con la escala "Aplicación" ("Application").
  Esto se aplica incluso a una aplicación que dice gestionar por sí misma un cambio de
  escala, porque algunas de esas aplicaciones solo vuelven a dibujar parte de su pantalla
  con la nueva escala.
- **System** (`system`): la aplicación conserva su escala, y DSA escala su ventana al nuevo
  monitor. La aplicación nunca se vuelve a crear. Al ampliarse, el texto puede verse un poco
  borroso.
- **System (forced)** (`system-forced`): la aplicación recibe la nueva escala sin
  reiniciarse, aunque no diga que la gestiona. Aún no está disponible: necesita un cambio en
  Android que esta versión no tiene, así que hasta entonces funciona como **System**.

El texto de las aplicaciones sigue el tamaño del texto de Windows (Configuración >
Accesibilidad > Tamaño del texto).

## Notificaciones

Las notificaciones de Android aparecen como notificaciones de Windows, con el nombre y el
icono de la aplicación.

- Seleccione una notificación para abrir la aplicación.
- Descártela y también se descartará en Android, salvo que la aplicación la mantenga
  visible.
- Las notificaciones siguen llegando mientras Android está en Doze. Consulte
  [Memoria](memory.md).

## Aplicaciones que se ejecutan en segundo plano

Una aplicación que sigue ejecutándose después de cerrar su ventana, como un reproductor de
música, aparece como un icono en el área de notificación. Su información sobre herramientas
es la notificación de la aplicación.

- Seleccione el icono para abrir la aplicación.
- Haga clic con el botón derecho en él para ver **Open**, las acciones propias de la
  notificación y **Stop** (detener).
- **Stop** pregunta antes. Marque **Do not ask again** (no volver a preguntar) para omitir
  la pregunta a partir de entonces. Para que se vuelva a preguntar, active **Ask before
  stopping an app from the notification area** en la página **Running in the background**
  (en segundo plano) de DSA Settings.
- Una aplicación que reproduce contenido multimedia también funciona con las teclas
  multimedia y los controles multimedia de Windows.

La página **Running in the background** de DSA Settings muestra estas aplicaciones, con
**Open** y **Stop** para cada una. Desde la línea de comandos: `dsa background`,
`dsa background open <package>` y `dsa background stop <package>`.

## Widgets en el escritorio

Un widget de Android puede estar en su escritorio, en una ventana propia. Se queda debajo de
las demás ventanas, permanece cuando muestra el escritorio y no aparece en la barra de tareas
ni en Alt+Tab.

1. Abra la página **Widgets** de DSA Settings.
2. En **Widgets your apps offer** (widgets que ofrecen sus aplicaciones), seleccione **Add
   to desktop** (añadir al escritorio) junto a un widget. Si la aplicación le pide que
   configure el widget, su configuración se abre en una ventana.
3. Para mover un widget, púlselo en cualquier punto y manténgalo quieto medio segundo. Se
   levanta, con un borde y un tirador en cada lado por el que se puede redimensionar.
   Arrástrelo para moverlo, o arrastre un tirador para cambiar su tamaño. Un arrastre rápido
   lo sigue recibiendo el propio widget, así que sus listas se siguen desplazando. También
   puede arrastrar un borde del widget para cambiar su tamaño sin levantarlo antes; solo lo
   permiten los bordes por los que se puede redimensionar. DSA recuerda dónde lo colocó y su
   tamaño.
4. Haga clic con el botón derecho en el widget, o suéltelo después de la pulsación larga,
   para ver **Widget settings** (cuando el widget se puede volver a configurar), **Resize**,
   para elegir uno de los tamaños que permite, y **Remove** (quitar). La tecla de menú y
   Mayús+F10 también lo abren. Las teclas de flecha, Entrar y Esc funcionan en este menú, y
   una letra elige un elemento: R para **Resize** y M para **Remove**.

Desde la línea de comandos: `dsa widgets list`, `dsa widgets add <provider>` y
`dsa widgets remove <id>`.

## Enlaces y portapapeles

- Un enlace web que toque en una aplicación de Android se abre en su navegador
  predeterminado. Un enlace que una aplicación instalada ha verificado como suyo se abre en
  esa aplicación.
- Para abrir en cambio todos los enlaces web en aplicaciones de Android, active **Open web
  links in Android apps** en la página **Advanced** (opciones avanzadas) de DSA Settings
  (`dsa settings set links-in-android on`).
- El texto que copia se comparte en ambos sentidos entre Windows y Android. Para impedirlo,
  desactive **Share the clipboard with Android** en la página **Advanced**
  (`dsa settings set clipboard off`).

## Google Play y vídeo protegido

DSA no incluye software de Google. Si lo quiere, DSA lo descarga de su origen por usted,
después de mostrarle el origen y las condiciones. Ambos se aplican a partir del siguiente
inicio de Android, y ambos se pueden volver a quitar.

- **Google Play.** En la página **Advanced** de DSA Settings, seleccione **Install** en
  **Google Play**, o ejecute `dsa gapps install`. DSA descarga el paquete público LiteGapps
  Core (servicios de Google Play, Play Store y sincronización de cuentas) para su versión de
  Android. Este Android no está certificado por Google, así que Google puede negarse a
  iniciar su sesión hasta que registre el dispositivo: **Register this device with Google**
  muestra el ID de Android y abre la página de registro. `dsa gapps status` muestra el
  estado, y `dsa gapps remove` quita las aplicaciones de Google, incluso mientras Android
  está detenido.
- **Widevine.** Las aplicaciones de streaming necesitan el módulo Widevine de Google para
  reproducir vídeo protegido. En la página **Advanced**, seleccione **Get** (obtener) en
  **Protected content** (contenido protegido), o ejecute `dsa drm widevine install`. DSA
  descarga la imagen del emulador de Android de Google para su versión de Android (unos
  2 GB), conserva solo el módulo Widevine, sin modificar, y elimina el resto. Aquí Widevine
  tiene el nivel de seguridad L3, así que los servicios de streaming envían como máximo
  definición estándar; para la alta definición, use un navegador o una aplicación de
  Windows. `dsa drm widevine remove` lo quita.

## Aplicaciones compiladas solo para ARM

En un equipo x64, una aplicación con código solo para procesadores ARM se ejecuta a través
de un traductor ARM. Android en un equipo x64 incluye uno, el traductor de código abierto
Digitalis, y lo usa desde su primer inicio. Puede usar otro en su lugar:

- **El traductor ARM de Google.** En la página **Advanced**, seleccione **Get** junto a
  **Google's ARM translator**, o ejecute `dsa arm-translation fetch google`. DSA muestra
  primero de dónde procede y las condiciones de Google (la
  [Android SDK License](https://developer.android.com/studio/terms)), y no descarga nada
  hasta que usted las acepta. Después, su equipo descarga la imagen del emulador de Android
  que Google ofrece para su versión de Android (unos 2 GB), conserva solo el traductor y
  elimina el resto. DSA no incluye ni distribuye el traductor de Google.
- **Un paquete que usted compila u obtiene en otro lugar**, como uno creado a partir del
  traductor de código abierto Digitalis: `dsa arm-translation install <folder>`.

Después, reinicie Android: deténgalo en la página **General** de DSA Settings y luego abra
una aplicación.

Para volver al traductor que incluye Android, seleccione **Use** junto a **Use the default
translator** en la página **Advanced**, o ejecute `dsa arm-translation use-default`. Esto
quita el paquete que instaló, a partir del siguiente inicio de Android.

`dsa arm-translation status` indica qué traductor usa Android. El interruptor **Run apps
built only for ARM** de la página **Advanced** activa o desactiva la traducción sin quitar
nada. Para una aplicación que tiene código ARM y x64, **Run as ARM** en la página **Apps**
(`dsa apps run-as-arm <package> on`) ejecuta en su lugar su código ARM. Solo se pueden
ejecutar aplicaciones ARM de 64 bits (`arm64-v8a`). En un equipo ARM64, Android ejecuta las
aplicaciones ARM por sí mismo y no necesita traductor.

## Acceso root

Algunas aplicaciones, como las herramientas de copia de seguridad, necesitan root. DSA lo
concede como lo hace `sudo` en un equipo Linux: solo cuando un administrador del equipo lo
aprueba, una solicitud cada vez. Está desactivado de forma predeterminada, y las
aplicaciones que no tienen permiso no pueden ver `su` en absoluto.

1. Un administrador activa **Let Android apps ask for root** en **Root access** (acceso
   root) de la página **Advanced** de DSA Settings (`dsa settings set root-elevation on`).
   Se aplica a partir del siguiente inicio de Android.
2. En la página **Apps**, defina el **Root access** de la aplicación:
   - **Root: not allowed** (no permitido; el valor predeterminado): la aplicación no tiene
     `su`.
   - **Root: ask each time** (preguntar cada vez): cada vez que la aplicación usa `su`,
     Windows muestra su aviso de administrador (UAC) con el nombre y el icono de la propia
     aplicación. Un usuario estándar escribe la contraseña de un administrador. Una
     aprobación dura 15 minutos para esa aplicación.
   - **Root: remember (no asking)** (recordar, sin preguntar): la aplicación usa `su` sin
     preguntar. Elegir esta opción requiere la aprobación de un administrador.

   Desde la línea de comandos: `dsa apps root <package> off|ask|remember`.

Con el modo de desarrollador activado, `adb shell` también puede usar `su`. Tiene su propia
fila en la página **Apps**, **adb shell**, y pregunta cada vez de forma predeterminada
(`dsa apps root adb ask`). Su aviso se llama **adb shell** y muestra el comando.

Cuando lo pide una parte del propio Android, el aviso la nombra: **Android system**,
**Android phone services**, **Android Bluetooth**, **Android NFC** o **Android system
service**. Las aplicaciones que comparten un mismo ID de usuario se aprueban como un solo
grupo, y el aviso las muestra todas.

Cada solicitud se guarda en el registro del servicio con la aplicación, la hora, el comando
y si se aprobó. Al desactivar la función, `su` se oculta a todas las aplicaciones a la vez,
sin reiniciar.

---

Android es una marca comercial de Google LLC. Windows es una marca comercial del grupo de
empresas Microsoft. DSA no está afiliado a Google ni a Microsoft, ni cuenta con su respaldo
o patrocinio.
