# Desinstalar

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · **Español** · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

Desinstale Desktop Subsystem for Android™ (DSA) de cualquiera de estas formas:

- **Configuración** de Windows > **Aplicaciones** > **Aplicaciones instaladas**: busque
  Desktop Subsystem for Android y seleccione **Desinstalar**.
- Vuelva a ejecutar el programa de instalación y seleccione **Uninstall**.

La desinstalación quita lo siguiente, para todos los usuarios del equipo, hayan iniciado
sesión o no:

- el programa, el servicio `DsaHost`, el comando `dsa`, DSA Settings y el APK Installer;
- las imágenes de Android y las actualizaciones descargadas;
- las entradas del menú Inicio de las aplicaciones de Android y sus entradas en
  **Aplicaciones instaladas**;
- **Android** en el Explorador de archivos, la asociación de archivos `.apk` y las entradas
  del menú contextual;
- la configuración y los registros de DSA.

**Conserva** el disco de Android de cada usuario, `%LOCALAPPDATA%\DSA\android.vhdx`, para que
una instalación posterior vuelva a encontrar sus aplicaciones y sus datos. Para quitar
también sus datos, ejecute `dsa reset-data --yes` antes de desinstalar, o elimine ese
archivo después.

Si el redireccionamiento de carpetas movió el menú Inicio de un usuario, las entradas de las
aplicaciones de Android de ese usuario se quedan, porque el desinstalador solo busca en la
ubicación predeterminada. Elimínelas a mano.

Su configuración de WSL no se toca, porque DSA nunca la modifica.

## Linux (aún sin publicar)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

Al quitar el paquete también se quita la entrada del repositorio de paquetes que añadió.

---

Android es una marca comercial de Google LLC. Windows es una marca comercial del grupo de
empresas Microsoft. DSA no está afiliado a Google ni a Microsoft, ni cuenta con su respaldo
o patrocinio.
