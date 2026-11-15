# Configuración

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · **Español** · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings** es la aplicación de configuración de Desktop Subsystem for Android™ (DSA)
en Windows. Ábrala desde el menú Inicio. Se muestra en inglés o en coreano, según el idioma
de visualización de Windows, y sigue su tema de Windows. Funciona con el teclado y con
lectores de pantalla.

Cada cambio que hace DSA Settings pasa por el comando `dsa`, así que la línea de comandos ve
los mismos valores:

```
dsa settings                        # todas las opciones y su valor
dsa settings set <name> <value>     # cambia una; los interruptores aceptan on u off
```

Algunas opciones se aplican a todos los usuarios del equipo, así que solo un administrador
puede cambiarlas. DSA Settings las marca con un icono de escudo y pide la aprobación de un
administrador cuando cambia una. Más abajo se marcan con "(admin)".

## Páginas

| Página | Qué encontrará en ella | Línea de comandos |
|---|---|---|
| **General** | El estado de Android, **Start**, **Stop**, **Freeze Android now**, el último problema, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** (memoria) | **Memory limit**, el tamaño de la máquina virtual de Android, lo que usa cada parte | `memory-limit`, `dsa memory` |
| **Apps** (aplicaciones) | Aplicaciones instaladas: **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. Para cada aplicación: **Resizable**, el tamaño con el que se abren las ventanas, **Scaling**, **Full screen request**, **When only the bars hide**, **Run as ARM**, **Root access**. Una fila para el acceso root de **adb shell** | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** (en segundo plano) | Aplicaciones que se ejecutan sin ventana: **Open**, **Stop**; si se pregunta antes de detenerlas | `dsa background`, `stop-without-asking` |
| **Widgets** | Los widgets de su escritorio y **Add to desktop** | `dsa widgets` |
| **Display & GPU** (pantalla y GPU) | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (solo Windows 11, activado de forma predeterminada), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** (identidad del dispositivo) | Primero este equipo, solo lectura: su nombre, fabricante y modelo, y **Device specifications** con **Copy**. Después, el fabricante, el modelo y la huella digital de compilación que Android comunica a las aplicaciones | `dsa settings device`, `device.<field>` |
| **Network** (red) | Cómo llega Android a la red, su dirección, quién responde a sus nombres (DNS) y su puerta de enlace, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** (actualizaciones) | **Android**: la versión y la compilación en ejecución, su nivel de parche de seguridad, la versión que sigue, la actualización de Android con su progreso y **Check now** (admin), **Restart Android into updates by itself**, **Update the Android image automatically** (admin), **Version to use** (admin). **DSA**: **Update DSA automatically** (admin), **Get pre-release versions** (admin), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** (opciones avanzadas) | Portapapeles, archivos, enlaces web, congelar en inactividad, copia de seguridad, restablecimiento, aplicaciones ARM, Google Play, contenido protegido, acceso root, trazas (admin), carpetas de registros | vea más abajo |
| **Logs** (registros) | El registro en vivo de Android, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** (acerca de) | Versiones, licencias, código fuente | |

## La página Advanced

| Opción | Valor predeterminado | Línea de comandos |
|---|---|---|
| **Share the clipboard with Android** | Activado | `clipboard` |
| **Share Windows folders with Android** | Activado | `file-sharing` |
| **Shared folders** | Las seis | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Desactivado | `freeze` |
| **Open web links in Android apps** | Desactivado | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Desactivado | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (no en ARM64) | | `dsa arm-translation fetch google` |
| **Use the default translator** (no en ARM64) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Sin instalar | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (en **Protected content**) | Sin instalar | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (admin) | Desactivado | `root-elevation` |
| **Input trace** (admin) | Desactivado | `input-trace` |
| **Resize trace** (admin) | Desactivado | `resize-trace` |

La copia de seguridad guarda una copia cifrada de los datos de sus aplicaciones en su carpeta
de usuario, con una clave que Windows guarda por usted. Para restaurarla en otro equipo, o
después de reinstalar Windows, necesita la clave de recuperación, así que anótela y guárdela
lejos del equipo. Google Play, Widevine y el acceso root se describen en
[Aplicaciones y ventanas](apps.md).

## Cuándo se aplica un cambio

- **Render GPU**: cuando el gestor de ventanas se vuelve a iniciar, en su siguiente inicio de
  sesión o cuando selecciona **Restart the window manager**.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for
  ARM**, Google Play, Widevine, la activación del acceso root y la versión de Android: en el
  siguiente inicio de Android. Para reiniciar Android, deténgalo en la página **General** y
  luego abra una aplicación.
