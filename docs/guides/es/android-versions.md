# Versiones de Android

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · [简体中文](../zh-CN/android-versions.md) · [繁體中文](../zh-TW/android-versions.md) · **Español** · [Français](../fr/android-versions.md) · [Deutsch](../de/android-versions.md) · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · [Tiếng Việt](../vi/android-versions.md) · [Bahasa Indonesia](../id/android-versions.md) · [Türkçe](../tr/android-versions.md)

Desktop Subsystem for Android™ (DSA) es compatible con las dos versiones principales más
recientes de Android: hoy, Android 16 y Android 17. De forma predeterminada ejecuta la más
reciente. Algunas aplicaciones solo funcionan en una de ellas, por eso puede elegir.

## Ver qué versión se ejecuta

Abra la página **Updates** (actualizaciones) de DSA Settings. En **Android** se muestran la
versión, su compilación, su nivel de parche de seguridad, la versión que sigue y el estado
de su actualización, con el progreso de la descarga. **Check now** (comprobar ahora) pide al
servicio que busque una actualización al instante. O ejecute:

```
dsa android version   # la versión que se ejecuta y las que puede elegir
```

## Cambiar la versión

Todos los usuarios del equipo ejecutan la misma versión de Android, así que solo un
administrador puede cambiarla.

1. Abra la página **Updates** de DSA Settings.
2. En **Version to use** (versión que se usará), elija una versión, o **Always the newest**
   (siempre la más reciente).
3. Seleccione **Change** (cambiar) y apruebe el aviso de administrador.

La versión elegida se descarga y se usa a partir del siguiente inicio de Android. Desde un
terminal de administrador:

```
dsa android use 16
dsa android use latest
```

### Pasar a una versión anterior borra los datos

Android no puede leer los datos que escribió una versión más reciente. Por eso, pasar a una
versión principal anterior borra los datos de Android de todos los usuarios del equipo: las
aplicaciones que instalaron, sus cuentas y todos los datos de las aplicaciones. La
configuración de DSA se conserva. Esto no se puede deshacer.

DSA Settings le pide que lo confirme con **Erase data and change** (borrar los datos y
cambiar). En la línea de comandos, añada `--erase-data`:

```
dsa android use 16 --erase-data
```

Pasar a una versión más reciente conserva sus aplicaciones y sus datos.

## Actualizaciones

- La versión elegida sigue recibiendo actualizaciones de seguridad.
- Cuando una versión deja de ser compatible, DSA pasa a la siguiente y conserva sus
  aplicaciones.
- DSA nunca pasa por sí solo a una versión principal anterior.
- Cada imagen se comprueba con una lista firmada antes de usarse. Una imagen que no arranca
  se sustituye por la anterior.

Un administrador puede desactivar las actualizaciones de imagen con **Update the Android
image automatically** en la página **Updates** de DSA Settings, o con
`dsa settings set image-updates off`.

---

Android es una marca comercial de Google LLC. Windows es una marca comercial del grupo de
empresas Microsoft. DSA no está afiliado a Google ni a Microsoft, ni cuenta con su respaldo
o patrocinio.
