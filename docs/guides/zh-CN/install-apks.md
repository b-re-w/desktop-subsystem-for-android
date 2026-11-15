# 安装 APK

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · **简体中文** · [繁體中文](../zh-TW/install-apks.md) · [Español](../es/install-apks.md) · [Français](../fr/install-apks.md) · [Deutsch](../de/install-apks.md) · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · [Tiếng Việt](../vi/install-apks.md) · [Bahasa Indonesia](../id/install-apks.md) · [Türkçe](../tr/install-apks.md)

Desktop Subsystem for Android™（DSA）不附带应用商店。
您可以使用 APK Installer 或 `dsa` 命令，从 APK 文件安装应用。
本页介绍 Windows 上的 DSA。

## 使用 APK Installer 安装

1. 在文件资源管理器中双击一个 `.apk` 文件。
   它会在 **APK Installer**（APK 安装程序）中打开。
2. 确认该 APK 的信息。窗口会在不启动 Android 的情况下显示：
   - 应用的名称、图标、软件包名和版本；
   - 发布者，取自签名证书，因为 APK 本身不注明发布者；
   - 来源：下载该文件的主机，或文件所在的文件夹；
   - 在 **Capabilities**（功能）下，它请求的前几项权限。
     **More**（更多）会显示全部权限、它面向的 Android 版本、
     大小、处理器类型以及证书的 SHA-256。

   如果 APK 只包含其他处理器类型的代码，窗口会提醒您。
3. 选择 **Install**（安装）。
   如果应用已经安装，按钮会显示为 **Update**（更新）或 **Install again**（重新安装）。
   如果 Android 未运行，会先启动 Android。
4. 勾选 **Launch when ready**（就绪后启动，默认勾选）时，
   应用安装完成后会自动打开，安装程序随之关闭。
   否则，请选择 **Launch**（启动），或稍后从开始菜单打开应用。

Android 运行时，窗口还会说明该文件是全新安装、更新、相同版本的重新安装，
还是 Android 会拒绝的内容：旧版本，或使用其他密钥签名的同一应用。

您也可以把 APK 文件拖放到 APK Installer、DSA Settings 或应用的窗口上。
每个 APK 都会在单独的安装程序窗口中打开。

## 从命令行安装

```
dsa apk-info app.apk            # APK 的信息；读取文件而不启动 Android
dsa install app.apk             # 安装；Android 未运行时会启动它
dsa install app.apk --progress  # 同时输出每个步骤
```

当 Android 拒绝该 APK 时，`dsa install` 以代码 1 退出，
其消息以 `not installed (<reason>):` 开头。

## 安装被拒绝的原因

| 原因 | 含义 | 处理方法 |
|---|---|---|
| `incompatible` | 应用需要其他处理器类型、更新的 Android 版本，或此设备缺少的功能 | 请参见[仅为 ARM 构建的应用](apps.md#仅为-arm-构建的应用)或 [Android 版本](android-versions.md) |
| `conflicting-signature` | 应用已安装，但使用其他密钥签名 | 先卸载已安装的应用 |
| `downgrade` | 已安装该应用的更新版本 | 先卸载已安装的应用 |
| `invalid-apk` | 该文件不是 Android 可以读取的已签名 APK | 重新获取该 APK |
| `insufficient-storage` | Android 没有足够的空间安装该应用 | 释放 Android 中的空间 |
| `blocked` | 此 Android 的策略不允许该应用 | |
| `other` | 其他任何原因；后面会附上 Android 自己的消息 | |

APK 最大可达 2 GiB。安装一次只进行一个：第二个会等待第一个完成。

## 卸载应用

可以使用以下任一方式：

- Windows 的“**设置** > **应用** > **已安装的应用**”。
- DSA Settings 的 **Apps** 页面：应用旁边的 **Uninstall**（卸载）。
- `dsa apps uninstall <package>`。

属于 Android 本身的应用无法卸载。

---

Android 是 Google LLC 的商标。
Windows 是 Microsoft 集团公司的商标。
DSA 与 Google 或 Microsoft 没有关联，也未获得二者的认可或赞助。
