# ORVIA Web — Giai đoạn 4A.2: Toolbar usability & regression QA

**Ngày:** 23/10/2026. **Branch:** `feature/orvia-4a2-toolbar-layout`. **Base:** `feature/orvia-4a1-inspector-layout` (người dùng xác nhận 4A1 đã xong; Cloudflare verified successful code 42d4bbc, subsequent changes technical log only).

## Vấn đề
Toolbar cũ hiển thị **tất cả nhóm trên một hàng cuộn ngang**: Transport, Counter, Tempo, Edit Mode, Tools, Grid/Nudge, Zoom, Level Meter. Trên màn hình 1366×768 và nhỏ hơn, các nút ở phía phải bị khuất, cần kéo ngang toàn bộ, khó tìm Tempo/Grid/công cụ.

## Phạm vi thực hiện
- **Hàng 1: PHÁT / NHỊP**: Loop / Stop / Play / Record, điều khiển cuộn nhạc, vị trí/selection, CountOff, Meter, Tempo, Grid/Nudge, Level Meter. Không còn các nhóm Edit Mode/Tools chen giữa Tempo và Grid.
- **Hàng 2: CHỈNH SỬA**: Shuffle/Slip/Spot/Grid edit modes, Tool palette/Smart Tool, Zoom và nhãn công cụ đang chọn.
- Tăng chiều cao toolbar 78px→140px để hai hàng được nhìn thấy đồng thời, mỗi hàng có thanh cuộn ngang **riêng** chỉ khi màn hình quá hẹp. Không cho hàng công cụ đẩy nút Play/Stop ra ngoài vùng nhìn trên desktop phổ biến.
- Nút vận chuyển rộng hơn 38→42px, vẫn gọi đúng các command: `transport.stop`, `transport.play`, `transport.record`, `transport.rtz`, `transport.rewind`, `transport.fast_forward`, `transport.go_to_end`, `options.loop_playback`.
- Giữ nguyên cách sửa Tempo bằng `event.tempo` với `execute_merged`, không đổi tempo/meter model. Giữ `view.grid`, `view.nudge`, `edit.mode`, `edit.tool`, Zoom, Waveform Zoom, Snap, audio click/Count Off và toàn bộ shortcut.
- Không sửa DSP/audio/MIDI/playback/Save/Open, Inspector 4A1, MixConsole, Piano Roll, Rack, Automation.

## Cổng nghiệm thu
1. Cloudflare Pages `Deploy successful` **đúng mã 4A2**, URL bất biến từ Cloudflare và PR Draft riêng. Trạng thái `in_progress` không phải thành công.
2. Rust Cargo compile/unit tests cho UI/Toolbar chưa chạy độc lập trong môi trường này; nếu Cloudflare build PASS vẫn phải ghi chưa chạy tests.
3. Chrome 1366×768 và 1440×900: hàng 1 luôn hiện Play/Stop/Record, Counter, Tempo, Grid/Nudge và hàng 2 hiện Edit Mode, Tools, Zoom, không đè Timeline.
4. Chrome màn hình hẹp: cuộn riêng từng hàng; hàng công cụ không kéo theo hàng Transport.
5. Phát/dừng vòng lặp, tua, Record khi có input (không giả định web thu âm native), CountOff/metronome; Tempo nhập hoặc kéo chỉnh, Undo, thay đổi Grid/Nudge; kiểm tra Snap và Smart/Pencil/Grabber tool.
6. Hồi quy Inspector 4A1, Track Automation 3D, FX Rack 3C, MIDI CC 3B, Mixer 3A, timeline/Save/Open/Export; không tự phát tiếng hoặc mất dữ liệu khi chỉ thay giao diện.

**Bảo toàn:** giữ nguyên các bản 3A–4A1 và production/main; không merge; không TinyFish/GitHub Actions tự phát/phí mới/CMD Windows. Nếu chưa có UAT Chrome thì chỉ ghi code implemented + verified web build, không tuyên bố toàn bộ test PASS.