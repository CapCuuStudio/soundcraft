# ORVIA 4A.2 — Responsive Toolbar hotfix QA (2026-10-23)

**Nguồn phản hồi:** ảnh screenshot thực tế do chủ dự án gửi sau 4A.2 cho thấy toolbar 140px chiếm toàn bề ngang nhưng các nút chỉ tập trung ở phía trái, khoảng trống lớn ở bên phải, khiến Timeline mất chiều cao.

**Base:** nhánh `feature/orvia-4a2-toolbar-layout`, mã Toolbar `933b6a3` đã Cloudflare PASS, chủ dự án đã xác nhận xong bản đầu. Hotfix chỉ sửa khả năng phản hồi kích thước màn hình, không xóa phiên bản 4A.2 gốc.

## Thay đổi
- Khi chiều ngang khu vực Edit >= 1550 logical px, Toolbar sử dụng một hàng gọn cao **78px**. Thứ tự: Transport → Counters → Tempo/Meter → Grid/Nudge → Edit Modes → Tools → Zoom → Output Meter. Không còn hàng thứ hai trống và Timeline được thêm 62px chiều cao.
- Khi chiều ngang Edit < 1550 logical px, Toolbar duy trì **hai hàng 140px** và hai thanh cuộn ngang độc lập, vẫn ưu tiên Transport và Tempo/Grid ở hàng 1.
- `edit_window.rs` sử dụng `toolbar::height_for_width(ui.available_width())` để TopPanel phân bổ đúng chiều cao, tránh khoảng trống giả sau khi chỉ ẩn hàng thứ hai.
- Unit test tại `toolbar.rs` xác nhận breakpoint 1366/1549/1550/1738 logical px và việc màn hình rộng thấp hơn màn hình hẹp.
- Đã đối chiếu mã Toolbar trước/sau: toàn bộ **20 lời gọi command trong file** được giữ nguyên, không đổi xử lý Play/Stop/Record, Tempo, Grid, Undo, Timeline, DSP/audio/MIDI, project Save/Open, export.

## QA Gate
1. Phải có Cloudflare Pages check-run `Deploy successful` đúng commit hotfix và immutable Preview URL. Chưa suy diễn từ bản 4A.2 trước.
2. Browser 1738×863/Chrome có 1 hàng Toolbar gọn, giảm khoảng trống như ảnh khoanh đỏ; không trùng/che nút cuối; Timeline mở rộng. Nếu logical width không đạt 1550 vì zoom/scale, kích hoạt hai hàng là hành vi đúng.
3. Browser 1366×768 vẫn 2 hàng và thanh cuộn riêng; nhóm Tempo/Grid không bị đẩy ra ngoài cùng sau Tools.
4. Điều khiển Play/Stop/Record, Tempo, Grid/Nudge, Zoom, Snap/Smart/Pencil, metronome/CountOff, Waveform, MIDI CC, Inspector/Rack/Automation, Save/Open phải hoạt động. Không đánh dấu bài test là PASS cho đến khi thao tác thật.
5. Rust unit tests chưa có bằng chứng chạy độc lập (sandbox không có cargo); web WASM build thành công chỉ là compile/deploy.
6. Không merge main/production, không Actions tự phát, TinyFish, phí, CMD người dùng.

**Lưu ý:** 1550 là logical viewport width (không đồng nghĩa pixel của ảnh khi trình duyệt zoom hay DPR khác 1).