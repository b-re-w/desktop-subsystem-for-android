# Instalar APK

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · [简体中文](../zh-CN/install-apks.md) · [繁體中文](../zh-TW/install-apks.md) · **Español** · [Français](../fr/install-apks.md) · [Deutsch](../de/install-apks.md) · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · [Tiếng Việt](../vi/install-apks.md) · [Bahasa Indonesia](../id/install-apks.md) · [Türkçe](../tr/install-apks.md)

Desktop Subsystem for Android™ (DSA) no incluye ninguna tienda de aplicaciones. Las
aplicaciones se instalan desde archivos APK, con el APK Installer o con el comando `dsa`.
Esta página describe DSA en Windows.

## Instalar con el APK Installer

1. Haga doble clic en un archivo `.apk` en el Explorador de archivos. Se abre en el **APK
   Installer** (instalador de APK).
2. Compruebe qué es el APK. La ventana lo muestra sin iniciar Android:
   - el nombre, el icono, el paquete y la versión de la aplicación;
   - el editor, tomado del certificado de firma, porque un APK no indica ningún editor;
   - el origen: el host desde el que se descargó el archivo, o su carpeta;
   - en **Capabilities** (capacidades), los primeros permisos que solicita. **More** (más)
     los muestra todos, junto con las versiones de Android para las que está hecho, su
     tamaño, sus tipos de procesador y el SHA-256 de su certificado.

   Cuando el APK tiene código solo para otro tipo de procesador, la ventana se lo advierte.
3. Seleccione **Install** (instalar). Cuando la aplicación ya está instalada, el botón dice
   **Update** (actualizar) o **Install again** (volver a instalar). Android se inicia antes si
   no se está ejecutando.
4. Con **Launch when ready** (abrir cuando esté lista) marcado (el valor predeterminado), la
   aplicación se abre en cuanto se instala y el instalador se cierra. Si no, seleccione
   **Launch** (abrir), o abra la aplicación más tarde desde el menú Inicio.

Mientras Android se está ejecutando, la ventana también indica si el archivo es una
instalación nueva, una actualización, la misma versión otra vez o algo que Android rechaza:
una versión anterior, o la misma aplicación firmada con otra clave.

También puede soltar archivos APK sobre el APK Installer, sobre DSA Settings o sobre la
ventana de una aplicación. Cada APK se abre en su propia ventana del instalador.

## Instalar desde la línea de comandos

```
dsa apk-info app.apk            # qué es el APK; lee el archivo sin iniciar Android
dsa install app.apk             # lo instala; inicia Android si no se está ejecutando
dsa install app.apk --progress  # además muestra cada paso
```

Cuando Android rechaza el APK, `dsa install` termina con el código 1, y su mensaje empieza
por `not installed (<reason>):`.

## Por qué se rechaza una instalación

| Motivo | Significado | Qué hacer |
|---|---|---|
| `incompatible` | La aplicación necesita otro tipo de procesador, una versión más reciente de Android o una función que este dispositivo no tiene | Consulte [Aplicaciones compiladas solo para ARM](apps.md#aplicaciones-compiladas-solo-para-arm) o [Versiones de Android](android-versions.md) |
| `conflicting-signature` | La aplicación ya está instalada, firmada con otra clave | Desinstale antes la aplicación instalada |
| `downgrade` | Hay instalada una versión más reciente de la aplicación | Desinstale antes la aplicación instalada |
| `invalid-apk` | El archivo no es un APK firmado que Android pueda leer | Vuelva a obtener el APK |
| `insufficient-storage` | Android no tiene espacio para la aplicación | Libere espacio en Android |
| `blocked` | Una directiva de este Android no permite la aplicación | |
| `other` | Cualquier otro caso; a continuación aparece el mensaje del propio Android | |

Un APK puede ocupar hasta 2 GiB. Las instalaciones se hacen de una en una: una segunda
espera a que termine la primera.

## Desinstalar una aplicación

Use cualquiera de estas opciones:

- **Configuración** de Windows > **Aplicaciones** > **Aplicaciones instaladas**.
- La página **Apps** de DSA Settings: **Uninstall** (desinstalar) junto a la aplicación.
- `dsa apps uninstall <package>`.

Las aplicaciones que forman parte de Android no se pueden desinstalar.

---

Android es una marca comercial de Google LLC. Windows es una marca comercial del grupo de
empresas Microsoft. DSA no está afiliado a Google ni a Microsoft, ni cuenta con su respaldo
o patrocinio.