- La configuración de ventana de una aplicación: a partir de la siguiente ventana que abra la
  aplicación.
- Todo lo demás, incluida la desactivación del acceso root: al instante.

## Cómo encuentra Android los nombres

Su equipo responde a cada nombre que buscan las aplicaciones de Android, con el mismo
sistema de resolución que usan las aplicaciones de Windows (en Linux y macOS, el del
sistema). Así, su archivo hosts, las reglas de nombres de su empresa, los servidores DNS
propios de una VPN y un filtro DNS se aplican a las aplicaciones de Android igual que al
equipo. Android no consulta por sí mismo a ningún servidor DNS.

- Lo mismo ocurre con **Advanced networking** activado: su equipo está en la misma red y
  conoce sus nombres, y una VPN o un filtro DNS del equipo se siguen aplicando.
- El **DNS privado** propio de Android está desactivado de forma predeterminada, porque
  consultaría a un servidor DNS sin pasar por su equipo. Si elige un servidor DNS privado en
  la configuración de Android, Android lo usa.
- Si DSA no puede responder durante un tiempo, Android consulta a los servidores DNS de la
  red hasta que DSA vuelva a poder hacerlo.
- La fila **DNS** de la página **Network**, y `dsa network`, indican quién responde ahora: el
  sistema de resolución de su equipo o los servidores DNS de la red, que se enumeran.

## Valores que acepta la línea de comandos

- `render-gpu`: `display` (la GPU del monitor principal, el valor predeterminado),
  `integrated`, `discrete`, o parte del nombre de una GPU tal como la muestra `dsa gpus`.
- `memory-limit`: `auto` (el valor predeterminado: sin límite fijo, hasta la memoria del
  equipo menos lo que se reserva Windows), o un tamaño como `4g` o `3072` (MiB).
- `shared-folders`: `all`, `none`, o nombres separados por comas, como
  `Documents,Pictures`.
- `adb-port`: un puerto TCP de 1 a 65535. El valor predeterminado es 58526.
- `app-fullscreen`: `screen` (el valor predeterminado) o `window`, para la petición clara de
  pantalla completa de una aplicación. `app-immersive`: `screen` o `window` (el valor
  predeterminado), para una aplicación que solo oculta sus barras. Consulte
  [Pantalla completa](apps.md#pantalla-completa).
- `touch-gestures-off`: `on` (el valor predeterminado) desactiva los gestos táctiles de tres
  y cuatro dedos de Windows 11 mientras una ventana de aplicación de DSA está en primer
  plano, para que las aplicaciones reciban hasta diez dedos, y restablece su propia
  configuración cuando otra ventana pasa a primer plano. Si Windows lo rechaza, la opción no
  se puede usar y la aplicación de configuración indica el motivo; aun así, puede desactivar
  los gestos para todas las aplicaciones en la Configuración de Windows, en Bluetooth y
  dispositivos, Entrada táctil.
- `host-certificates`: `on` (el valor predeterminado) permite que las aplicaciones de
  Android confíen en los certificados raíz añadidos a este equipo, como los de su empresa o
  los de un programa de seguridad.
- `adb-auto-connect`: `on` (el valor predeterminado) muestra Android en `adb devices` y en
  Android Studio mientras el modo de desarrollador está activado, sin `adb connect`.
- `image-auto-restart`: `on` (el valor predeterminado) reinicia Android al instante con una
  actualización de imagen descargada cuando no hay ninguna ventana de aplicación abierta. Si
  no, DSA pregunta antes o espera a que usted las cierre.
- `prerelease-updates`: `on` toma las versiones preliminares además de las estables, `off`
  toma solo las versiones estables. Su valor inicial depende del instalador que usó: un
  instalador de versión preliminar lo activa, uno estable lo deja desactivado, y las
  actualizaciones conservan su elección. Todas las versiones anteriores a la 1.0 son
  preliminares. Desactivarlo nunca hace volver DSA a una versión anterior; DSA espera a una
  versión estable más reciente que la instalada.
- `root-elevation`: `on` permite que las aplicaciones que usted autorice en la página
  **Apps** soliciten root. El nivel de cada aplicación se define con
  `dsa apps root <package> off|ask|remember`.
- `device.<field>`: `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint`, o una parte de la huella digital como `fingerprint.release`.
  `dsa settings reset device` restablece todos los valores.

---

Android es una marca comercial de Google LLC. Windows es una marca comercial del grupo de
empresas Microsoft. DSA no está afiliado a Google ni a Microsoft, ni cuenta con su respaldo
o patrocinio.
