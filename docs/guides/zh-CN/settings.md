# 设置

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · **简体中文** · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings**（DSA 设置）是 Windows 上 Desktop Subsystem for Android™（DSA）的设置应用。
请从开始菜单打开它。
它根据您的 Windows 显示语言以英语或韩语显示，并跟随您的 Windows 主题。
它支持键盘和屏幕阅读器操作。

DSA Settings 所做的每项更改都会通过 `dsa` 命令进行，
所以命令行看到的值与它相同：

```
dsa settings                        # 所有设置及其值
dsa settings set <name> <value>     # 更改一项设置；开关使用 on 或 off
```

有些设置适用于电脑上的所有用户，因此只有管理员才能更改。
DSA Settings 会用盾牌图标标出这些设置，
并在您更改其中一项时请求管理员批准。
下文用“(admin)”标出它们。

## 页面

| 页面 | 包含的内容 | 命令行 |
|---|---|---|
| **General**（常规） | Android 的状态、**Start**、**Stop**、**Freeze Android now**、最近一次出现的问题、**Open my log folder**、**Refresh** | `dsa status`、`dsa start`、`dsa stop`、`dsa freeze` |
| **Memory**（内存） | **Memory limit**、Android 虚拟机的大小、各部分的使用量 | `memory-limit`、`dsa memory` |
| **Apps**（应用） | 已安装的应用：**Open**、**App info**、**Uninstall**、**Create desktop shortcut**。每个应用的设置：**Resizable**、窗口打开时的大小、**Scaling on another monitor**、**Full screen**、**Bars hidden**、**Run as ARM**、**Root access**。还有一行用于 **adb shell** 的 root 访问权限 | `dsa apps`、`dsa app-window`、`dsa apps run-as-arm`、`dsa apps root` |
| **Running in the background**（在后台运行） | 没有窗口但仍在运行的应用：**Open**、**Stop**；停止前是否询问 | `dsa background`、`stop-without-asking` |
| **Widgets**（小部件） | 桌面上的小部件，以及 **Add to desktop** | `dsa widgets` |
| **Display & GPU**（显示与 GPU） | **Render GPU**、**GPU of the primary monitor**、**App full screen**、**Apps that hide their bars**、**Turn off Windows touch gestures in DSA app windows**（仅限 Windows 11，默认开启）、**Restart the window manager** | `render-gpu`、`dsa gpus`、`app-fullscreen`、`app-immersive`、`touch-gestures-off` |
| **Device identity**（设备标识） | 首先是这台电脑本身（只读）：其名称、制造商和型号，以及带 **Copy** 的 **Device specifications**。然后是 Android 向应用报告的制造商、型号和构建指纹 | `dsa settings device`、`device.<field>` |
| **Network**（网络） | Android 如何连接网络、其地址、由谁解析其域名（DNS）及其网关、**Advanced networking**、**Use this PC's added root certificates**、**Developer mode**（adb）、**Connect adb automatically** | `dsa network`、`advanced-networking`、`host-certificates`、`developer-mode`、`adb-port`、`adb-auto-connect` |
| **Updates**（更新） | **Android**：正在运行的版本和构建号、其安全补丁级别、所跟随的版本、Android 更新及其进度和 **Check now**（admin）、**Restart Android into updates by itself**、**Update the Android image automatically**（admin）、**Version to use**（admin）。**DSA**：**Update DSA automatically**（admin）、**Get pre-release versions**（admin）、**Install the newest version of DSA now** | `dsa android version`、`dsa android use`、`image-auto-restart`、`image-updates`、`program-updates`、`prerelease-updates`、`dsa update` |
| **Advanced**（高级） | 剪贴板、文件、网页链接、空闲时冻结、备份、重置、ARM 应用、Google Play、受保护的内容、root 访问权限、跟踪（admin）、日志文件夹 | 见下文 |
| **Logs**（日志） | Android 的实时日志、**Save diagnostics** | `dsa logcat`、`dsa logs collect` |
| **About**（关于） | 版本、许可证、源代码 | |

## Advanced 页面

| 设置 | 默认值 | 命令行 |
|---|---|---|
| **Share the clipboard with Android** | 开 | `clipboard` |
| **Share Windows folders with Android** | 开 | `file-sharing` |
| **Shared folders** | 全部六个 | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`、`dsa share` |
| **Freeze Android when idle** | 关 | `freeze` |
| **Open web links in Android apps** | 关 | `links-in-android` |
| **Back up apps' data to this PC**、**Back up now**、**Restore** | 关 | `dsa backup on`、`dsa backup now`、`dsa backup restore` |
| **Recovery key**、**Use a recovery key** | | `dsa backup key show`、`dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator**（ARM64 上没有） | | `dsa arm-translation fetch google` |
| **Use the default translator**（ARM64 上没有） | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**、**Remove Google Play** | 未安装 | `dsa gapps install`、`dsa gapps remove` |
| **Widevine**（位于 **Protected content** 下） | 未安装 | `dsa drm widevine install`、`dsa drm widevine remove` |
| **Let Android apps ask for root**（admin） | 关 | `root-elevation` |
| **Input trace**（admin） | 关 | `input-trace` |
| **Resize trace**（admin） | 关 | `resize-trace` |

