# Solución de problemas y registros

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · **Español** · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

Esta página le ayuda a averiguar por qué no funciona Desktop Subsystem for Android™ (DSA), a
leer el registro de Android y a enviar un informe de problemas. Trata Windows, y Linux donde
se indica.

## Compruebe primero el estado

```
dsa status          # estado de Android, cualquier problema y cualquier actualización pendiente
dsa start --wait    # inicia Android y espera a que haya arrancado
```

La página **General** de DSA Settings muestra el mismo estado.

| `dsa status` | DSA Settings | Significado |
|---|---|---|
| `Active` | **Running** | Android ha arrancado y el agente de DSA que contiene está conectado |
| `Dozing` | **Dozing (idle)** | Android está inactivo. Consulte [Memoria](memory.md) |
| `Frozen` | **Frozen** | Android está congelado hasta que abra una de sus ventanas |
| `Stalled` | **Not ready** | Android se ejecuta, pero ha tardado demasiado. La línea siguiente indica el motivo y qué hacer |

## Problemas habituales

| Problema | Qué hacer |
|---|---|
| El programa de instalación indica que Plataforma de máquina virtual está desactivada | Active la característica de Windows, reinicie y vuelva a ejecutar el programa de instalación. Consulte [Primeros pasos](getting-started.md) |
| No se abre ninguna ventana de aplicación | Cierre la sesión y vuelva a iniciarla. El gestor de ventanas se inicia al iniciar sesión |
| Una aplicación se abre con un tamaño incorrecto | `dsa app-window <package> reset` |
| Android no arranca después de una actualización de imagen | Nada. DSA vuelve por sí solo a la imagen anterior, y `dsa status` lo indica |
| Android no tiene red | Abra la página **Network** (red) de DSA Settings, o ejecute `dsa network`. Muestra el modo, la dirección, los servidores DNS y cualquier problema |
| Android está dañado y nada más funciona | Restablezca los datos de Android (más abajo) |

### Restablecer los datos de Android

El restablecimiento borra todas las aplicaciones que instaló, sus cuentas y todos los datos
de las aplicaciones. Después, Android se inicia como cuando se instaló por primera vez. La
imagen de Android y la configuración de DSA se conservan.

- En DSA Settings: página **Advanced** (opciones avanzadas) > **Reset Android's data** >
  **Reset**.
- En la línea de comandos: `dsa reset-data` indica lo que se perdería.
  `dsa reset-data --yes` lo borra.

## Leer el registro de Android

El registro de Android permanece en la memoria de Android y nunca se escribe en un archivo.
Solo llega a su equipo mientras algo lo lee.

- En DSA Settings: la página **Logs** (registros). Puede filtrar por etiqueta o por texto, y
  pausarlo.
- En la línea de comandos:

  ```
  dsa logcat                        # sigue el registro; Ctrl+C lo termina
  dsa logcat -d -b all *:W          # lo que contiene ahora el registro, advertencias o superior
  dsa logcat ActivityManager:I *:S  # una sola etiqueta
  ```

## Enviar un informe de problemas

1. Guarde los diagnósticos. En la página **Logs**, seleccione **Save diagnostics** (guardar
   diagnósticos), o ejecute:

   ```
   dsa logs collect -o diag.zip
   ```

   Tarda un minuto o más.
2. Adjunte el archivo zip a una
   [incidencia](https://github.com/thisisthepy/desktop-subsystem-for-android/issues).

El archivo contiene el registro completo de Android, sus registros de fallos (tombstones,
trazas ANR, dropbox, pstore), un resumen de los servicios del sistema de Android y los
registros de DSA de este equipo que le conciernen, sin las líneas de otros usuarios.
`summary.txt` dentro del archivo indica lo que no se pudo recopilar.

## Dónde están los registros en Windows

| Carpeta | Archivos |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (el gestor de ventanas), `dsa-app-window.log` (las ventanas de las aplicaciones) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (el servicio), `android-<SID>.log` (la consola de Android de cada usuario, como máximo 2 MiB por arranque), `setup-<version>.log` (las actualizaciones automáticas) |

Para abrirlas, use **Open my log folder** y **Open the service's log folder
(administrators)** en la página **Advanced** de DSA Settings.

## Dónde están los registros en Linux (aún sin publicar)

Los servicios escriben en el diario de systemd:

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## Todos los comandos

`dsa` sin argumentos muestra todos los comandos. Añada `--json` para obtener una salida que
los programas puedan leer.

---

Android es una marca comercial de Google LLC. Windows es una marca comercial del grupo de
empresas Microsoft. DSA no está afiliado a Google ni a Microsoft, ni cuenta con su respaldo
o patrocinio.
