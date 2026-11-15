# Параметры

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · **Русский** · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings** (параметры DSA) это приложение настроек Desktop Subsystem for Android™ (DSA)
в Windows. Откройте его из меню «Пуск». Оно отображается на английском или корейском, в
зависимости от языка интерфейса Windows, и следует теме Windows. Им можно пользоваться с
клавиатуры и с программами чтения с экрана.

Каждое изменение, которое вносит DSA Settings, проходит через команду `dsa`, поэтому в командной
строке видны те же значения:

```
dsa settings                        # все параметры и их значения
dsa settings set <name> <value>     # изменить один; переключатели принимают on или off
```

Некоторые параметры действуют для всех пользователей ПК, поэтому менять их может только
администратор. DSA Settings отмечает их значком щита и запрашивает одобрение администратора при
изменении. Ниже они отмечены пометкой «(адм.)».

## Страницы

| Страница | Что на ней находится | Командная строка |
|---|---|---|
| **General** | Состояние Android, **Start**, **Stop**, **Freeze Android now**, последняя проблема, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** | **Memory limit**, размер виртуальной машины Android, сколько использует каждая часть | `memory-limit`, `dsa memory` |
| **Apps** | Установленные приложения: **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. Для каждого приложения: **Resizable**, размер, с которым открываются окна, **Scaling**, **Full screen request**, **When only the bars hide**, **Run as ARM**, **Root access**. Строка для прав root у **adb shell** | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** | Приложения, работающие без окна: **Open**, **Stop**; спрашивать ли перед остановкой | `dsa background`, `stop-without-asking` |
| **Widgets** | Виджеты на рабочем столе и **Add to desktop** | `dsa widgets` |
| **Display & GPU** | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (только Windows 11, по умолчанию включено), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** | Сначала этот компьютер, только для чтения: его имя, производитель и модель, а также **Device specifications** с кнопкой **Copy**. Затем производитель, модель и отпечаток сборки, которые Android сообщает приложениям | `dsa settings device`, `device.<field>` |
| **Network** | Как Android выходит в сеть, его адрес, кто отвечает на запросы имён (DNS), его шлюз, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** | **Android**: запущенная версия и сборка, уровень исправлений безопасности, версия, которой она следует, обновление Android с ходом загрузки и **Check now** (адм.), **Restart Android into updates by itself**, **Update the Android image automatically** (адм.), **Version to use** (адм.). **DSA**: **Update DSA automatically** (адм.), **Get pre-release versions** (адм.), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** | Буфер обмена, файлы, веб-ссылки, заморозка при простое, резервное копирование, сброс, приложения ARM, Google Play, защищённое содержимое, права root, трассировка (адм.), папки журналов | см. ниже |
| **Logs** | Журнал Android в реальном времени, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** | Версии, лицензии, исходный код | |

## Страница Advanced

| Параметр | По умолчанию | Командная строка |
|---|---|---|
| **Share the clipboard with Android** | Вкл. | `clipboard` |
| **Share Windows folders with Android** | Вкл. | `file-sharing` |
| **Shared folders** | Все шесть | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Выкл. | `freeze` |
| **Open web links in Android apps** | Выкл. | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Выкл. | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (нет на ARM64) | | `dsa arm-translation fetch google` |
| **Use the default translator** (нет на ARM64) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Не установлено | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (в разделе **Protected content**) | Не установлено | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (адм.) | Выкл. | `root-elevation` |
| **Input trace** (адм.) | Выкл. | `input-trace` |
| **Resize trace** (адм.) | Выкл. | `resize-trace` |

Резервное копирование хранит зашифрованную копию данных ваших приложений в вашей папке
пользователя, с ключом, который Windows хранит для вас. Чтобы восстановить её на другом ПК или
после переустановки Windows, нужен ключ восстановления, поэтому запишите его и храните отдельно
от ПК. Google Play, Widevine и права root описаны в разделе [Приложения и окна](apps.md).

## Когда изменение начинает действовать

- **Render GPU**: при следующем запуске оконного менеджера, то есть при следующем входе в систему
  или когда вы выберете **Restart the window manager**.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for
  ARM**, Google Play, Widevine, включение прав root и версия Android: при следующем запуске
  Android. Чтобы перезапустить Android, остановите его на странице **General**, а затем
  откройте приложение.