备份会在您的用户文件夹中保存一份加密的应用数据副本，
所用的密钥由 Windows 替您保管。
要在另一台电脑上恢复，或在重装 Windows 后恢复，您需要恢复密钥，
所以请把它记下来，并保存在电脑之外的地方。
Google Play、Widevine 和 root 访问权限在[应用与窗口](apps.md)中介绍。

## 更改何时生效

- **Render GPU**（渲染 GPU）：在窗口管理器重新启动时生效，
  即您下次登录时，或您选择 **Restart the window manager**（重启窗口管理器）时。
- **Memory limit**、**Device identity**、**Advanced networking**、
  **Run apps built only for ARM**、Google Play、Widevine、开启 root 访问权限，
  以及 Android 版本：在 Android 下一次启动时生效。
  要重启 Android，请在 **General** 页面上停止它，然后打开一个应用。
- 应用的窗口设置：从该应用打开的下一个窗口起生效。
- 其他所有设置（包括关闭 root 访问权限）：立即生效。

## Android 如何解析域名

Android 应用查询的每个域名都由您的电脑解析，
使用的解析器与 Windows 应用相同（在 Linux 和 macOS 上则是系统的解析器）。
因此，您的 hosts 文件、公司的域名规则、VPN 自己的 DNS 服务器以及 DNS 过滤，
对 Android 应用的作用与对电脑本身完全一样。
Android 自己不会向 DNS 服务器查询。

- 开启 **Advanced networking**（高级网络）时也是如此：
  您的电脑位于同一网络中并知道这些域名，
  电脑上的 VPN 或 DNS 过滤仍然有效。
- Android 自己的**私人 DNS**（Private DNS）默认关闭，
  因为它会绕过您的电脑去查询 DNS 服务器。
  如果您在 Android 的设置中选择了私人 DNS 服务器，Android 就会使用它。
- 如果 DSA 暂时无法解析，Android 会改为查询网络的 DNS 服务器，直到 DSA 恢复为止。
- **Network** 页面上的 **DNS** 行以及 `dsa network` 会说明当前由谁解析：
  您电脑的解析器，或网络的 DNS 服务器（会列出这些服务器）。

## 命令行接受的值

- `render-gpu`：`display`（主显示器的 GPU，默认值）、`integrated`、
  `discrete`，或 `dsa gpus` 列出的 GPU 名称中的一部分。
- `memory-limit`：`auto`（默认值：没有固定上限，
  最多可用到电脑内存减去 Windows 保留的部分），
  或一个大小，例如 `4g` 或 `3072`（MiB）。
- `shared-folders`：`all`、`none`，或用逗号分隔的名称，例如
  `Documents,Pictures`。
- `adb-port`：1 到 65535 之间的 TCP 端口。默认值为 58526。
- `app-fullscreen`：`screen`（默认值）或 `window`，用于应用明确的全屏请求。
  `app-immersive`：`screen` 或 `window`（默认值），用于只隐藏自身栏的应用。
  请参见[全屏](apps.md#全屏)。
- `touch-gestures-off`：`on`（默认值）会在 DSA 应用窗口位于前台时
  关闭 Windows 11 的三指和四指触控手势，使应用最多可以接收十指触控，
  并在其他窗口切换到前台时恢复您自己的设置。
  如果 Windows 拒绝，则无法使用此设置，设置应用会说明原因；
  您仍然可以在 Windows 设置的“蓝牙和其他设备 > 触摸”中为所有应用关闭这些手势。
- `host-certificates`：`on`（默认值）允许 Android 应用信任添加到这台电脑上的根证书，
  例如公司或安全软件的根证书。
- `adb-auto-connect`：`on`（默认值）会在开发者模式开启时，
  让 Android 无需 `adb connect` 就显示在 `adb devices` 和 Android Studio 中。
- `image-auto-restart`：`on`（默认值）会在没有打开任何应用窗口时，
  立即重启 Android 并进入已下载的映像更新。
  否则，DSA 会先询问，或等待您关闭这些窗口。
- `prerelease-updates`：`on` 会同时获取预发布版本和稳定版本，
  `off` 只获取稳定版本。
  它的初始值取决于您使用的安装程序：
  预发布版安装程序会将其开启，稳定版安装程序则保持关闭，
  升级时会保留您的选择。
  1.0 之前的每个版本都是预发布版本。
  关闭它绝不会让 DSA 回退到旧版本；
  DSA 会等待比已安装版本更新的稳定版本。
- `root-elevation`：`on` 允许您在 **Apps** 页面上允许的应用请求 root。
  每个应用的级别用 `dsa apps root <package> off|ask|remember` 设置。
- `device.<field>`：`manufacturer`、`brand`、`model`、`device`、`name`、
  `characteristics`、`fingerprint`，或指纹的一部分，例如 `fingerprint.release`。
  `dsa settings reset device` 会恢复所有值。

---

Android 是 Google LLC 的商标。
Windows 是 Microsoft 集团公司的商标。
DSA 与 Google 或 Microsoft 没有关联，也未获得二者的认可或赞助。
