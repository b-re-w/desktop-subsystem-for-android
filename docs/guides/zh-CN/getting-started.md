# 入门

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · **简体中文** · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™（DSA）在一个带有自己内核的轻量级虚拟机中运行 Android。
每个 Android 应用都在桌面上的单独窗口中打开。

## 各平台的状态

| 主机 | 状态 |
|---|---|
| Windows 11，x64 | 已发布。 |
| Windows 11，ARM64 | 已发布。Android 同样是 ARM64，因此 ARM 应用无需转译即可运行。 |
| Linux，x64 和 ARM64 | 开发中。`.deb` 和 `.rpm` 软件包已在 CI 中构建；尚未发布。 |
| macOS，Apple 芯片 | 开发中。`DSA.app` 及其 `.pkg` 和 `.dmg` 已在 CI 中构建；尚未发布。 |

## 在 Windows 上安装

您需要一台运行 Windows 11 的 x64 或 ARM64 电脑，
并开启 Windows 功能**虚拟机平台**（Virtual Machine Platform）。

1. 开启虚拟机平台。打开**启用或关闭 Windows 功能**，
   勾选**虚拟机平台**，选择**确定**，然后重启电脑。
   也可以在管理员终端中运行
   `dism /online /enable-feature /featurename:VirtualMachinePlatform /all`。
   安装程序不会替您开启它：如果该功能未开启，安装程序会显示提示并停止。
2. 从[最新版本](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest)
   下载安装程序和 `SHA256SUMS.txt`：
   x64 电脑下载 `DSA-v<version>-win-x64-setup.exe`，
   ARM64 电脑下载 `DSA-v<version>-win-arm64-setup.exe`。
3. 校验下载的文件。下面命令输出的哈希值必须与 `SHA256SUMS.txt` 中对应的行一致：

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. 运行安装程序，选择 **Install**（安装），并批准管理员提示。
5. 等待第一个 Android 映像。
   如果安装程序未包含映像，DSA 服务会在后台下载。
   `dsa status` 会显示下载进度。
6. 从开始菜单打开一个应用，或在终端中检查：

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## 安装程序会安装什么

- `DsaHost` 服务，用于运行每个用户的 Android。
- `dsa` 命令。
- 开始菜单中的 **DSA Settings**（DSA 设置）。
- 文件资源管理器导航窗格中的 **Android**，用于打开 Android 的文件。
- APK Installer（APK 安装程序），用于打开 `.apk` 文件。
- DSA 窗口管理器（`dsa-wm`）。它在您登录时启动，并随即启动 Android，
  所以您打开的第一个应用无需等待 Android 启动。

每个 Windows 用户都有自己的 Android 和自己的数据磁盘
`%LOCALAPPDATA%\DSA\android.vhdx`。其他用户无法读取它。

DSA 使用自己的内核和自己的虚拟机。
它不会使用或更改您的 WSL 设置、内核或发行版。

## 更新

DSA 会更新自身和 Android 映像。
两者都经过签名，签名、大小或哈希值不匹配的内容，DSA 一律不会应用。

- **DSA 本身**会在没有任何用户的 Android 运行时更新。
  在此之前更新会等待；`dsa update` 会立即安装。
  DSA 绝不会降到更低的版本。
- **发布渠道。** 从 1.0 起，DSA 有稳定渠道，
  以及用于尚在完善中的版本的预发布渠道。
  您的渠道取决于您使用的安装程序，升级时保持不变。
  管理员可以在 DSA Settings 的 **Updates**（更新）页面上用
  **Get pre-release versions**（获取预发布版本）更改渠道。
  1.0 之前的每个版本都是预发布版本。
- **Android 映像**与渠道无关。
  如果某个映像无法启动，DSA 会回到上一个映像，`dsa status` 会提示这一点。

管理员可以在 DSA Settings 的 **Updates** 页面上关闭更新。
所有开关请参见[设置](settings.md)。

## Linux（开发中，尚未发布）

Linux 版 DSA 构建为 `.deb`（Debian、Ubuntu）和 `.rpm`（Fedora、openSUSE）。
它需要 KVM（`/dev/kvm`）和 Wayland 桌面。
Android 在 crosvm 虚拟机中运行。

安装软件包时还会添加 DSA 的签名软件包仓库，
因此以后的版本会通过 `apt`、`dnf`、`zypper` 或软件中心送达。
DSA 在 Linux 上没有自己的更新程序。

- Debian 和 Ubuntu：`/etc/apt/sources.list.d/dsa.sources`，
  密钥位于 `/usr/share/keyrings/dsa-archive-keyring.gpg`。
- Fedora 和 openSUSE：`/etc/yum.repos.d/dsa.repo`
  （存在 zypper 时还有 `/etc/zypp/repos.d/dsa.repo`），
  密钥位于 `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`。

该仓库计划从
`https://thisisthepy.github.io/desktop-subsystem-for-android/` 提供。
目前尚未发布。

发布后，首次安装的方式如下：

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian、Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

软件包会安装 `dsa-hostd` 系统服务、
`dsa-devices` 和 `dsa-compositor` 用户服务、
位于 `/usr/bin` 的 `dsa` 命令、**DSA Files** 菜单项，
以及文件操作 **Open in an Android app**（在 Android 应用中打开）、
**Share with an Android app**（分享给 Android 应用）和
**Install in Android**（安装到 Android）。

## macOS（开发中，尚未发布）

macOS 版 DSA 在 Apple 芯片上通过 Apple 的 Virtualization 框架运行 Android。
它构建为 `DSA.app`，并提供安装包（`DSA-<version>.pkg`）
和磁盘映像（`DSA-<version>.dmg`）。
CI 构建使用临时签名（ad hoc），因此只能在构建它们的机器上运行。
目前还没有使用 Developer ID 签名的版本。

## 下一步

- [应用与窗口](apps.md)
- [安装 APK](install-apks.md)

---

Android 是 Google LLC 的商标。
Windows 是 Microsoft 集团公司的商标。
macOS 是 Apple Inc. 的商标。
DSA 与 Google、Microsoft 或 Apple 没有关联，也未获得它们的认可或赞助。