- Параметры окна приложения: начиная со следующего окна, которое откроет приложение.
- Всё остальное, включая выключение прав root: сразу.

## Как Android находит имена

На каждый запрос имени от приложений Android отвечает ваш компьютер, тем же сопоставителем
имён, которым пользуются приложения Windows (в Linux и macOS системным сопоставителем). Поэтому
ваш файл hosts, правила имён вашей компании, собственные DNS-серверы VPN и DNS-фильтр действуют
для приложений Android так же, как для компьютера. Android сам не обращается к DNS-серверу.

- То же самое происходит при включённом **Advanced networking**: ваш компьютер находится в той
  же сети и знает её имена, а VPN или DNS-фильтр на нём по-прежнему действуют.
- Собственный **Персональный DNS-сервер** (Private DNS) Android по умолчанию выключен, потому
  что он обращался бы к DNS-серверу в обход вашего компьютера. Если вы выберете персональный
  DNS-сервер в настройках Android, Android будет его использовать.
- Если DSA какое-то время не может отвечать, Android обращается к DNS-серверам сети, пока DSA
  снова не сможет отвечать.
- Строка **DNS** на странице **Network** и команда `dsa network` сообщают, кто отвечает сейчас:
  сопоставитель имён вашего компьютера или DNS-серверы сети, которые они перечисляют.

## Значения для командной строки

- `render-gpu`: `display` (GPU основного монитора, по умолчанию), `integrated`, `discrete`
  или часть имени GPU, как его выводит `dsa gpus`.
- `memory-limit`: `auto` (по умолчанию: без фиксированного предела, вплоть до памяти ПК за
  вычетом того, что оставляет себе Windows) или размер, например `4g` или `3072` (МиБ).
- `shared-folders`: `all`, `none` или имена через запятую, например `Documents,Pictures`.
- `adb-port`: TCP-порт от 1 до 65535. По умолчанию 58526.
- `app-fullscreen`: `screen` (по умолчанию) или `window`, для явного запроса приложения на
  полноэкранный режим. `app-immersive`: `screen` или `window` (по умолчанию), для приложения,
  которое только скрывает свои панели. См. [Полноэкранный режим](apps.md#полноэкранный-режим).
- `touch-gestures-off`: `on` (по умолчанию) выключает трёх- и четырёхпальцевые сенсорные жесты
  Windows 11, пока окно приложения DSA находится на переднем плане, чтобы приложения получали до
  десяти пальцев, и возвращает вашу собственную настройку, когда на передний план выходит другое
  окно. Если Windows отказывает, этот параметр использовать нельзя, и приложение настроек
  объясняет причину. Жесты всё равно можно выключить для всех приложений в параметрах Windows, в
  разделе «Bluetooth и устройства» > «Сенсорный ввод».
- `host-certificates`: `on` (по умолчанию) позволяет приложениям Android доверять корневым
  сертификатам, добавленным на этот ПК, например сертификатам вашей компании или программы
  безопасности.
- `adb-auto-connect`: `on` (по умолчанию) показывает Android в `adb devices` и в Android
  Studio, пока включён режим разработчика, без `adb connect`.
- `image-auto-restart`: `on` (по умолчанию) сразу перезапускает Android в загруженное
  обновление образа, когда не открыто ни одного окна приложения. Иначе DSA сначала спрашивает
  или ждёт, пока вы их закроете.
- `prerelease-updates`: `on` получает предварительные выпуски вместе со стабильными, `off`
  получает только стабильные. Начальное значение зависит от программы установки, которой вы
  пользовались: программа установки предварительного выпуска включает его, стабильная оставляет
  выключенным, а обновления сохраняют ваш выбор. Все выпуски до 1.0 являются предварительными.
  Выключение никогда не откатывает DSA на более старую версию: DSA ждёт стабильного выпуска,
  более нового, чем установленный.
- `root-elevation`: `on` позволяет приложениям, которым вы разрешили это на странице **Apps**,
  запрашивать права root. Уровень для каждого приложения задаётся командой
  `dsa apps root <package> off|ask|remember`.
- `device.<field>`: `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint` или часть отпечатка, например `fingerprint.release`.
  `dsa settings reset device` возвращает все значения.

---

Android является товарным знаком Google LLC. Windows является товарным знаком группы компаний
Microsoft. DSA не связан с Google или Microsoft, не одобрен и не спонсируется ими.
