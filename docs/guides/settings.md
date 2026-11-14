# Settings

**English** · [한국어](ko/settings.md) · [日本語](ja/settings.md) · [简体中文](zh-CN/settings.md) · [繁體中文](zh-TW/settings.md) · [Español](es/settings.md) · [Français](fr/settings.md) · [Deutsch](de/settings.md) · [Português (Brasil)](pt-BR/settings.md) · [Русский](ru/settings.md) · [Italiano](it/settings.md) · [Tiếng Việt](vi/settings.md) · [Bahasa Indonesia](id/settings.md) · [Türkçe](tr/settings.md)

**DSA Settings** is the settings app of Desktop Subsystem for Android™ (DSA) on Windows.
Open it from the Start menu. It shows in English or Korean, following your Windows display
language, and follows your Windows theme. It works with the keyboard and with screen
readers.

Every change DSA Settings makes goes through the `dsa` command, so the command line sees
the same values:

```
dsa settings                        # every setting and its value
dsa settings set <name> <value>     # change one; switches take on or off
```

Some settings apply to every user of the PC, so only an administrator can change them. DSA
Settings marks them with a shield icon and asks for administrator approval when you change
one. They are marked "(admin)" below.

## Pages

| Page | What you find there | Command line |
|---|---|---|
| **General** | Android's state, **Start**, **Stop**, **Freeze Android now**, the last problem, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** | **Memory limit**, the size of Android's VM, what each part uses | `memory-limit`, `dsa memory` |
| **Apps** | Installed apps: **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. For each app: **Resizable**, the size windows open at, **Scaling on another monitor**, **Full screen**, **Bars hidden**, **Run as ARM**, **Root access**. A row for **adb shell**'s root access | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** | Apps running with no window: **Open**, **Stop**; whether to ask before stopping | `dsa background`, `stop-without-asking` |
| **Widgets** | Widgets on your desktop, and **Add to desktop** | `dsa widgets` |
| **Display & GPU** | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (Windows 11 only, on by default), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** | This computer first, read only: its name, maker and model, and **Device specifications** with **Copy**. Then the manufacturer, model and build fingerprint Android reports to apps | `dsa settings device`, `device.<field>` |
| **Network** | How Android reaches the network, its address, who answers its names (DNS) and its gateway, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** | **Android**: the running version and build, its security patch level, the version it follows, the Android update with its progress and **Check now** (admin), **Restart Android into updates by itself**, **Update the Android image automatically** (admin), **Version to use** (admin). **DSA**: **Update DSA automatically** (admin), **Get pre-release versions** (admin), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** | Clipboard, files, web links, freeze when idle, backup, reset, ARM apps, Google Play, protected content, root access, traces (admin), log folders | see below |
| **Logs** | Android's live log, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** | Versions, licences, source code | |

## The Advanced page

| Setting | Default | Command line |
|---|---|---|
| **Share the clipboard with Android** | On | `clipboard` |
| **Share Windows folders with Android** | On | `file-sharing` |
| **Shared folders** | All six | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Off | `freeze` |
| **Open web links in Android apps** | Off | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Off | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (not on ARM64) | | `dsa arm-translation fetch google` |
| **Use the default translator** (not on ARM64) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Not installed | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (under **Protected content**) | Not installed | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (admin) | Off | `root-elevation` |
| **Input trace** (admin) | Off | `input-trace` |
| **Resize trace** (admin) | Off | `resize-trace` |

Backup keeps an encrypted copy of your apps' data in your user folder, with a key Windows
keeps for you. To restore it on another PC, or after Windows is reinstalled, you need the
recovery key, so write it down away from the PC. Google Play, Widevine and root access are
described in [Apps and windows](apps.md).

## When a change applies

- **Render GPU**: when the window manager starts again, at your next sign-in or when you
  select **Restart the window manager**.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for
  ARM**, Google Play, Widevine, turning on root access, and the Android version: at the next
  start of Android. To restart Android, stop it on the **General** page, then open an app.
- An app's window settings: from the next window the app opens.
- Everything else, including turning root access off: at once.

## How Android finds names

Your computer answers every name Android's apps look up, with the same resolver Windows apps
use (on Linux and macOS, the system's resolver). So your hosts file, your company's name rules,
a VPN's own DNS servers and a DNS filter apply to Android's apps as they do to the computer's.
Android does not ask a DNS server itself.

- This is the same with **Advanced networking** on: your computer is on the same network and
  knows its names, and a VPN or DNS filter on it still applies.
- Android's own **Private DNS** is off by default, because it would ask a DNS server around your
  computer. If you choose a private DNS server in Android's settings, Android uses it.
- If DSA cannot answer for a while, Android asks the network's DNS servers until it can again.
- The **DNS** row of the **Network** page, and `dsa network`, say which one answers now: your
  computer's resolver, or the network's DNS servers, which they list.

## Values the command line takes

- `render-gpu`: `display` (the GPU of the primary monitor, the default), `integrated`,
  `discrete`, or part of a GPU's name as `dsa gpus` lists it.
- `memory-limit`: `auto` (the default: no fixed limit, up to the PC's memory less what Windows keeps), or a
  size such as `4g` or `3072` (MiB).
- `shared-folders`: `all`, `none`, or names separated by commas, such as
  `Documents,Pictures`.
- `adb-port`: a TCP port from 1 to 65535. The default is 58526.
- `app-fullscreen`: `screen` (the default) or `window`, for an app's clear full screen
  request. `app-immersive`: `screen` or `window` (the default), for an app that only hides
  its bars. See [Full screen](apps.md#full-screen).
- `touch-gestures-off`: `on` (the default) turns Windows 11's three- and four-finger touch
  gestures off while a DSA app window is in front, so apps get up to ten fingers, and puts
  your own setting back when another window comes to the front. If Windows refuses, the
  setting can't be used and the settings app says why; you can still turn the gestures off
  for every app in Windows Settings, under Bluetooth & devices, Touch.
- `host-certificates`: `on` (the default) lets Android apps trust root certificates added to
  this PC, such as your company's or a security program's.
- `adb-auto-connect`: `on` (the default) shows Android in `adb devices` and Android Studio
  while developer mode is on, without `adb connect`.
- `image-auto-restart`: `on` (the default) restarts Android into a downloaded image update at
  once when no app window is open. Otherwise DSA asks first, or waits until you close them.
- `prerelease-updates`: `on` takes pre-releases as well as stable releases, `off` takes stable
  releases only. It starts from the installer you used: a pre-release installer turns it on,
  a stable one leaves it off, and upgrades keep your choice. Every release before 1.0 is a
  pre-release. Turning it off never moves DSA back to an older version; DSA waits for a stable
  release newer than the one installed.
- `root-elevation`: `on` lets apps you allow on the **Apps** page ask for root. Each app's
  level is set with `dsa apps root <package> off|ask|remember`.
- `device.<field>`: `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint`, or a part of the fingerprint such as `fingerprint.release`.
  `dsa settings reset device` puts every value back.

---

Android is a trademark of Google LLC. Windows is a trademark of the Microsoft group of
companies. DSA is not affiliated with, endorsed by or sponsored by Google or Microsoft.
