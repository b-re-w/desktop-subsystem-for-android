# Tệp

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · [简体中文](../zh-CN/files.md) · [繁體中文](../zh-TW/files.md) · [Español](../es/files.md) · [Français](../fr/files.md) · [Deutsch](../de/files.md) · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · [Italiano](../it/files.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/files.md) · [Türkçe](../tr/files.md)

Desktop Subsystem for Android™ (DSA) cho phép bạn mở tệp của Android trong File Explorer, chia sẻ
các thư mục Windows của bạn với ứng dụng Android, và gửi một tệp đến ứng dụng Android. Trang này
mô tả DSA trên Windows.

## Mở tệp của Android trong File Explorer

Dùng một trong các cách sau. Mỗi cách đều khởi động Android nếu Android chưa chạy.

- **Android** trong ngăn điều hướng của File Explorer, ngay bên dưới **Linux** của WSL.
- **Android files** (tệp Android) trên trang **Advanced** (nâng cao) của DSA Settings.
- `dsa files open`. `dsa files` cho biết bộ nhớ nằm ở đâu mà không mở nó.

Bạn sẽ thấy bộ nhớ dùng chung của Android: các tệp mà ứng dụng Android đã lưu, chẳng hạn ảnh và
tệp tải xuống. Bạn có thể sao chép tệp vào và ra như với bất kỳ thư mục nào. File Explorer truy
cập bộ nhớ qua trình khách WebDAV của Windows. Nếu bộ nhớ không phản hồi, DSA hiển thị lỗi kèm lý
do.

## Chia sẻ thư mục Windows của bạn với ứng dụng Android

Các thư mục Tài liệu, Tải xuống, Ảnh, Nhạc, Video và Màn hình nền của bạn xuất hiện trong bộ nhớ
dùng chung của Android dưới `Host`, chẳng hạn `Host/Pictures`. Ứng dụng Android có thể mở và lưu
tệp ở đó. Ứng dụng cần quyền bộ nhớ của Android trước, và Windows chỉ cho nó làm những gì chính
tài khoản của bạn được phép làm.

Việc chia sẻ được bật theo mặc định. Để thay đổi, mở trang **Advanced** của DSA Settings:

- Tắt **Share Windows folders with Android** để không chia sẻ thư mục nào.
- Trong **Shared folders** (thư mục dùng chung), bỏ chọn một thư mục để ngừng chia sẻ nó. Thư mục
  đó rời khỏi Android ngay lập tức.

Từ dòng lệnh:

```
dsa settings set shared-folders Documents,Pictures   # chỉ các thư mục này; cũng có all hoặc none
dsa settings set file-sharing off                     # không chia sẻ thư mục nào
```

Các phiên bản trước dùng thư mục tên `Windows` thay cho `Host`. Android xóa thư mục cũ khi nó
trống, nên các tệp bạn đã lưu ở đó vẫn còn.

## Gửi một tệp đến ứng dụng Android

- **Kéo và thả.** Kéo một tệp từ File Explorer thả vào cửa sổ của một ứng dụng. Tệp sẽ đến ứng
  dụng đó. Riêng APK thì đến APK Installer. Thư mục không được gửi.
- **Menu ngữ cảnh.** Nhấp chuột phải vào một tệp và chọn **Open in an Android app** (mở trong ứng
  dụng Android) hoặc **Share with an Android app** (chia sẻ với ứng dụng Android). Trên Windows 11,
  các mục này nằm trong **Hiển thị thêm tùy chọn**. Sau đó Android hỏi ứng dụng nào nhận tệp.
- **Dòng lệnh.** `dsa open <file>` và `dsa share <file>`.

Các cách này gửi tệp có kích thước đến 64 MiB. Tệp lớn hơn sẽ bị từ chối trước khi bất cứ thứ gì
được gửi. Để đưa một tệp lớn hơn cho ứng dụng, hãy đặt nó vào một thư mục dùng chung.

## Lấy một tệp từ Android

- Lưu tệp trong Android dưới `Host/<folder>`. Khi đó tệp nằm trong thư mục đó trên PC của bạn.
- Hoặc mở **Android** trong File Explorer và sao chép tệp ra.

---

Android là nhãn hiệu của Google LLC. Windows là nhãn hiệu của nhóm công ty Microsoft. DSA không
liên kết với Google hay Microsoft, và không được các công ty này xác nhận hoặc tài trợ.
