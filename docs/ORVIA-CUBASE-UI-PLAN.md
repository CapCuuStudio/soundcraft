# ORVIA — Kế hoạch chuyển giao diện theo hướng Cubase

**Quyết định:** chia thành 4 giai đoạn để bảo toàn engine, project/session, MIDI, audio và VST của SoundCraft. Không sao chép logo, ảnh, âm thanh, mã nguồn hay nội dung độc quyền của Cubase/Steinberg.

| Giai đoạn | Phạm vi | Điều kiện nghiệm thu |
| --- | --- | --- |
| 1. Khung làm việc | Inspector bên trái, timeline trung tâm, Media & Clips bên phải, thanh Transport/Tempo dễ thấy, Lower Zone MIDI có thể mở | Trên Cloudflare Preview: layout hiển thị, chọn track -> Inspector cập nhật, mở Mixer/MIDI, Play/Stop cũ không hỏng |
| 2. Chỉnh sửa dự án | Thanh track chuyên nghiệp, điều chỉnh vùng Inspector, công cụ kéo/cắt/snap, bố cục phối màu và khả năng thu gọn panel | Kiểm thử tương tác clip/undo/redo, thay đổi track, lưu/mở session |
| 3. MixConsole & MIDI | Mixer dạng channel strip, lower zone cho Piano Roll/Velocity/automation, bộ duyệt plugin và nhạc cụ với khả năng thật của host | Fader/pan/mute/solo, xuất âm thanh và MIDI không bị ảnh hưởng, không quảng cáo tính năng plugin chưa có |
| 4. Hoàn thiện | Nâng cấp thẩm mỹ, phím tắt, responsive, QA audio/MIDI/web, rebranding loại dấu ArtCraft trước phát hành; sau nghiệm thu mới xem xét Windows | Cloudflare web vận hành thật và người dùng chốt UI, mới đóng gói |

## Giai đoạn 1 — phạm vi kỹ thuật

- Branch: `feature/orvia-cubase-layout-stage1`, nền `feature/orvia-web-bootstrap`.
- Sửa UI Rust/egui dùng chung web/desktop, không tạo web HTML mockup.
- Inspector hiển thị track đang chọn, có nút gọi Mix, MIDI Editor, Add Track thực tế.
- Bố cục dock: trái Inspector/Tracks tabs, giữa project timeline thực, phải Media & Clips (chỉ list media/clip hiện có, CHƯA có VST browser), MIDI ở lower zone mở bằng chức năng sẵn có.
- Ưu tiên Transport, counter và Tempo lên đầu toolbar trên màn hình hẹp.
- Không sửa: audio engine, DSP, session format, MIDI events, clip editing commands, Cloudflare build script, main, release workflow.

## QA / cảnh báo

- Tài liệu và source đã được kiểm tra qua GitHub, **chưa có bằng chứng compile/test ở runner**. Cần đợi Cloudflare bot báo đúng commit rồi kiểm thử giao diện/âm thanh trên Chrome thật.
- Các phần 2–4 chưa triển khai; đây là khung Cubase-inspired **không phải sao chép Cubase 1:1**.
- Mỗi stage phải có PR nhánh thử và URL Preview commit thực tế trước khi chuyển giai đoạn; không tự merge main hay site LUNOR.
