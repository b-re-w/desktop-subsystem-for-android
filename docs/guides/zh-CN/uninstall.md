# 卸载

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · **简体中文** · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

可以用以下任一方式卸载 Desktop Subsystem for Android™（DSA）：

- Windows 的“**设置** > **应用** > **已安装的应用**”：
  找到 Desktop Subsystem for Android，然后选择**卸载**。
- 再次运行安装程序，然后选择 **Uninstall**（卸载）。

卸载会为电脑上的每个用户（无论是否已登录）删除以下内容：

- 程序、`DsaHost` 服务、`dsa` 命令、DSA Settings 和 APK Installer；
- 已下载的 Android 映像和更新；
- Android 应用的开始菜单项，以及它们在**已安装的应用**中的条目；
- 文件资源管理器中的 **Android**、`.apk` 文件关联以及上下文菜单项；
- DSA 的设置和日志。

卸载会**保留**每个用户的 Android 磁盘 `%LOCALAPPDATA%\DSA\android.vhdx`，
这样以后重新安装时，您的应用和数据仍然都在。
如果也想删除您的数据，请在卸载前运行 `dsa reset-data --yes`，
或在卸载后删除该文件。

如果文件夹重定向移动了某个用户的开始菜单，
该用户的 Android 应用条目会保留下来，
因为卸载程序只在默认位置查找。请手动删除它们。

您的 WSL 设置不会受到影响，因为 DSA 从不更改它。

## Linux（尚未发布）

```
sudo apt remove desktop-subsystem-for-android    # Debian、Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

删除软件包时，也会删除它添加的软件包仓库条目。

---

Android 是 Google LLC 的商标。
Windows 是 Microsoft 集团公司的商标。
DSA 与 Google 或 Microsoft 没有关联，也未获得二者的认可或赞助。
