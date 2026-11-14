# Режим разработчика и adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · **Русский** · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

Режим разработчика позволяет Android Debug Bridge (adb) на вашем ПК подключаться к вашему
Android в Desktop Subsystem for Android™ (DSA). По умолчанию он выключен.

adb не входит в состав DSA. Его можно получить из Android SDK Platform-Tools.

## Подключение adb

1. Откройте страницу **Network** (сеть) в DSA Settings и включите **Developer mode**. Или
   выполните `dsa settings set developer-mode on`.
2. Запустите Android, если он не запущен: `dsa start --wait`.
3. Подключитесь:

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android не просит разрешить подключение. Порт отвечает только на этом ПК, поэтому другие
   компьютеры подключиться не могут.
4. Закончив работу, снова выключите **Developer mode** или выполните
   `dsa settings set developer-mode off`.

**Connect adb** на странице **Network** показывает точную команду, а `dsa settings` выводит её
в строке `Developer mode`.

## Другой порт

Порт по умолчанию: 58526. Чтобы изменить его, отредактируйте **Port** в разделе **Connect
adb** или выполните:

```
dsa settings set adb-port 5555
```

## Что режим разработчика не меняет

DSA никогда не использует adb для своей работы. Окна, ввод и установки используют собственный
канал управления DSA, поэтому всё работает одинаково и при выключенном режиме разработчика.

Чтобы читать журнал Android, adb не нужен. См. [Устранение неполадок](troubleshooting.md).

---

Android является товарным знаком Google LLC. DSA не связан с Google, не одобрен и не
спонсируется ею.
