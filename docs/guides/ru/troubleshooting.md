# Устранение неполадок и журналы

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · **Русский** · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

Эта страница поможет выяснить, почему Desktop Subsystem for Android™ (DSA) не работает,
прочитать журнал Android и отправить сообщение о проблеме. Она описывает Windows, а там, где
это отмечено, и Linux.

## Сначала проверьте состояние

```
dsa status          # состояние Android, проблемы и ожидающее обновление
dsa start --wait    # запустить Android и дождаться окончания загрузки
```

Страница **General** (общие) в DSA Settings показывает то же состояние.

| `dsa status` | DSA Settings | Значение |
|---|---|---|
| `Active` | **Running** | Android загрузился, и агент DSA в нём подключён |
| `Dozing` | **Dozing (idle)** | Android простаивает. См. [Память](memory.md) |
| `Frozen` | **Frozen** | Android заморожен, пока вы не откроете одно из его окон |
| `Stalled` | **Not ready** | Android работает, но слишком долго не готов. Следующая строка объясняет причину и что делать |

## Частые проблемы

| Проблема | Что делать |
|---|---|
| Программа установки сообщает, что платформа виртуальной машины выключена | Включите этот компонент Windows, перезагрузите компьютер и снова запустите программу установки. См. [Начало работы](getting-started.md) |
| Не открывается ни одно окно приложения | Выйдите из системы и войдите снова. Оконный менеджер запускается при входе в систему |
| Приложение открывается с неправильным размером | `dsa app-window <package> reset` |
| Android не загружается после обновления образа | Ничего делать не нужно. DSA сам возвращается к предыдущему образу, и `dsa status` сообщает об этом |
| В Android нет сети | Откройте страницу **Network** в DSA Settings или выполните `dsa network`. Там показаны режим, адрес, DNS-серверы и проблемы, если они есть |
| Android сломан, и ничто другое не помогает | Сбросьте данные Android (см. ниже) |

### Сброс данных Android

Сброс стирает все установленные вами приложения, ваши аккаунты и все данные приложений. После
этого Android запускается таким, каким был при первой установке. Образ Android и параметры DSA
сохраняются.

- В DSA Settings: страница **Advanced** (дополнительно) > **Reset Android's data** > **Reset**.
- В командной строке: `dsa reset-data` сообщает, что будет потеряно. `dsa reset-data --yes`
  стирает данные.

## Чтение журнала Android

Журнал Android хранится в памяти внутри Android и никогда не записывается в файл. Он попадает
на ваш ПК, только пока его что-то читает.

- В DSA Settings: страница **Logs** (журналы). Можно фильтровать по тегу или тексту и ставить
  на паузу.
- В командной строке:

  ```
  dsa logcat                        # следить за журналом; Ctrl+C завершает
  dsa logcat -d -b all *:W          # что сейчас есть в журнале, предупреждения и выше
  dsa logcat ActivityManager:I *:S  # только один тег
  ```

## Отправка сообщения о проблеме

1. Сохраните диагностику. На странице **Logs** выберите **Save diagnostics** или выполните:

   ```
   dsa logs collect -o diag.zip
   ```

   Это занимает минуту или больше.
2. Приложите zip-файл к
   [задаче](https://github.com/thisisthepy/desktop-subsystem-for-android/issues).

Архив содержит весь журнал Android, записи о сбоях (tombstones, трассировки ANR, dropbox,
pstore), сводку системных служб Android и те журналы DSA на этом ПК, которые относятся к вам,
без строк других пользователей. `summary.txt` в архиве сообщает, что собрать не удалось.

## Где находятся журналы в Windows

| Папка | Файлы |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (оконный менеджер), `dsa-app-window.log` (окна приложений) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (служба), `android-<SID>.log` (консоль Android каждого пользователя, не более 2 МиБ за загрузку), `setup-<version>.log` (самообновления) |

Чтобы открыть их, используйте **Open my log folder** и **Open the service's log folder
(administrators)** на странице **Advanced** в DSA Settings.

## Где находятся журналы в Linux (пока не выпущен)

Службы пишут в журнал systemd:

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## Все команды

`dsa` без аргументов выводит все команды. Добавьте `--json`, чтобы получить вывод, который могут
читать программы.

---

Android является товарным знаком Google LLC. Windows является товарным знаком группы компаний
Microsoft. DSA не связан с Google или Microsoft, не одобрен и не спонсируется ими.
