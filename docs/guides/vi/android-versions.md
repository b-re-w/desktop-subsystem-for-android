# Phiên bản Android

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · [简体中文](../zh-CN/android-versions.md) · [繁體中文](../zh-TW/android-versions.md) · [Español](../es/android-versions.md) · [Français](../fr/android-versions.md) · [Deutsch](../de/android-versions.md) · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/android-versions.md) · [Türkçe](../tr/android-versions.md)

Desktop Subsystem for Android™ (DSA) hỗ trợ hai phiên bản Android chính mới nhất: hiện nay là
Android 16 và Android 17. Theo mặc định, DSA chạy phiên bản mới nhất. Một số ứng dụng chỉ chạy
trên một trong hai phiên bản, vì vậy bạn có thể chọn.

## Xem phiên bản đang chạy

Mở trang **Updates** (cập nhật) của DSA Settings. Trong **Android**, trang này hiển thị phiên bản,
bản dựng, mức bản vá bảo mật, phiên bản mà nó theo dõi và tình trạng cập nhật, kèm tiến trình tải
xuống. **Check now** (kiểm tra ngay) yêu cầu dịch vụ tìm bản cập nhật ngay lập tức. Hoặc chạy:

```
dsa android version   # phiên bản đang chạy, và các phiên bản bạn có thể chọn
```

## Thay đổi phiên bản

Mọi người dùng của PC đều chạy cùng một phiên bản Android, nên chỉ quản trị viên mới có thể thay
đổi nó.

1. Mở trang **Updates** của DSA Settings.
2. Trong **Version to use** (phiên bản sử dụng), chọn một phiên bản, hoặc **Always the newest**
   (luôn dùng bản mới nhất).
3. Chọn **Change** (thay đổi) và chấp thuận lời nhắc quản trị viên.

Phiên bản đã chọn được tải xuống và dùng từ lần khởi động Android tiếp theo. Từ một terminal quản
trị viên:

```
dsa android use 16
dsa android use latest
```

### Chuyển sang phiên bản cũ hơn sẽ xóa dữ liệu

Android không đọc được dữ liệu do phiên bản mới hơn ghi. Vì vậy, chuyển sang một phiên bản chính
cũ hơn sẽ xóa dữ liệu Android của mọi người dùng trên PC: các ứng dụng họ đã cài, tài khoản của họ
và toàn bộ dữ liệu ứng dụng. Cài đặt của DSA vẫn được giữ. Thao tác này không thể hoàn tác.

DSA Settings yêu cầu bạn xác nhận bằng **Erase data and change** (xóa dữ liệu và thay đổi). Trên
dòng lệnh, hãy thêm `--erase-data`:

```
dsa android use 16 --erase-data
```

Chuyển sang phiên bản mới hơn sẽ giữ lại ứng dụng và dữ liệu của bạn.

## Cập nhật

- Phiên bản đã chọn tiếp tục nhận các bản cập nhật bảo mật.
- Khi một phiên bản hết được hỗ trợ, DSA chuyển sang phiên bản tiếp theo và giữ lại ứng dụng của
  bạn.
- DSA không bao giờ tự chuyển sang một phiên bản chính cũ hơn.
- Mỗi ảnh được kiểm tra với một danh sách có ký trước khi được dùng. Ảnh không khởi động được sẽ
  được thay bằng ảnh trước đó.

Quản trị viên có thể tắt cập nhật ảnh bằng **Update the Android image automatically** (tự động cập
nhật ảnh Android) trên trang **Updates** của DSA Settings, hoặc bằng
`dsa settings set image-updates off`.

---

Android là nhãn hiệu của Google LLC. Windows là nhãn hiệu của nhóm công ty Microsoft. DSA không
liên kết với Google hay Microsoft, và không được các công ty này xác nhận hoặc tài trợ.
