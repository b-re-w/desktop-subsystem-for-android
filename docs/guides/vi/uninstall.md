# Gỡ cài đặt

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

Gỡ cài đặt Desktop Subsystem for Android™ (DSA) theo một trong hai cách:

- **Cài đặt** > **Ứng dụng** > **Ứng dụng đã cài đặt** của Windows: tìm Desktop Subsystem for
  Android, rồi chọn **Gỡ cài đặt**.
- Chạy lại trình cài đặt và chọn **Uninstall** (gỡ cài đặt).

Gỡ cài đặt sẽ xóa những thứ sau, cho mọi người dùng của PC, dù đang đăng nhập hay không:

- chương trình, dịch vụ `DsaHost`, lệnh `dsa`, DSA Settings và APK Installer;
- các ảnh Android và bản cập nhật đã tải xuống;
- các mục trong menu Bắt đầu của ứng dụng Android và các mục của chúng trong **Ứng dụng đã cài
  đặt**;
- **Android** trong File Explorer, liên kết tệp `.apk` và các mục trong menu ngữ cảnh;
- cài đặt và nhật ký của DSA.

Việc gỡ cài đặt **giữ lại** đĩa Android của từng người dùng, `%LOCALAPPDATA%\DSA\android.vhdx`,
để lần cài đặt sau tìm lại được ứng dụng và dữ liệu của bạn. Để xóa cả dữ liệu, hãy chạy
`dsa reset-data --yes` trước khi gỡ cài đặt, hoặc xóa tệp đó sau khi gỡ.

Nếu tính năng chuyển hướng thư mục đã di chuyển menu Bắt đầu của một người dùng, các mục ứng dụng
Android của người dùng đó sẽ vẫn còn, vì trình gỡ cài đặt chỉ tìm ở vị trí mặc định. Hãy xóa chúng
bằng tay.

Thiết lập WSL của bạn không bị động đến, vì DSA không bao giờ thay đổi nó.

## Linux (chưa phát hành)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

Gỡ gói cũng sẽ xóa mục kho gói mà gói đó đã thêm.

---

Android là nhãn hiệu của Google LLC. Windows là nhãn hiệu của nhóm công ty Microsoft. DSA không
liên kết với Google hay Microsoft, và không được các công ty này xác nhận hoặc tài trợ.
