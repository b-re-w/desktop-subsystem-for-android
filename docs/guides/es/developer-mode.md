# Modo de desarrollador y adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · **Español** · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

El modo de desarrollador permite que Android Debug Bridge (adb) de su equipo se conecte a su
Android en Desktop Subsystem for Android™ (DSA). Está desactivado de forma predeterminada.

adb no forma parte de DSA. Obténgalo de Android SDK Platform-Tools.

## Conectar adb

1. Abra la página **Network** (red) de DSA Settings y active **Developer mode** (modo de
   desarrollador). O ejecute `dsa settings set developer-mode on`.
2. Inicie Android, si no se está ejecutando: `dsa start --wait`.
3. Conéctese:

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android no le pide que permita la conexión. El puerto solo responde en este equipo, así
   que otros equipos no pueden conectarse.
4. Cuando termine, vuelva a desactivar **Developer mode**, o ejecute
   `dsa settings set developer-mode off`.

**Connect adb** (conectar adb) en la página **Network** muestra el comando exacto, y
`dsa settings` lo muestra en la línea `Developer mode`.

## Usar otro puerto

El puerto predeterminado es 58526. Para cambiarlo, edite **Port** (puerto) en **Connect
adb**, o ejecute:

```
dsa settings set adb-port 5555
```

## Lo que el modo de desarrollador no cambia

DSA nunca usa adb para su propio trabajo. Las ventanas, la entrada y las instalaciones usan
el canal de control propio de DSA, así que todo funciona igual con el modo de desarrollador
desactivado.

Para leer el registro de Android no necesita adb. Consulte
[Solución de problemas](troubleshooting.md).

---

Android es una marca comercial de Google LLC. DSA no está afiliado a Google, ni cuenta con
su respaldo o patrocinio.
