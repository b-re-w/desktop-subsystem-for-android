# Bộ nhớ và hành vi khi nhàn rỗi

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · [简体中文](../zh-CN/memory.md) · [繁體中文](../zh-TW/memory.md) · [Español](../es/memory.md) · [Français](../fr/memory.md) · [Deutsch](../de/memory.md) · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · **Tiếng Việt** · [Bahasa Indonesia](../id/memory.md) · [Türkçe](../tr/memory.md)

Desktop Subsystem for Android™ (DSA) trả bộ nhớ lại cho PC của bạn trong khi Android đang chạy,
không chỉ khi Android dừng. Bạn không cần thiết lập gì cho việc này. Trang này giải thích điều gì
xảy ra và cách xem nó.

## Xem Android dùng bao nhiêu bộ nhớ

Mở trang **Memory** (bộ nhớ) của DSA Settings, hoặc chạy `dsa memory`. Cả hai đều hiển thị:

- **On this PC** (trên PC này): lượng bộ nhớ mà máy ảo của Android, trình quản lý cửa sổ và từng
  cửa sổ ứng dụng đang giữ.
- **Inside Android** (bên trong Android): bộ nhớ đang dùng, bộ nhớ đệm, bộ nhớ nén và các tiến
  trình lớn nhất. Android báo các số liệu này vài phút sau khi khởi động.

## Doze, trạng thái nghỉ

Khi mọi cửa sổ Android đã đóng hoặc thu nhỏ được 30 giây, màn hình của Android tắt và Android
chuyển sang Doze, giống như điện thoại khi nằm trong túi.

- Thông báo, báo thức và widget vẫn hoạt động trong Doze. Giữa các lần đó, CPU được nghỉ.
- `dsa status` ghi `Dozing`. Trang **General** (chung) của DSA Settings ghi **Dozing (idle)**.
- Hiện bất kỳ cửa sổ Android nào cũng đánh thức Android ngay lập tức.

Android cũng theo chế độ ngủ của PC. Nó lưu trạng thái trước khi PC ngủ và chỉnh đồng hồ khi PC
thức dậy.

## Đóng băng (tắt theo mặc định)

Đóng băng dừng mọi tiến trình trong Android. Khi đó Android không dùng CPU, nhưng thông báo, báo
thức và widget cũng dừng. Đó là lý do DSA không bao giờ đóng băng Android trừ khi bạn yêu cầu.

- Để đóng băng ngay: **Freeze** trong **Freeze Android now** trên trang **General**, hoặc
  `dsa freeze`.
- Để đóng băng sau 30 phút ở Doze: bật **Freeze Android when idle** trên trang **Advanced** (nâng
  cao), hoặc `dsa settings set freeze on`.
- Mở một cửa sổ Android, hoặc nhập liệu vào đó, sẽ tiếp tục Android.

## Bộ nhớ được trả lại như thế nào

- Máy ảo của Android chỉ lấy bộ nhớ từ PC của bạn khi Android dùng nó lần đầu.
- Bộ nhớ mà Android giải phóng được trả lại cho PC của bạn trong khi Android đang chạy.
- Khi cửa sổ cuối cùng của một ứng dụng đóng, Android kết thúc các tiến trình của ứng dụng đó,
  nên bộ nhớ của chúng được trả lại trong vài giây. Ứng dụng có widget trên desktop, hoặc ứng dụng
  vẫn đang làm việc ở chế độ nền (chẳng hạn trình phát nhạc), sẽ tiếp tục chạy.
- Ứng dụng giữ một dịch vụ nền chạy mà không có cửa sổ, không có thông báo, không có âm thanh và
  không có widget sẽ bị dừng sau năm phút. Nếu Android khởi động nó lại hết lần này đến lần khác,
  DSA chờ lâu hơn sau mỗi lần, tối đa một giờ.
- Khi không có cửa sổ ứng dụng nào mở, Android bỏ bộ nhớ đệm tệp và chuyển bộ nhớ nhàn rỗi sang bộ
  nhớ nén, để trả lại được nhiều hơn.
- Trình quản lý cửa sổ trả lại bộ nhớ nó không dùng 30 giây sau khi cửa sổ ứng dụng cuối cùng
  đóng.

## Giới hạn bộ nhớ

Theo mặc định, giới hạn là **Automatic** (tự động), nghĩa là không có giới hạn cố định. Android
bắt đầu ở 2 GB và tăng dần trong khi chạy, tối đa bằng bộ nhớ của PC trừ phần Windows giữ lại (10%
bộ nhớ hoặc 2 GB, tùy giá trị nào lớn hơn). Android chỉ giữ những gì nó dùng.
Để đặt giới hạn, hãy dùng **Memory limit** (giới hạn bộ nhớ) trên trang **Memory**, hoặc:

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # quay về tự động
```

Giới hạn mới có hiệu lực vào lần khởi động Android tiếp theo.

---

Android là nhãn hiệu của Google LLC. Windows là nhãn hiệu của nhóm công ty Microsoft. DSA không
liên kết với Google hay Microsoft, và không được các công ty này xác nhận hoặc tài trợ.
