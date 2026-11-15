# 应用与窗口

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · **简体中文** · [繁體中文](../zh-TW/apps.md) · [Español](../es/apps.md) · [Français](../fr/apps.md) · [Deutsch](../de/apps.md) · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · [Tiếng Việt](../vi/apps.md) · [Bahasa Indonesia](../id/apps.md) · [Türkçe](../tr/apps.md)

每个 Android 应用任务都作为单独的桌面窗口打开，并有自己的任务栏按钮。
没有包住手机屏幕的模拟器边框。
本页介绍 Windows 上的 Desktop Subsystem for Android™（DSA）。

## 打开应用

- **开始菜单。** 每个已安装的应用都有一个带有自己名称和图标的开始菜单项，
  因此 Windows 搜索可以找到它。
  如果 Android 未运行，打开应用时会先启动 Android。
- **DSA Settings。** 在 **Apps**（应用）页面上，选择应用旁边的 **Open**（打开）。
- **命令行。** `dsa apps list` 列出已安装的应用及其软件包名。
  `dsa launch <package>` 打开其中一个，例如 `dsa launch com.android.settings`。

如果其他程序的开始菜单项已经使用了该应用的名称，
DSA 会在名称后加上软件包名，例如 `Clock (com.example.clock)`。
DSA 只会更改或删除它自己创建的条目。

## 桌面快捷方式和跳转列表

应用图标的用法与 Windows 应用相同，而不是像 Android 主屏幕那样。

- **桌面快捷方式。** 在 DSA Settings 的 **Apps** 页面上，
  选择应用旁边的 **Create desktop shortcut**（创建桌面快捷方式）。
  也可以右键单击应用的任务栏按钮，然后选择 **Create desktop shortcut**。
  在命令行中：`dsa apps desktop-shortcut <package>`。
  该快捷方式会像开始菜单项一样打开应用。
- **应用快捷方式。** Android 应用提供的快捷方式（例如聊天应用的“新建聊天”），
  会在您右键单击应用的任务栏按钮或其开始菜单项时显示。
  选择其中一个，会在窗口中打开应用的对应界面；
  如果 Android 未运行，会先启动 Android。
  应用更改其快捷方式时，列表也会随之更新。
  在命令行中：`dsa apps shortcut <package> <id>`；
  `dsa apps info <package> --json` 会列出这些 ID。
- **应用请求添加的快捷方式。** 当应用请求把它的某个快捷方式添加到桌面时，
  DSA 会先询问您，只有在您同意后才会添加。

当应用更改名称时（例如更新之后），
其开始菜单项、桌面快捷方式和任务栏固定项都会使用新名称，并且仍然可用。
您删除的桌面快捷方式不会被重新创建。
卸载应用时，也会删除其桌面快捷方式。

## 使用应用窗口

- 每个任务都有自己的窗口、任务栏按钮和 Alt+Tab 条目，
  并显示应用的名称和图标。
  任务管理器会以应用自己的名称列出每个应用。
- 标题栏是应用的一部分：应用的颜色一直延伸到窗口顶部，
  Windows 11 的标题栏按钮位于其上。应用的图标和名称位于左侧。
  当应用可以返回时，图标会变成返回箭头；选择它即可返回。
- 贴靠布局（将指针悬停在最大化按钮上）、拖动标题栏、
  双击标题栏最大化以及实时调整大小都可以使用。
  调整大小不会重启应用。
  最大化的窗口会填满显示器的工作区，并保留任务栏可见。
- 没有 Android 导航栏。
  应用的底部栏紧贴窗口的下边缘，下方没有空白带。
- 鼠标（支持悬停）、触控、触控笔（压感、橡皮擦和笔杆按钮）和键盘都可以使用。
  文字来自您的 Windows 输入法，包括韩语和其他语言；
  组字内容直接显示在 Android 编辑框中，候选窗口位于光标旁边。
  没有 Android 键盘应用。
- 可以把文件从文件资源管理器拖到应用的窗口上，
  也可以把文件从应用中拖到文件资源管理器。
- 讲述人（Narrator）和 NVDA 等屏幕阅读器通过 UI Automation 朗读应用。
- 关闭窗口时，会请求 Android 结束该任务。

## 窗口菜单

右键单击标题栏、选择应用的图标，或按 Alt+Space。
在常见的 Windows 菜单项之后，菜单还包含：

- **Full screen**（全屏）：填满整个屏幕，与 F11 相同。
  再次选择它或按 F11 即可退出。
- **New window**（新窗口）：为支持多个窗口的应用再打开一个窗口。
- **Screenshot**（截图）：把应用的画面（不含标题栏）复制到剪贴板，
  并保存到“图片”中的“屏幕截图”文件夹。
