# Cài đặt APK

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · [简体中文](../zh-CN/install-apks.md) · [繁體中文](../zh-TW/install-apks.md) · [Español](../es/install-apks.md) · [Français](../fr/install-apks.md) · [Deutsch](../de/install-apks.md) · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/install-apks.md) · [Türkçe](../tr/install-apks.md)

Desktop Subsystem for Android™ (DSA) không đi kèm cửa hàng ứng dụng nào. Bạn cài ứng dụng từ tệp
APK, bằng APK Installer hoặc lệnh `dsa`. Trang này mô tả DSA trên Windows.

## Cài bằng APK Installer

1. Nhấp đúp vào một tệp `.apk` trong File Explorer. Tệp mở trong **APK Installer** (trình cài
   APK).
2. Kiểm tra APK là gì. Cửa sổ hiển thị thông tin mà không khởi động Android:
   - tên, biểu tượng, gói và phiên bản của ứng dụng;
   - nhà phát hành, lấy từ chứng chỉ ký, vì APK không ghi tên nhà phát hành;
   - nguồn: máy chủ nơi tệp được tải về, hoặc thư mục chứa tệp;
   - trong **Capabilities** (khả năng), các quyền đầu tiên mà ứng dụng yêu cầu. **More** (thêm)
     hiển thị tất cả các quyền, các phiên bản Android mà ứng dụng được làm cho, kích thước, các
     loại bộ xử lý và SHA-256 của chứng chỉ.

   Khi APK chỉ có mã cho một loại bộ xử lý khác, cửa sổ sẽ cảnh báo bạn.
3. Chọn **Install** (cài đặt). Khi ứng dụng đã được cài, nút sẽ ghi **Update** (cập nhật) hoặc
   **Install again** (cài lại). Android khởi động trước nếu chưa chạy.
4. Khi **Launch when ready** (mở khi sẵn sàng) được đánh dấu (mặc định), ứng dụng mở ngay sau khi
   cài xong và trình cài đặt đóng lại. Nếu không, hãy chọn **Launch** (mở), hoặc mở ứng dụng sau
   từ menu Bắt đầu.

Khi Android đang chạy, cửa sổ cũng cho biết tệp là một lần cài mới, một bản cập nhật, cùng phiên
bản đó lần nữa, hay thứ mà Android từ chối: một phiên bản cũ hơn, hoặc cùng ứng dụng nhưng được ký
bằng khóa khác.

Bạn cũng có thể thả tệp APK lên APK Installer, lên DSA Settings hoặc lên cửa sổ của một ứng dụng.
Mỗi APK mở trong một cửa sổ trình cài đặt riêng.

## Cài từ dòng lệnh

```
dsa apk-info app.apk            # APK là gì; đọc tệp mà không khởi động Android
dsa install app.apk             # cài đặt; khởi động Android nếu chưa chạy
dsa install app.apk --progress  # in thêm từng bước
```

Khi Android từ chối APK, `dsa install` thoát với mã 1, và thông báo của nó bắt đầu bằng
`not installed (<reason>):`.

## Vì sao một lần cài bị từ chối

| Lý do | Ý nghĩa | Việc cần làm |
|---|---|---|
| `incompatible` | Ứng dụng cần loại bộ xử lý khác, phiên bản Android mới hơn, hoặc một tính năng mà thiết bị này không có | Xem [Ứng dụng chỉ dành cho ARM](apps.md#ứng-dụng-chỉ-dành-cho-arm), hoặc [Phiên bản Android](android-versions.md) |
| `conflicting-signature` | Ứng dụng đã được cài, được ký bằng khóa khác | Gỡ ứng dụng đã cài trước |
| `downgrade` | Một phiên bản mới hơn của ứng dụng đã được cài | Gỡ ứng dụng đã cài trước |
| `invalid-apk` | Tệp không phải là APK có ký mà Android đọc được | Tải lại APK |
| `insufficient-storage` | Android không còn chỗ cho ứng dụng | Giải phóng dung lượng trong Android |
| `blocked` | Một chính sách của Android này không cho phép ứng dụng | |
| `other` | Mọi lý do khác; thông báo riêng của Android theo sau | |

Một APK có thể lớn đến 2 GiB. Các lần cài chạy lần lượt từng cái: lần cài thứ hai chờ lần thứ
nhất.

## Gỡ cài đặt ứng dụng

Dùng một trong các cách sau:

- **Cài đặt** > **Ứng dụng** > **Ứng dụng đã cài đặt** của Windows.
- Trang **Apps** của DSA Settings: **Uninstall** (gỡ cài đặt) bên cạnh ứng dụng.
- `dsa apps uninstall <package>`.

Các ứng dụng là một phần của Android không thể gỡ cài đặt.

---

Android là nhãn hiệu của Google LLC. Windows là nhãn hiệu của nhóm công ty Microsoft. DSA không
liên kết với Google hay Microsoft, và không được các công ty này xác nhận hoặc tài trợ.
