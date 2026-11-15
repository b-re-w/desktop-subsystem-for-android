# Apps and windows

**English** · [한국어](ko/apps.md) · [日本語](ja/apps.md) · [简体中文](zh-CN/apps.md) · [繁體中文](zh-TW/apps.md) · [Español](es/apps.md) · [Français](fr/apps.md) · [Deutsch](de/apps.md) · [Português (Brasil)](pt-BR/apps.md) · [Русский](ru/apps.md) · [Italiano](it/apps.md) · [Tiếng Việt](vi/apps.md) · [Bahasa Indonesia](id/apps.md) · [Türkçe](tr/apps.md)

Each Android app task opens as a desktop window of its own, with its own taskbar button.
There is no emulator frame around a phone screen. This page describes Desktop Subsystem
for Android™ (DSA) on Windows.

## Open an app

- **Start menu.** Each installed app has a Start menu entry with its own name and icon, so
  Windows search finds it. Opening it starts Android first when Android is not running.
- **DSA Settings.** On the **Apps** page, select **Open** next to the app.
- **Command line.** `dsa apps list` lists the installed apps with their packages.
  `dsa launch <package>` opens one, for example `dsa launch com.android.settings`.

When another program's Start menu entry already has the app's name, DSA adds the package to
the name, such as `Clock (com.example.clock)`. DSA changes or removes only the entries it
made.

## Desktop shortcuts and jump lists

App icons work the way Windows apps' do, not the way an Android home screen does.

- **Desktop shortcut.** On the **Apps** page of DSA Settings, select **Create desktop
  shortcut** next to the app. You can also right-click the app's taskbar button and select
  **Create desktop shortcut**. From the command line: `dsa apps desktop-shortcut <package>`.
  The shortcut opens the app as its Start menu entry does.
- **App shortcuts.** The shortcuts an Android app offers, such as a messenger's new chat,
  show when you right-click the app's taskbar button or its Start menu entry. Selecting one
  opens that screen of the app in a window, starting Android first when it is not running.
  When the app changes its shortcuts, the list follows. From the command line:
  `dsa apps shortcut <package> <id>`; `dsa apps info <package> --json` lists the IDs.
- **Shortcuts an app asks for.** When an app asks to add one of its shortcuts to your
  desktop, DSA asks you first and adds it only if you agree.

When an app changes its name, for example after an update, its Start menu entry, its
desktop shortcuts and its pin on the taskbar follow the new name, and keep working. A
desktop shortcut you deleted is not made again. Uninstalling an app removes its desktop
shortcuts too.

## Work with app windows

- Each task has its own window, taskbar button and Alt+Tab entry, with the app's name and
  icon. Task Manager lists each app under its own name.
- The title bar is part of the app: the app's colours reach the top of the window, and the
  Windows 11 caption buttons sit over it. The app's icon and name are at the left. When the
  app can go back, the icon turns into a back arrow; select it to go back.
- Snap layouts (hover over the maximize button), dragging the title bar, double-clicking it
  to maximize, and live resizing work. Resizing does not restart the app. A maximized window
  fills the monitor's work area and leaves the taskbar visible.
- There is no Android navigation bar. An app's bottom bar sits at the bottom edge of the
  window, with no empty band below it.
- Mouse (with hover), touch, pen (pressure, eraser and barrel button) and keyboard all work.
  Text comes from your Windows input method, including Korean and other languages; the
  composition shows inline in the Android editor, and the candidate window sits next to the
  cursor. There is no Android keyboard app.
- Drag files from File Explorer onto an app's window, or drag files out of an app into File
  Explorer.
- Screen readers such as Narrator and NVDA read the app through UI Automation.
- Closing a window asks Android to end that task.

## The window menu

Right-click the title bar, select the app's icon, or press Alt+Space. After the usual
Windows items, the menu has:

- **Full screen**: fills the whole screen, like F11. Select it again, or press F11, to leave.
- **New window**: opens another window of the app, for apps that support several.
- **Screenshot**: copies the app's screen, without the title bar, to the clipboard and saves
  it in the Screenshots folder of your Pictures.
- **Aspect ratio**: opens Android's setting for the app's aspect ratio, for apps that look
  stretched in a large window.
- **Open by default**: chooses whether the links the app supports open in the app or in
  your browser.
- **App info**: opens the app's entry on the **Apps** page of DSA Settings.

## Full screen

When an app asks for full screen, its window shows it one of two ways:

- **On the whole screen**: the window fills the monitor, and the title bar hides. Touch the
  top edge of the screen with the pointer to show it, with minimize and close; the middle
  button leaves full screen.
- **Inside the window**: the window keeps its size and title bar, and the app fills it.

Apps ask in two kinds of ways, and each kind has its own default:

| Kind of request | Example | Default |
|---|---|---|
| A clear full screen request | A video player's full screen button | On the whole screen |
| Hiding the bars only | A feed of short videos | Inside the window |