- **Aspect ratio**（宽高比）：打开该应用的 Android 宽高比设置，
  适用于在大窗口中显得被拉伸的应用。
- **Open by default**（默认打开）：选择应用支持的链接是在应用中打开，还是在浏览器中打开。
- **App info**（应用信息）：在 DSA Settings 的 **Apps** 页面上打开该应用的条目。

## 全屏

当应用请求全屏时，其窗口会以以下两种方式之一显示：

- **整个屏幕**：窗口填满显示器，标题栏隐藏。
  用指针碰触屏幕顶部边缘即可显示标题栏，其中有最小化和关闭按钮；
  中间的按钮用于退出全屏。
- **窗口内**：窗口保持其大小和标题栏，由应用填满窗口。

应用的请求方式有两种，每种都有自己的默认值：

| 请求方式 | 示例 | 默认值 |
|---|---|---|
| 明确的全屏请求 | 视频播放器的全屏按钮 | 整个屏幕 |
| 只隐藏各种栏 | 短视频信息流 | 窗口内 |

可以在 DSA Settings 的 **Display & GPU**（显示与 GPU）页面上，
通过 **App full screen**（应用全屏）和
**Apps that hide their bars**（隐藏栏的应用）更改默认值；
也可以在 **Apps** 页面上，通过 **Full screen request**（全屏请求） 和 **When only the bars hide**（仅隐藏栏时）
为单个应用设置。
应用自己的选择优先。
无论如何设置，F11 和窗口菜单中的 **Full screen**
随时都可以在整个屏幕上切换全屏。

在命令行中：

