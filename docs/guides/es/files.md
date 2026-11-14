# Archivos

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · [简体中文](../zh-CN/files.md) · [繁體中文](../zh-TW/files.md) · **Español** · [Français](../fr/files.md) · [Deutsch](../de/files.md) · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · [Italiano](../it/files.md) · [Tiếng Việt](../vi/files.md) · [Bahasa Indonesia](../id/files.md) · [Türkçe](../tr/files.md)

Desktop Subsystem for Android™ (DSA) le permite abrir los archivos de Android en el
Explorador de archivos, compartir sus carpetas de Windows con aplicaciones de Android y
enviar un archivo a una aplicación de Android. Esta página describe DSA en Windows.

## Abrir los archivos de Android en el Explorador de archivos

Use cualquiera de estas opciones. Cada una inicia Android si no se está ejecutando.

- **Android** en el panel de navegación del Explorador de archivos, justo debajo de
  **Linux** de WSL.
- **Android files** (archivos de Android) en la página **Advanced** (opciones avanzadas) de
  DSA Settings.
- `dsa files open`. `dsa files` indica dónde está el almacenamiento, sin abrirlo.

Verá el almacenamiento compartido de Android: los archivos que guardaron las aplicaciones de
Android, como fotos y descargas. Puede copiar archivos hacia dentro y hacia fuera como en
cualquier carpeta. El Explorador de archivos llega al almacenamiento a través del cliente
WebDAV de Windows. Si el almacenamiento no responde, DSA muestra un error con el motivo.

## Compartir sus carpetas de Windows con aplicaciones de Android

Sus carpetas Documentos, Descargas, Imágenes, Música, Vídeos y Escritorio aparecen en el
almacenamiento compartido de Android dentro de `Host`, como `Host/Pictures`. Las
aplicaciones de Android pueden abrir y guardar archivos allí. Una aplicación necesita antes
el permiso de almacenamiento de Android, y Windows solo le permite hacer lo que puede hacer
su propia cuenta.

El uso compartido está activado de forma predeterminada. Para cambiarlo, abra la página
**Advanced** de DSA Settings:

- Desactive **Share Windows folders with Android** para no compartir ninguna carpeta.
- En **Shared folders** (carpetas compartidas), desmarque una carpeta para dejar de
  compartirla. Desaparece de Android al instante.

Desde la línea de comandos:

```
dsa settings set shared-folders Documents,Pictures   # solo estas; también all o none
dsa settings set file-sharing off                     # no compartir ninguna carpeta
```

Las versiones anteriores usaban una carpeta llamada `Windows` en lugar de `Host`. Android
quita la carpeta antigua cuando queda vacía, así que los archivos que guardó allí se
conservan.

## Enviar un archivo a una aplicación de Android

- **Arrastrar y soltar.** Arrastre un archivo desde el Explorador de archivos a la ventana de
  una aplicación. El archivo va a esa aplicación. Un APK va en cambio al APK Installer. Las
  carpetas no se envían.
- **Menú contextual.** Haga clic con el botón derecho en un archivo y seleccione **Open in an
  Android app** (abrir en una aplicación de Android) o **Share with an Android app**
  (compartir con una aplicación de Android). En Windows 11 están en **Mostrar más
  opciones**. Después, Android pregunta qué aplicación recibe el archivo.
- **Línea de comandos.** `dsa open <file>` y `dsa share <file>`.

Así se envían archivos de hasta 64 MiB. Un archivo más grande se rechaza antes de enviar
nada. Para dar a una aplicación un archivo más grande, póngalo en una carpeta compartida.

## Obtener un archivo de Android

- Guárdelo en Android dentro de `Host/<folder>`. Así estará en esa carpeta de su equipo.
- O abra **Android** en el Explorador de archivos y copie el archivo fuera.

---

Android es una marca comercial de Google LLC. Windows es una marca comercial del grupo de
empresas Microsoft. DSA no está afiliado a Google ni a Microsoft, ni cuenta con su respaldo
o patrocinio.
