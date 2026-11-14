# Начало работы

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · **Русский** · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™ (DSA) запускает Android в лёгкой виртуальной машине с
собственным ядром. Каждое приложение Android открывается в отдельном окне на вашем рабочем
столе.

## Состояние по платформам

| Хост | Состояние |
|---|---|
| Windows 11, x64 | Выпущен. |
| Windows 11, ARM64 | Выпущен. Android тоже ARM64, поэтому приложения ARM работают без трансляции. |
| Linux, x64 и ARM64 | В разработке. Пакеты `.deb` и `.rpm` собираются в CI, но пока не выпущены. |
| macOS, Apple silicon | В разработке. `DSA.app`, его `.pkg` и его `.dmg` собираются в CI, но пока не выпущены. |

## Установка в Windows

Вам понадобится Windows 11 на ПК с x64 или ARM64 и включённый компонент Windows **Платформа
виртуальной машины**.

1. Включите платформу виртуальной машины. Откройте **Включение или отключение компонентов
   Windows**, отметьте **Платформа виртуальной машины**, нажмите **ОК** и перезагрузите
   компьютер. Или выполните
   `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` в терминале
   администратора. Программа установки не включает этот компонент за вас: если он выключен, она
   останавливается с сообщением.
2. Скачайте программу установки и `SHA256SUMS.txt` из
   [последнего выпуска](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest):
   `DSA-v<version>-win-x64-setup.exe` на ПК с x64, `DSA-v<version>-win-arm64-setup.exe` на ПК
   с ARM64.
3. Проверьте загруженный файл. Хеш, который выводит эта команда, должен совпадать со строкой в
   `SHA256SUMS.txt`:

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. Запустите программу установки, выберите **Install** (Установить) и подтвердите запрос
   администратора.
5. Дождитесь первого образа Android. Если программа установки его не содержит, служба DSA
   загружает его в фоне. `dsa status` показывает ход загрузки.
6. Откройте приложение из меню «Пуск» или проверьте из терминала:

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## Что устанавливает программа установки

- Службу `DsaHost`, которая запускает Android каждого пользователя.
- Команду `dsa`.
- **DSA Settings** (параметры DSA) в меню «Пуск».
- **Android** в области навигации проводника, где открываются файлы Android.
- APK Installer (установщик APK), который открывает файлы `.apk`.
- Оконный менеджер DSA (`dsa-wm`). Он запускается при входе в систему и сразу запускает
  Android, поэтому первое открытое приложение не ждёт загрузки Android.

У каждого пользователя Windows свой Android и свой диск данных,
`%LOCALAPPDATA%\DSA\android.vhdx`. Другие пользователи не могут его читать.

DSA использует собственное ядро и собственную виртуальную машину. Он не использует и не меняет
ваши параметры, ядро и дистрибутивы WSL.

## Обновления

DSA обновляет себя и образ Android. Оба подписаны, и DSA ничего не применяет, если подпись,
размер или хеш не совпадают.

- **Сам DSA** обновляется, когда ни у одного пользователя не запущен Android. До тех пор
  обновление ждёт, а `dsa update` устанавливает его сразу. DSA никогда не понижает свою версию.
- **Каналы выпусков.** Начиная с 1.0 у DSA есть стабильный канал и предварительный канал для
  версий, работа над которыми ещё идёт. Ваш канал определяется программой установки, которой вы
  пользовались, и сохраняется при обновлениях. Администратор меняет его параметром **Get
  pre-release versions** (получать предварительные версии) на странице **Updates** (обновления)
  в DSA Settings. Все выпуски до 1.0 являются предварительными.
- **Образ Android** не зависит от канала. Если образ не загружается, DSA возвращается к
  предыдущему, и `dsa status` сообщает об этом.

Администратор может выключить обновления на странице **Updates** в DSA Settings. Все
переключатели описаны в разделе [Параметры](settings.md).

## Linux (в разработке, не выпущен)

DSA для Linux собирается в виде `.deb` (Debian, Ubuntu) и `.rpm` (Fedora, openSUSE). Ему нужны
KVM (`/dev/kvm`) и рабочий стол Wayland. Android работает в виртуальной машине crosvm.

При установке пакета также добавляется подписанный репозиторий пакетов DSA, поэтому следующие
версии приходят через `apt`, `dnf`, `zypper` или центр приложений. Собственного средства
обновления в Linux у DSA нет.

- Debian и Ubuntu: `/etc/apt/sources.list.d/dsa.sources`, ключ в
  `/usr/share/keyrings/dsa-archive-keyring.gpg`.
- Fedora и openSUSE: `/etc/yum.repos.d/dsa.repo` (и `/etc/zypp/repos.d/dsa.repo`, если есть
  zypper), ключ в `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`.

Репозиторий будет размещён по адресу
`https://thisisthepy.github.io/desktop-subsystem-for-android/`. Он пока не опубликован.

После выпуска первая установка будет выглядеть так:

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

Пакет устанавливает системную службу `dsa-hostd`, пользовательские службы `dsa-devices` и
`dsa-compositor`, команду `dsa` в `/usr/bin`, пункт меню **DSA Files** и действия с файлами
**Open in an Android app**, **Share with an Android app** и **Install in Android**.

## macOS (в разработке, не выпущен)

DSA для macOS запускает Android на Virtualization framework от Apple, на Apple silicon. Он
собирается как `DSA.app` в пакете установки (`DSA-<version>.pkg`) и в образе диска
(`DSA-<version>.dmg`). Сборки CI подписаны ad hoc, поэтому работают только на том компьютере,
где они собраны. Выпуска, подписанного Developer ID, пока нет.

## Что дальше

- [Приложения и окна](apps.md)
- [Установка APK](install-apks.md)

---

Android является товарным знаком Google LLC. Windows является товарным знаком группы компаний
Microsoft. macOS является товарным знаком Apple Inc. DSA не связан с Google, Microsoft или
Apple, не одобрен и не спонсируется ими.