```
dsa settings set app-fullscreen screen|window           # 明确请求的默认值
dsa settings set app-immersive screen|window            # 隐藏栏的默认值
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## 设置应用的窗口大小

在 DSA Settings 的 **Apps** 页面上：

- 关闭 **Resizable**（可调整大小），让应用的窗口保持打开时的大小。
- 选择应用窗口打开时的大小：由 Android 决定的大小，
  或者纵向、横向的手机或平板尺寸。

更改会应用到应用打开的下一个窗口。在命令行中：

```
dsa app-window <package> --fixed-size on        # 无法调整大小或最大化
dsa app-window <package> --start-size 1280x800  # 以此大小打开；auto 表示由 Android 决定
dsa app-window <package> --dpi-scaling system   # 在其他显示器上如何缩放；见下文
dsa app-window <package> reset                  # 恢复默认值
```

## 让应用从标题栏下方开始

应用会一直绘制到窗口顶端，也就是标题栏下面。因此有些应用的顶部栏会被挤压，或有一部分被标题栏遮挡。对于这样的应用，
请在 DSA Settings 的 **Apps** 页面打开 **Start below the title bar**（从标题栏下方开始）。应用的内容随后从标题栏下方开始，标题栏则采用应用在那里显示的颜色。
默认关闭，下次打开该应用的窗口时生效。在命令行中：

```
dsa app-window <package> --below-title-bar on   # off 恢复默认值
```

## 把应用移到缩放比例不同的显示器

应用从第一帧起就以其窗口所在显示器的缩放比例打开，
因此打开后不会马上被重新创建。
当您把窗口移到缩放比例不同的显示器时，应用在屏幕上的大小保持不变。
每个应用会采用以下方式之一，
您可以在 **Apps** 页面的 **Scaling**（缩放）中选择，
或使用 `dsa app-window <package> --dpi-scaling`：

- **Automatic**（自动，`auto`，默认值）：
  对 DSA 内置列表中在运行时更改缩放比例会出问题的应用使用 **System**，
  对其他所有应用使用 **App**。
  该列表中有 YouTube：它会保持启动时缩放比例下的布局，
  所以移动后，其底部选项卡和迷你播放器会一直错乱，直到重启为止。
  对于这类应用，Settings 会把默认值显示为
  **Automatic (system, recommended for this app)**。
  您为该应用做出的选择优先。
- **App**（应用，`app`）：拖动窗口时，DSA 会把它缩放到新的显示器。
  松开后，应用会以新显示器的缩放比例重新绘制，文字保持清晰：
  Android 会重新创建一次其界面并恢复状态（滚动位置、已输入的文字），
  所以每次移动后会短暂重绘，
  就像 Windows 程序使用“应用程序”缩放时那样。
  即使应用声明自己能处理缩放变化，也会这样处理，
  因为其中有些应用只会以新的缩放比例重绘部分界面。
- **System**（系统，`system`）：应用保持其缩放比例，
  由 DSA 把窗口缩放到新的显示器。应用永远不会被重新创建。
  放大后，文字可能看起来略微模糊。
- **System (forced)**（系统（强制），`system-forced`）：
  在不重启的情况下把新的缩放比例交给应用，即使它没有声明能够处理。
  目前尚不可用：它需要对 Android 做一项本版本尚未包含的修改，
  在此之前它的效果与 **System** 相同。

应用中的文字跟随 Windows 的文本大小（“设置 > 辅助功能 > 文本大小”）。

## 通知

Android 通知以 Windows 通知的形式显示，并带有应用的名称和图标。

- 选择通知即可打开应用。
- 关闭通知时，Android 中的通知也会被关闭，除非应用让它保持显示。
- Android 处于 Doze 时，通知仍会到达。请参见[内存](memory.md)。

## 在后台运行的应用

窗口关闭后仍继续运行的应用（例如音乐播放器）会显示为通知区域中的图标。
其工具提示就是该应用的通知。

- 选择图标即可打开应用。
- 右键单击图标，可以使用 **Open**、通知自身的操作以及 **Stop**（停止）。
- **Stop** 会先询问。
  勾选 **Do not ask again**（不再询问）后，以后将不再询问。
  如果想重新开始询问，请在 DSA Settings 的
  **Running in the background**（在后台运行）页面上开启
  **Ask before stopping an app from the notification area**
  （从通知区域停止应用前询问）。
- 播放媒体的应用还支持媒体键和 Windows 媒体控件。

DSA Settings 的 **Running in the background** 页面会列出这些应用，
每个应用都有 **Open** 和 **Stop**。
在命令行中：`dsa background`、`dsa background open <package>`
和 `dsa background stop <package>`。

## 桌面上的小部件

Android 小部件可以放在桌面上，并拥有自己的窗口。
它始终位于其他窗口下方，显示桌面时仍然保留，
并且不会出现在任务栏或 Alt+Tab 中。

1. 打开 DSA Settings 的 **Widgets**（小部件）页面。
2. 在 **Widgets your apps offer**（您的应用提供的小部件）下，
   选择小部件旁边的 **Add to desktop**（添加到桌面）。
   如果应用要求您设置小部件，其设置界面会在窗口中打开。
3. 要移动小部件，请在其上任意位置按住并保持不动半秒钟。
   它会浮起，并显示边框，在可以调整大小的每条边上都有一个控制柄。
   拖动它即可移动，拖动控制柄即可调整大小。
   快速拖动仍由小部件自己处理，所以其中的列表照样可以滚动。
   您也可以直接拖动小部件的边缘来调整大小，无需先让它浮起；
   只有可以调整大小的边缘才支持这样做。
   DSA 会记住您放置它的位置和大小。
4. 右键单击小部件，或在长按后松开，
   即可使用 **Widget settings**（小部件设置，在小部件可以重新设置时）、
   **Resize**（调整大小，从它允许的尺寸中选择一种）
   和 **Remove**（删除）。
   菜单键和 Shift+F10 也可以打开此菜单。
   在此菜单中可以使用方向键、Enter 和 Esc，
   按字母可以选择菜单项：R 表示 **Resize**，M 表示 **Remove**。

在命令行中：`dsa widgets list`、`dsa widgets add <provider>`
和 `dsa widgets remove <id>`。

## 链接与剪贴板

- 在 Android 应用中点按的网页链接会在您的默认浏览器中打开。
  已安装的应用验证为自己所有的链接会在该应用中打开。
- 如果想改为在 Android 应用中打开所有网页链接，
  请在 DSA Settings 的 **Advanced**（高级）页面上开启
  **Open web links in Android apps**（在 Android 应用中打开网页链接）
  （`dsa settings set links-in-android on`）。
- 您复制的文本会在 Windows 和 Android 之间双向共享。
  如果想停止共享，请在 **Advanced** 页面上关闭
  **Share the clipboard with Android**（与 Android 共享剪贴板）
  （`dsa settings set clipboard off`）。

## Google Play 与受保护的视频

DSA 不附带任何 Google 软件。
如果您需要，DSA 会先向您显示来源和条款，然后替您从其来源下载。
两者都从 Android 下一次启动时生效，也都可以再次删除。

- **Google Play。** 在 DSA Settings 的 **Advanced** 页面上，
  选择 **Google Play** 下的 **Install**，或运行 `dsa gapps install`。
  DSA 会下载适用于您 Android 版本的公开 LiteGapps Core 软件包
  （Google Play 服务、Play 商店和账号同步）。
  此 Android 未经 Google 认证，因此在您注册设备之前，Google 可能会拒绝您登录：
  **Register this device with Google**（向 Google 注册此设备）
  会显示 Android ID 并打开注册页面。
  `dsa gapps status` 显示状态，
  `dsa gapps remove` 会删除 Google 的应用，即使 Android 已停止也可以。
- **Widevine。** 流媒体应用需要 Google 的 Widevine 模块才能播放受保护的视频。
  在 **Advanced** 页面上，选择 **Protected content**（受保护的内容）下的 **Get**（获取），
  或运行 `dsa drm widevine install`。
  DSA 会下载适用于您 Android 版本的 Google Android 模拟器映像（约 2 GB），
  只原样保留 Widevine 模块，并删除其余部分。
  这里的 Widevine 为 L3 安全级别，因此流媒体服务最多只提供标清画质；
  如需高清，请使用 Windows 浏览器或应用。
  `dsa drm widevine remove` 会将其删除。

## 仅为 ARM 构建的应用

在 x64 电脑上，只包含 ARM 处理器代码的应用会通过 ARM 转译器运行。
x64 电脑上的 Android 自带一个转译器，即开源转译器 Digitalis，
并从第一次启动起就使用它。
您也可以改用其他转译器：

- **Google 的 ARM 转译器。** 在 **Advanced** 页面上，
  选择 **Google's ARM translator**（Google 的 ARM 转译器）旁边的 **Get**，
  或运行 `dsa arm-translation fetch google`。
  DSA 会先显示它的来源和 Google 的条款
  （[Android SDK License](https://developer.android.com/studio/terms)），
  在您接受之前不会下载任何内容。
  接受后，您的电脑会下载 Google 为您的 Android 版本提供的 Android 模拟器映像（约 2 GB），
  只保留转译器，并删除其余部分。
  DSA 不会附带或转发 Google 的转译器。
- **您自己构建或从别处获得的软件包**，
  例如基于开源转译器 Digitalis 制作的软件包：
  `dsa arm-translation install <folder>`。

然后重启 Android：在 DSA Settings 的 **General**（常规）页面上停止它，然后打开一个应用。

如果想回到 Android 自带的转译器，
请在 **Advanced** 页面上选择 **Use the default translator**（使用默认转译器）旁边的 **Use**，
或运行 `dsa arm-translation use-default`。
它会从 Android 下一次启动起删除您安装的软件包。

`dsa arm-translation status` 会说明 Android 使用的是哪个转译器。
**Advanced** 页面上的 **Run apps built only for ARM**（运行仅为 ARM 构建的应用）开关
可以开启或关闭转译，而不会删除任何内容。
对于同时包含 ARM 和 x64 代码的应用，
**Apps** 页面上的 **Run as ARM**（以 ARM 运行）
（`dsa apps run-as-arm <package> on`）会改为运行其 ARM 代码。
只有 64 位 ARM 应用（`arm64-v8a`）可以运行。
在 ARM64 电脑上，Android 本身就能运行 ARM 应用，不需要转译器。

## root 访问权限

有些应用（例如备份工具）需要 root。
DSA 以类似 Linux 电脑上 `sudo` 的方式提供 root：
只有在电脑的管理员批准时才会授予，并且每次只处理一个请求。
它默认关闭，未获允许的应用完全看不到 `su`。

1. 管理员在 DSA Settings 的 **Advanced** 页面上，
   于 **Root access**（root 访问权限）下开启
   **Let Android apps ask for root**（允许 Android 应用请求 root）
   （`dsa settings set root-elevation on`）。
   它从 Android 下一次启动时生效。
2. 在 **Apps** 页面上设置应用的 **Root access**：
   - **Root: not allowed**（不允许，默认值）：应用没有 `su`。
   - **Root: ask each time**（每次询问）：每次应用使用 `su` 时，
     Windows 都会以该应用自己的名称和图标显示其管理员提示（UAC）。
     标准用户需要输入管理员的密码。
     批准对该应用有效 15 分钟。
   - **Root: remember (no asking)**（记住，不再询问）：应用使用 `su` 时不会询问。
     选择此项需要管理员批准。

   在命令行中：`dsa apps root <package> off|ask|remember`。

开启开发者模式后，`adb shell` 也可以使用 `su`。
它在 **Apps** 页面上有自己的一行 **adb shell**，
默认每次询问（`dsa apps root adb ask`）。
它的提示名为 **adb shell**，并显示要执行的命令。

当 Android 自身的某个部分发出请求时，提示会显示它的名称：
**Android system**、**Android phone services**、**Android Bluetooth**、
**Android NFC** 或 **Android system service**。
共享同一个用户 ID 的应用会作为一组一起批准，提示中会列出全部应用。

每个请求都会记录在服务日志中，包括应用、时间、命令以及是否获得批准。
关闭此功能后，所有应用都会立即看不到 `su`，无需重启。

---

Android 是 Google LLC 的商标。
Windows 是 Microsoft 集团公司的商标。
DSA 与 Google 或 Microsoft 没有关联，也未获得二者的认可或赞助。
