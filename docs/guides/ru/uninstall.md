# Удаление

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · **Русский** · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

Удалить Desktop Subsystem for Android™ (DSA) можно одним из двух способов:

- **Параметры** Windows > **Приложения** > **Установленные приложения**: найдите Desktop
  Subsystem for Android и выберите **Удалить**.
- Снова запустите программу установки и выберите **Uninstall**.

Удаление убирает следующее для всех пользователей ПК, вошедших в систему или нет:

- программу, службу `DsaHost`, команду `dsa`, DSA Settings и APK Installer;
- загруженные образы и обновления Android;
- пункты приложений Android в меню «Пуск» и их записи в **Установленных приложениях**;
- **Android** в проводнике, связь с файлами `.apk` и пункты контекстного меню;
- параметры и журналы DSA.

Оно **сохраняет** диск Android каждого пользователя, `%LOCALAPPDATA%\DSA\android.vhdx`, чтобы
при последующей установке ваши приложения и данные нашлись снова. Чтобы удалить и данные,
выполните `dsa reset-data --yes` перед удалением или удалите этот файл после него.

Если перенаправление папок переместило меню «Пуск» пользователя, пункты приложений Android
этого пользователя остаются, потому что программа удаления ищет только в стандартном месте.
Удалите их вручную.

Ваша настройка WSL не затрагивается, потому что DSA никогда её не меняет.

## Linux (пока не выпущен)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

Удаление пакета также удаляет добавленную им запись репозитория пакетов.

---

Android является товарным знаком Google LLC. Windows является товарным знаком группы компаний
Microsoft. DSA не связан с Google или Microsoft, не одобрен и не спонсируется ими.
