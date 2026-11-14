# Android 版本

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · **简体中文** · [繁體中文](../zh-TW/android-versions.md) · [Español](../es/android-versions.md) · [Français](../fr/android-versions.md) · [Deutsch](../de/android-versions.md) · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · [Tiếng Việt](../vi/android-versions.md) · [Bahasa Indonesia](../id/android-versions.md) · [Türkçe](../tr/android-versions.md)

Desktop Subsystem for Android™（DSA）支持最新的两个 Android 主版本：
目前是 Android 16 和 Android 17。默认运行其中最新的一个。
有些应用只能在其中一个版本上运行，所以您可以自行选择。

## 查看运行的版本

打开 DSA Settings 的 **Updates**（更新）页面。
在 **Android** 下，会显示版本、构建号、安全补丁级别、所跟随的版本，
以及更新的状态和下载进度。
**Check now**（立即检查）会让服务立即查找更新。也可以运行：

```
dsa android version   # 正在运行的版本，以及可以选择的版本
```

## 更改版本

电脑上的所有用户都运行同一个 Android 版本，因此只有管理员才能更改它。

1. 打开 DSA Settings 的 **Updates** 页面。
2. 在 **Version to use**（要使用的版本）下，选择一个版本，
   或选择 **Always the newest**（始终使用最新版本）。
3. 选择 **Change**（更改），并批准管理员提示。

所选版本会被下载，并从 Android 下一次启动时开始使用。
在管理员终端中：

```
dsa android use 16
dsa android use latest
```

### 切换到较旧的版本会清除数据

Android 无法读取较新版本写入的数据。
因此，切换到较旧的主版本会清除电脑上每个用户的 Android 数据：
他们安装的应用、他们的账号以及所有应用数据。
DSA 的设置会保留。此操作无法撤销。

DSA Settings 会要求您用 **Erase data and change**（清除数据并更改）确认。
在命令行中，请加上 `--erase-data`：

```
dsa android use 16 --erase-data
```

切换到较新的版本会保留您的应用和数据。

## 更新

- 所选版本会持续获得安全更新。
- 当某个版本不再受支持时，DSA 会切换到下一个版本，并保留您的应用。
- DSA 绝不会自行切换到较旧的主版本。
- 每个映像在使用前都会根据一份签名列表进行检查。
  无法启动的映像会被替换为上一个映像。

管理员可以在 DSA Settings 的 **Updates** 页面上，
通过 **Update the Android image automatically**（自动更新 Android 映像）关闭映像更新，
也可以使用 `dsa settings set image-updates off`。

---

Android 是 Google LLC 的商标。
Windows 是 Microsoft 集团公司的商标。
DSA 与 Google 或 Microsoft 没有关联，也未获得二者的认可或赞助。
