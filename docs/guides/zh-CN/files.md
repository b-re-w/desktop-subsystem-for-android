# 文件

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · **简体中文** · [繁體中文](../zh-TW/files.md) · [Español](../es/files.md) · [Français](../fr/files.md) · [Deutsch](../de/files.md) · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · [Italiano](../it/files.md) · [Tiếng Việt](../vi/files.md) · [Bahasa Indonesia](../id/files.md) · [Türkçe](../tr/files.md)

使用 Desktop Subsystem for Android™（DSA），
您可以在文件资源管理器中打开 Android 的文件，
与 Android 应用共享您的 Windows 文件夹，
并把文件发送给 Android 应用。
本页介绍 Windows 上的 DSA。

## 在文件资源管理器中打开 Android 的文件

可以使用以下任一方式。如果 Android 未运行，每种方式都会启动它。

- 文件资源管理器导航窗格中的 **Android**，位于 WSL 的 **Linux** 正下方。
- DSA Settings 的 **Advanced**（高级）页面上的 **Android files**（Android 文件）。
- `dsa files open`。`dsa files` 会说明存储的位置，但不会打开它。

您看到的是 Android 的共享存储：Android 应用保存的文件，例如照片和下载内容。
您可以像在任何文件夹中一样复制文件进出。
文件资源管理器通过 Windows 的 WebDAV 客户端访问该存储。
如果存储没有响应，DSA 会显示错误及其原因。

## 与 Android 应用共享您的 Windows 文件夹

您的“文档”、“下载”、“图片”、“音乐”、“视频”和“桌面”文件夹
会显示在 Android 共享存储的 `Host` 下，例如 `Host/Pictures`。
Android 应用可以在那里打开和保存文件。
应用需要先获得 Android 的存储权限，
并且 Windows 只允许它做您自己的账户能做的事。

共享默认开启。如需更改，请打开 DSA Settings 的 **Advanced** 页面：

- 关闭 **Share Windows folders with Android**（与 Android 共享 Windows 文件夹），
  即不共享任何文件夹。
- 在 **Shared folders**（共享文件夹）下，取消勾选某个文件夹即可停止共享它。
  它会立即从 Android 中消失。

在命令行中：

```
dsa settings set shared-folders Documents,Pictures   # 只共享这些；也可以是 all 或 none
dsa settings set file-sharing off                     # 不共享任何文件夹
```

早期版本使用名为 `Windows` 的文件夹，而不是 `Host`。
旧文件夹变空后，Android 会将其删除，所以您保存在那里的文件会保留下来。

## 把文件发送给 Android 应用

- **拖放。** 把文件从文件资源管理器拖到应用的窗口上。
  文件会发送给该应用。APK 则会改为交给 APK Installer。
  文件夹不会被发送。
- **上下文菜单。** 右键单击文件，
  然后选择 **Open in an Android app**（在 Android 应用中打开）
  或 **Share with an Android app**（分享给 Android 应用）。
  在 Windows 11 上，它们位于“显示更多选项”下。
  然后 Android 会询问由哪个应用接收该文件。
- **命令行。** `dsa open <file>` 和 `dsa share <file>`。

这些方式可以发送最大 64 MiB 的文件。
更大的文件会在发送任何内容之前被拒绝。
如果要把更大的文件交给应用，请改为把它放在共享文件夹中。

## 从 Android 获取文件

- 在 Android 中把文件保存到 `Host/<folder>` 下。
  这样它就会出现在电脑上的对应文件夹中。
- 或者在文件资源管理器中打开 **Android**，把文件复制出来。

---

Android 是 Google LLC 的商标。
Windows 是 Microsoft 集团公司的商标。
DSA 与 Google 或 Microsoft 没有关联，也未获得二者的认可或赞助。
