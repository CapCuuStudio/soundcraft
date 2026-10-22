# ORVIA Web — Giai đoạn 3A: MixConsole (kiểm thử thực)

Ngày: 22/10/2026  
Nhánh: \`feature/orvia-web-stage3a-mixconsole\`  
Cơ sở: GĐ2C \`70e27562ba6df5123eef39454aa4b998ebcd60c4\`; Preview 2C đã được chủ dự án xác nhận UAT 12/12.
Mục tiêu: MixConsole thật, giao diện channel-strip rõ, giữ nguyên engine audio và luồng MIDI.

## Thay đổi mã nguồn

- Nâng cấp **Lower Zone MixConsole** trong \`crates/ui-egui/src/panels.rs\`. Đây là vùng MixConsole mở từ Project Window; Mix window đầy đủ nguyên trạng, không nhân đôi hoặc viết lại engine.
- Hiển thị strip dựa trên chính Track/Mixer của Session, kèm màu Track, tên có thể chọn để xem Inspector, nhấp đúp đổi tên.
- Mute/Solo gọi \`mix.mute\`/\`mix.solo\` theo Track ID; Master không có nút giả.
- Fader dọc dùng \`widgets::fader\`; level meter hiển thị \`app.meters\` đang chạy, không vẽ peak giả. Thay đổi Volume thật qua \`mix.volume\`, nút 0 dB.
- Pan từ \`mixer.pan\` thực, hỗ trợ tối đa hai index với \`mix.pan\`, nút căn giữa riêng mỗi index.
- Output chọn \`Route\` Main/Bus đang có qua \`mix_window::route_popup\`; menu Send xem và chọn Bus thực, chỉnh mức \`mix.send_level\`.
- Tùy chọn **Track được chọn** chỉ lọc hiển thị, không xóa/hide/sửa Track; helper thuần \`orvia_mix_ids\`, unit regression đảm bảo thứ tự, loại hidden và không đổi Session.
- Không chạm file \`crates/engine\`, Session serialization, DSP, plugin host, imports/exports, Transport hoặc mã nguồn bản 2C. Trình duyệt **không host trực tiếp Kontakt/VST3 native**.

## Trạng thái kiểm chứng

1. **Source committed:** chỉ xác nhận sau GitHub diff và SHA.
2. **Cloudflare build:** chỉ PASS khi bot Pages báo Deploy successful đúng commit mới; URL Preview phải lấy từ bot chứ không tự suy đoán.
3. **Rust tests:** \`cargo test -p soundcraft-ui-egui orvia_stage3a_tests\` và \`cargo test -p soundcraft-engine\` hiện **chưa chạy** trên môi trường không có Cargo. Build WASM **không** thay thế Rust unit tests.
4. **Browser manual:** cần người dùng xác nhận trên Chrome/Edge, không tự đánh dấu.

## Các bước kiểm thử GĐ3A

- Mở đúng Cloudflare Preview mới; Project Window → **MIXCONSOLE** dưới Timeline. Ở 1366x768 và 1920x1080: thanh ngang cuộn danh sách Track, các strip và Output phải hiển thị, không che bố cục cũ.
- Bật/tắt lọc **Track được chọn**, chọn Track Kick, Snare hoặc Keys, kiểm tra đúng dải và Inspector đổi Track. Đổi tên nhấp đúp nếu cần.
- Khi bài demo Play: peak meter hoạt động khi engine có tín hiệu; kéo fader của một Track nghe nhỏ/lớn; thử 0 dB; Undo/Redo và Save/Open .scraftz giữ mức. Master không được thay thành Track khác.
- Mute/Solo mỗi Track, kiểm tra chỉ đối tượng được chọn tác động. Thử cả playback sau Undo.
- Điều chỉnh Pan riêng từng index trên Track có hỗ trợ; nút C về giữa; không ép nút Pan lên Track không có trường pan.
- Output chọn Main hoặc Bus tồn tại; Send chọn Bus, chỉnh mức Send; lắng nghe routing, rồi Undo. Không chọn bus không có thật.
- Hồi quy 2C: duplicate/split/delete/Undo/Redo, Play, Save/Open, nhập/xuất WAV/MIDI vẫn hoạt động.

## Nguyên tắc dừng

Không merge vào \`main\`, không đổi Production, không dùng TinyFish, không bật GitHub Actions tự phát, không yêu cầu CMD trên Windows. Không tuyên bố hoàn tất 3A cho đến khi build đúng SHA và thực nghiệm runtime đúng.
