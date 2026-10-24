# ORVIA Web — 4B.3 Brand Mark / Icons / Control States

**Ngày:** 2026-10-24 · **Base:** `feature/orvia-4b2-mixer-key-editor`, owner accepted, Cloudflare PASS code `48d35a5`, docs-only head `1ffa967`.
**Branch:** `feature/orvia-4b3-brand-control-states` · **PR:** Draft; không merge Main/Production.

## Đã triển khai (UI-only)

- `crates/ui-egui/src/icons.rs`: icon `orvia_mark` tự vẽ bằng egui Painter (vòng âm thanh + waveform) từ các đoạn thẳng/vòng tròn, không thêm ảnh/file font, không sao chép logo Cubase/Pro Tools/Steinberg hoặc asset bên thứ ba.
- `crates/ui-egui/src/menus.rs`: biểu tượng 16 × 16 được gắn cạnh menu **ORVIA** thật, menu vẫn bấm được, vẫn nguyên các mục About, Session Info, Quit. Dark cyan #47D7D1 / Light teal.
- `crates/ui-egui/src/widgets.rs`: `icon_button` Toolbar dùng kiểu phẳng radius 5, màu Graphite/Cyan Dark/Light, trạng thái normal/hover/pressed/selected/focus tách biệt, vạch cyan 2px chỉ khi selected, icon dễ đọc, giữ kích thước và handler.
- `crates/ui-egui/src/toolbar.rs`: Play/Stop/Record giữ đúng vùng bấm và lệnh, thêm semantic styling: Play active cyan, Record active coral #F27476, Stop neutral; hover/pressed/focus rõ và vạch active nhỏ. Toolbar vẫn rộng 78px/cửa sổ hẹp 140px, không đổi responsive breakpoint.
- Có Rust unit tests cho palette icons/transport; không đổi engine, DSP, MIDI, VST3, Kontakt, session Save/Open, Mixer, Key Editor, Track/Header, User Track/Clip colors, font loader hay menu command list.

## QA gate

1. GitHub compare với baseline 4B.2 chỉ có `icons.rs`, `menus.rs`, `widgets.rs`, `toolbar.rs` và 2 file tài liệu; commit/code/branch độc lập.
2. Cloudflare Pages check-run **completed/success đúng commit head**; chỉ giao URL preview bất biến được Cloudflare công bố sau PASS, không dùng URL cũ 4B.2.
3. Chrome 1366×768, 1440×900, 1738×863: ORVIA mark không đè menu, `ORVIA` menu bấm được; Toolbar giữ chiều cao 140px và 78px tương ứng; icon không mờ, lệch hoặc thiếu.
4. Play/Stop/Record/Loop/RTZ/Rewind/Fast-forward/End và tools/zoom/edit-modes thực thi như trước; icon active/hover/focus phản ánh đúng trạng thái DAW thực, Record hint vẫn có khi không khả dụng.
5. Thử các trạng thái Dark/Light/System, thao tác chuột, bàn phím Tab/Space/Enter, disabled/lock; âm thanh playback và recording command không thay hành vi.
6. Regression 4A.2 responsive, 4A.3 Rack, 4A.4 Track/Timeline, 4B.1 Lower Tabs, 4B.2 Mixer/Key Editor; undo/redo, Save/Open, Audio/MIDI QA. Rust Cargo tests/clippy và Chromium audio QA cần bằng chứng độc lập.

**Không được suy diễn:** commit lên GitHub không phải bằng chứng build; Cloudflare success không thay thế Rust unit tests và Chrome runtime test; nghiệm thu vẫn do chủ dự án xác nhận.
**An toàn quy trình:** không merge main/production, không chủ động bật GitHub Actions/phí, không TinyFish/CMD Windows.
