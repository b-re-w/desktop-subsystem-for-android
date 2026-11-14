# Primeros pasos

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · **Español** · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™ (DSA) ejecuta Android en una máquina virtual ligera con su
propio kernel. Cada aplicación de Android se abre en su propia ventana en su escritorio.

## Estado por plataforma

| Host | Estado |
|---|---|
| Windows 11, x64 | Publicado. |
| Windows 11, ARM64 | Publicado. Android también es ARM64, así que las aplicaciones ARM se ejecutan sin traducción. |
| Linux, x64 y ARM64 | En desarrollo. Los paquetes `.deb` y `.rpm` se compilan en CI; aún no se han publicado. |
| macOS, Apple silicon | En desarrollo. `DSA.app`, su `.pkg` y su `.dmg` se compilan en CI; aún no se han publicado. |

## Instalar en Windows

Necesita Windows 11 en un equipo x64 o ARM64, con la característica de Windows
**Plataforma de máquina virtual** activada.

1. Active Plataforma de máquina virtual. Abra **Activar o desactivar las características de
   Windows**, marque **Plataforma de máquina virtual**, seleccione **Aceptar** y reinicie. O
   ejecute `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` en un
   terminal de administrador. El programa de instalación no la activa por usted: se detiene
   con un mensaje si la característica está desactivada.
2. Descargue el programa de instalación y `SHA256SUMS.txt` desde la
   [última versión](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest):
   `DSA-v<version>-win-x64-setup.exe` en un equipo x64, `DSA-v<version>-win-arm64-setup.exe`
   en un equipo ARM64.
3. Compruebe la descarga. El hash que muestra este comando debe coincidir con la línea de
   `SHA256SUMS.txt`:

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. Ejecute el programa de instalación, seleccione **Install** (instalar) y apruebe el aviso
   de administrador.
5. Espere a la primera imagen de Android. Cuando el programa de instalación no la incluye,
   el servicio de DSA la descarga en segundo plano. `dsa status` muestra la descarga.
6. Abra una aplicación desde el menú Inicio, o compruébelo desde un terminal:

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## Qué instala el programa de instalación

- El servicio `DsaHost`, que ejecuta el Android de cada usuario.
- El comando `dsa`.
- **DSA Settings** (la configuración de DSA) en el menú Inicio.
- **Android** en el panel de navegación del Explorador de archivos, que abre los archivos de
  Android.
- El APK Installer (instalador de APK), que abre los archivos `.apk`.
- El gestor de ventanas de DSA (`dsa-wm`). Se inicia cuando usted inicia sesión y en ese
  momento inicia Android, así que la primera aplicación que abra no espera a que Android
  arranque.

Cada usuario de Windows tiene su propio Android y su propio disco de datos,
`%LOCALAPPDATA%\DSA\android.vhdx`. Los demás usuarios no pueden leerlo.

DSA usa su propio kernel y su propia máquina virtual. No usa ni modifica su configuración,
su kernel ni sus distribuciones de WSL.

## Actualizaciones

DSA se actualiza a sí mismo y actualiza la imagen de Android. Ambos están firmados, y DSA no
aplica nada cuya firma, tamaño o hash no coincida.

- **DSA en sí** se actualiza cuando no se está ejecutando el Android de ningún usuario.
  Hasta entonces, la actualización espera; `dsa update` la instala al instante. DSA nunca
  pasa a una versión inferior.
- **Canales de publicación.** A partir de la 1.0, DSA tiene un canal estable y un canal de
  versiones preliminares para las versiones que aún se están terminando. Su canal depende
  del instalador que usó, y las actualizaciones lo conservan. Un administrador lo cambia con
  **Get pre-release versions** (obtener versiones preliminares) en la página **Updates**
  (actualizaciones) de DSA Settings. Todas las versiones anteriores a la 1.0 son
  preliminares.
- **La imagen de Android** no depende del canal. Si una imagen no arranca, DSA vuelve a la
  anterior, y `dsa status` lo indica.

Un administrador puede desactivar las actualizaciones en la página **Updates** de DSA
Settings. Consulte [Configuración](settings.md) para ver todos los interruptores.

## Linux (en desarrollo, sin publicar)

DSA para Linux se compila como `.deb` (Debian, Ubuntu) y como `.rpm` (Fedora, openSUSE).
Necesita KVM (`/dev/kvm`) y un escritorio Wayland. Android se ejecuta en una máquina virtual
crosvm.

Al instalar un paquete también se añade el repositorio de paquetes firmado de DSA, de modo
que las versiones posteriores llegan a través de `apt`, `dnf`, `zypper` o el centro de
software. En Linux, DSA no tiene actualizador propio.

- Debian y Ubuntu: `/etc/apt/sources.list.d/dsa.sources`, con la clave en
  `/usr/share/keyrings/dsa-archive-keyring.gpg`.
- Fedora y openSUSE: `/etc/yum.repos.d/dsa.repo` (y `/etc/zypp/repos.d/dsa.repo` donde está
  zypper), con la clave en `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`.

El repositorio se servirá desde
`https://thisisthepy.github.io/desktop-subsystem-for-android/`. Aún no se ha publicado.

Una vez publicado, la primera instalación será así:

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

El paquete instala el servicio del sistema `dsa-hostd`, los servicios de usuario
`dsa-devices` y `dsa-compositor`, el comando `dsa` en `/usr/bin`, la entrada de menú
**DSA Files** y las acciones de archivo **Open in an Android app**, **Share with an Android
app** e **Install in Android**.

## macOS (en desarrollo, sin publicar)

DSA para macOS ejecuta Android sobre el framework Virtualization de Apple, en Apple silicon.
Se compila como `DSA.app`, en un paquete de instalación (`DSA-<version>.pkg`) y en una
imagen de disco (`DSA-<version>.dmg`). Las compilaciones de CI tienen una firma ad hoc, así
que solo se ejecutan en el equipo que las compiló. Aún no existe ninguna versión firmada con
un Developer ID.

## Siguiente paso

- [Aplicaciones y ventanas](apps.md)
- [Instalar APK](install-apks.md)

---

Android es una marca comercial de Google LLC. Windows es una marca comercial del grupo de
empresas Microsoft. macOS es una marca comercial de Apple Inc. DSA no está afiliado a
Google, Microsoft ni Apple, ni cuenta con su respaldo o patrocinio.