Change the defaults under **App full screen** and **Apps that hide their bars** on the
**Display & GPU** page of DSA Settings, or for one app under **Full screen** and **Bars
hidden** on the **Apps** page. An app's own choice comes first. F11 and **Full screen** in
the window menu switch full screen on the whole screen at any time, whatever the setting.

From the command line:

```
dsa settings set app-fullscreen screen|window           # default for clear requests
dsa settings set app-immersive screen|window            # default for hidden bars
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## Set an app's window size

On the **Apps** page of DSA Settings:

- Turn off **Resizable** to keep the app's windows at the size they open at.
- Pick the size the app's windows open at: the size Android chooses, or a phone or tablet
  size in portrait or landscape.

The change applies to the next window the app opens. From the command line:

```
dsa app-window <package> --fixed-size on        # cannot be resized or maximized
dsa app-window <package> --start-size 1280x800  # opens at this size; auto lets Android choose
dsa app-window <package> --dpi-scaling system   # how it scales on another monitor; see below
dsa app-window <package> reset                  # back to the defaults
```

## Start an app below the title bar

An app draws up to the top of its window, under the title bar. A few apps then squeeze their
top bar or hide part of it under the title bar. For such an app, turn on **Start below the
title bar** on the **Apps** page of DSA Settings. The app's content then starts below the title
bar, and the title bar takes the color the app shows there. It is off by default, and applies
the next time the app's window opens. From the command line:

```
dsa app-window <package> --below-title-bar on   # off goes back to the default
```

## Move an app to a monitor of another scale

An app opens at the scale of the monitor its window opens on, from its first frame, so it is
not recreated right after it opens. When you move a window to a monitor of another scale,
the app keeps its size on the screen. Each app does this one of these ways, which you pick
under **Scaling on another monitor** on the **Apps** page, or with
`dsa app-window <package> --dpi-scaling`:

- **Automatic** (`auto`, the default): **System** for the apps on DSA's built-in list of apps
  that break when their scale changes while they run, and **App** for every other app. The
  list holds YouTube: it keeps its layout for the scale it started at, so after a move its
  bottom tabs and miniplayer would stay broken until it restarts. Settings shows such an
  app's default as **Scaling: system (recommended for this app)**. Your own choice for the
  app comes first.
- **App** (`app`): the window keeps its size in pixels while you drag it and after you let go,
  and shows the app as it was drawn, so the app looks larger or smaller on the new monitor
  until you let go. When you let
  go, the app redraws at the new monitor's scale, and its text stays sharp: Android recreates
  its screen once and restores its state (scroll position, typed text), so it redraws briefly
  after each move, as a Windows program does with "Application" scaling. This holds even for
  an app that says it handles a change of scale itself, because some of those apps redraw
  only part of their screen at the new scale.
- **System** (`system`): the app keeps its scale, and its window keeps its size in pixels and
  shows the app as it was drawn, so the app stays sharp and looks larger or smaller on a
  monitor of another scale. The app is never recreated.
- **System (forced)** (`system-forced`): the app is given the new scale without a restart,
  even if it does not say it handles one. Not available yet: it needs a change to Android
  that this version does not have, so until then it works like **System**.

Text in apps follows the Windows text size (Settings > Accessibility > Text size).

## Notifications

Android notifications show as Windows notifications, with the app's name and icon.

- Select a notification to open the app.
- Dismiss it, and it is dismissed in Android too, unless the app keeps it shown.
- Notifications keep arriving while Android is in Doze. See [Memory](memory.md).

## Apps running in the background

An app that keeps running after its window closes, such as a music player, shows as an
icon in the notification area. Its tooltip is the app's notification.

- Select the icon to open the app.
- Right-click it for **Open**, the notification's own actions, and **Stop**.
- **Stop** asks first. Tick **Do not ask again** to skip the question from then on. To be
  asked again, turn on **Ask before stopping an app from the notification area** on the
  **Running in the background** page of DSA Settings.
- An app that plays media also works with the media keys and the Windows media controls.

The **Running in the background** page of DSA Settings lists these apps, with **Open** and
**Stop** for each. From the command line: `dsa background`, `dsa background open <package>`
and `dsa background stop <package>`.

## Widgets on the desktop

An Android widget can sit on your desktop in a window of its own. It stays below other
windows, stays when you show the desktop, and is not in the taskbar or Alt+Tab.

1. Open the **Widgets** page of DSA Settings.
2. Under **Widgets your apps offer**, select **Add to desktop** next to a widget. If the app
   asks you to set the widget up, its setup opens in a window.
3. To move a widget, press anywhere on it and hold still for half a second. It lifts, with a
   border and a handle on each edge it can be resized along. Drag it to move it, or drag a
   handle to resize it. A quick drag stays the widget's own, so its lists still scroll. You
   can also drag an edge of the widget to resize it without lifting it first; only the edges
   it can be resized along do this. DSA remembers where you put it and how large.
4. Right-click the widget, or let go after the long press, for **Widget settings** (when the
   widget can be set up again), **Resize**, to pick one of the sizes it allows, and
   **Remove**. The menu key and Shift+F10 open it too. The arrow keys, Enter and Esc work in
   this menu, and a letter picks an item: R for **Resize** and M for **Remove**.

From the command line: `dsa widgets list`, `dsa widgets add <provider>` and
`dsa widgets remove <id>`.

## Links and the clipboard

- A web link you tap in an Android app opens in your default browser. A link that an
  installed app has verified as its own opens in that app.
- To open every web link in Android apps instead, turn on **Open web links in Android
  apps** on the **Advanced** page of DSA Settings (`dsa settings set links-in-android on`).
- Text you copy is shared both ways between Windows and Android. To stop it, turn off
  **Share the clipboard with Android** on the **Advanced** page
  (`dsa settings set clipboard off`).

## Google Play and protected video

DSA ships no Google software. If you want it, DSA downloads it from its source for you,
after showing you the source and the terms. Both apply from the next start of Android, and
both can be removed again.

- **Google Play.** On the **Advanced** page of DSA Settings, select **Install** under
  **Google Play**, or run `dsa gapps install`. DSA downloads the public LiteGapps Core
  package (Google Play services, the Play Store and account sync) for your Android version.
  This Android is not Google-certified, so Google may refuse to sign you in until you
  register the device: **Register this device with Google** shows the Android ID and opens
  the registration page. `dsa gapps status` shows the state, and `dsa gapps remove` removes
  Google's apps, even while Android is stopped.
- **Widevine.** Streaming apps need Google's Widevine module to play protected video. On the
  **Advanced** page, select **Get** under **Protected content**, or run
  `dsa drm widevine install`. DSA downloads Google's Android emulator image for your Android
  version (about 2 GB), keeps only the Widevine module, unchanged, and deletes the rest.
  Widevine here is security level L3, so streaming services send standard definition at
  most; for high definition, use a Windows browser or app. `dsa drm widevine remove` removes
  it.

## Apps built only for ARM

On an x64 PC, an app with code only for ARM processors runs through an ARM translator.
Android on an x64 PC comes with one, the open source translator Digitalis, and uses it from
its first start. You can use another instead:

- **Google's ARM translator.** On the **Advanced** page, select **Get** beside **Google's ARM
  translator**, or run `dsa arm-translation fetch google`. DSA first shows where it comes
  from and Google's terms (the
  [Android SDK License](https://developer.android.com/studio/terms)), and downloads nothing
  until you accept. Your PC then downloads the Android emulator image Google offers for your
  Android version (about 2 GB), keeps only the translator and deletes the rest. DSA does not
  ship or pass on Google's translator.
- **A package you build or get elsewhere**, such as one made from the open source translator
  Digitalis: `dsa arm-translation install <folder>`.

Then restart Android: stop it on the **General** page of DSA Settings, then open an app.

To go back to the translator that comes with Android, select **Use** beside **Use the default
translator** on the **Advanced** page, or run `dsa arm-translation use-default`. It removes the
package you installed, from the next start of Android.

`dsa arm-translation status` says which translator Android uses. The **Run apps built only for
ARM** switch on the **Advanced** page turns translation on or off without removing anything.
For an app that has both ARM and x64 code, **Run as ARM** on the **Apps** page
(`dsa apps run-as-arm <package> on`) runs its ARM code instead. Only 64-bit ARM apps
(`arm64-v8a`) can run. On an ARM64 PC, Android runs ARM apps itself and needs no translator.

## Root access

Some apps, such as backup tools, need root. DSA gives it the way `sudo` does on a Linux PC:
only when an administrator of the PC approves, one request at a time. It is off by default,
and apps that are not allowed cannot see `su` at all.

1. An administrator turns on **Let Android apps ask for root** under **Root access** on the
   **Advanced** page of DSA Settings (`dsa settings set root-elevation on`). It applies from
   the next start of Android.
2. On the **Apps** page, set the app's **Root access**:
   - **Root: not allowed** (the default): the app has no `su`.
   - **Root: ask each time**: each time the app uses `su`, Windows shows its administrator
     prompt (UAC) under the app's own name and icon. A standard user enters an
     administrator's password. An approval lasts 15 minutes for that app.
   - **Root: remember (no asking)**: the app uses `su` without asking. Choosing this needs an
     administrator's approval.

   From the command line: `dsa apps root <package> off|ask|remember`.

With developer mode on, `adb shell` can use `su` too. It has its own row on the **Apps**
page, **adb shell**, and asks each time by default (`dsa apps root adb ask`). Its prompt is
named **adb shell** and shows the command.

When a part of Android itself asks, the prompt names it: **Android system**, **Android phone
services**, **Android Bluetooth**, **Android NFC** or **Android system service**. Apps that
share one user ID are approved as one group, and the prompt lists them all.

Every request is kept in the service log with the app, the time, the command and whether
it was approved. Turning the feature off hides `su` from every app at once, without a
restart.

---

Android is a trademark of Google LLC. Windows is a trademark of the Microsoft group of
companies. DSA is not affiliated with, endorsed by or sponsored by Google or Microsoft.
