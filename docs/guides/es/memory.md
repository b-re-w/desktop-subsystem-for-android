# Memoria y comportamiento en inactividad

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · [简体中文](../zh-CN/memory.md) · [繁體中文](../zh-TW/memory.md) · **Español** · [Français](../fr/memory.md) · [Deutsch](../de/memory.md) · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · [Tiếng Việt](../vi/memory.md) · [Bahasa Indonesia](../id/memory.md) · [Türkçe](../tr/memory.md)

Desktop Subsystem for Android™ (DSA) devuelve memoria a su equipo mientras Android se
ejecuta, no solo cuando se detiene. No hay nada que configurar para ello. Esta página explica
lo que ocurre y cómo verlo.

## Ver cuánta memoria usa Android

Abra la página **Memory** (memoria) de DSA Settings, o ejecute `dsa memory`. Ambos muestran:

- **On this PC** (en este equipo): lo que ocupan la máquina virtual de Android, el gestor de
  ventanas y cada ventana de aplicación.
- **Inside Android** (dentro de Android): la memoria en uso, la caché, la memoria comprimida
  y los procesos más grandes. Android lo comunica unos minutos después de iniciarse.

## Doze, el estado inactivo

Cuando todas las ventanas de Android llevan 30 segundos cerradas o minimizadas, la pantalla
de Android se apaga y Android entra en Doze, como hace un teléfono en un bolsillo.

- Las notificaciones, las alarmas y los widgets siguen funcionando en Doze. Entre tanto, la
  CPU descansa.
- `dsa status` indica `Dozing`. La página **General** de DSA Settings indica **Dozing
  (idle)**.
- Mostrar cualquier ventana de Android despierta Android al instante.

Android también sigue la suspensión de su equipo. Guarda su estado antes de que el equipo se
suspenda y ajusta su reloj cuando el equipo se reactiva.

## Congelación (desactivada de forma predeterminada)

La congelación detiene todos los procesos de Android. Así no usa CPU, pero las
notificaciones, las alarmas y los widgets también se detienen. Por eso DSA nunca congela
Android a menos que usted lo pida.

- Para congelarlo ahora: **Freeze** en **Freeze Android now** de la página **General**, o
  `dsa freeze`.
- Para congelarlo después de 30 minutos en Doze: active **Freeze Android when idle** en la
  página **Advanced** (opciones avanzadas), o `dsa settings set freeze on`.
- Abrir una ventana de Android, o la entrada en una de ellas, reanuda Android.

## Cómo vuelve la memoria

- La máquina virtual de Android toma memoria de su equipo solo cuando Android la usa por
  primera vez.
- La memoria que Android libera se devuelve a su equipo mientras Android se ejecuta.
- Cuando se cierra la última ventana de una aplicación, Android finaliza los procesos de esa
  aplicación, así que su memoria vuelve en pocos segundos. Una aplicación con un widget en el
  escritorio, o una que sigue trabajando en segundo plano (como un reproductor de música),
  sigue ejecutándose.
- Una aplicación que mantiene en ejecución un servicio en segundo plano sin ventana, sin
  notificación, sin sonido y sin widget se detiene al cabo de cinco minutos. Si Android la
  vuelve a iniciar una y otra vez, DSA espera más cada vez, hasta una hora.
- Mientras no hay ninguna ventana de aplicación abierta, Android descarta sus cachés de
  archivos y pasa la memoria inactiva a memoria comprimida, para que pueda volver una parte
  mayor.
- El gestor de ventanas devuelve la memoria que no usa 30 segundos después de que se cierre
  la última ventana de aplicación.

## Límite de memoria

El límite es **Automatic** (automático) de forma predeterminada, lo que no fija ningún
límite. Android empieza con 2 GB y crece mientras se ejecuta, hasta la memoria de su equipo
menos lo que se reserva Windows (el 10 % de la memoria o 2 GB, lo que sea mayor). Android
solo ocupa lo que usa.
Para fijar un límite, use **Memory limit** en la página **Memory**, o:

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # vuelve al modo automático
```

Un límite nuevo se aplica en el siguiente inicio de Android.

---

Android es una marca comercial de Google LLC. Windows es una marca comercial del grupo de
empresas Microsoft. DSA no está afiliado a Google ni a Microsoft, ni cuenta con su respaldo
o patrocinio.
