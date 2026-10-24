# ORVIA Web — Giai đoạn 4B.2 MixConsole / Key Editor UI

**Ngày:** 2026-10-24. **Base:** `feature/orvia-4b1-lower-tabs` (`259c49e1`, docs-only; Cloudflare PASS code `a57faeb8`, người dùng nghiệm thu).

## Scope UI-only

1. **MixConsole:** `crates/ui-egui/src/panels.rs`: khu header sáng rõ hơn (13pt), bảng màu ORVIA Graphite/Cyan Dark và Light riêng, phân biệt strip Track đang chọn, màu semantic cho nút Mute và Solo thật, tên Track/loại Track/volume số đọc dễ hơn, khoảng cách channel dễ phân biệt. Các lệnh engine `edit.select`, `mix.mute`, `mix.solo`, `mix.volume`, `mix.pan`, `mix.send_level`, chọn route thực giữ nguyên. Không đổi cơ chế fader/meter, đơn vị dB, pan law, undo-merge, buffer/audio.
2. **Key Editor:** `crates/ui-egui/src/midi_editor.rs`: header có 2 hàng 56px, title 13pt, thông tin Clip nốt được cắt an toàn tại giới hạn view, các nút Quantize/Velocity ±10/Legato ở hàng 2 vẫn gọi các lệnh cũ. Nền Piano Roll theo token Graphite/Grid/Bar Line Dark, Light tương phản độc lập; giữ nguyên màu nốt theo Track, MIDI sequence và các phép tính note pitch/ticks/grid. Empty-state được Việt hóa.
3. **Resize:** kéo splitter Lower Zone vẫn do `craft_ui::docking` chịu trách nhiệm, không viết bộ splitter mới, không reset workspace người dùng; Key Editor vẫn dùng `docking::minimum_body` ở dưới giới hạn kích thước. Tooltip trên MixConsole nhắc cách kéo đường phân cách.
4. Thêm Rust unit tests cho hai bảng màu và chiều cao header; không sửa `theme.rs`, Saved Workspace, Engine/DSP/session, Audio/MIDI import-export, VST3/Kontakt.

## QA bắt buộc

- [ ] GitHub PR Draft từ đúng nhánh đã nghiệm thu 4B.1; compare chỉ chạm các file UI + tài liệu.
- [ ] Cloudflare Deploy successful **đúng SHA code HEAD**, lấy immutable preview từ check-run/bot; không dùng URL 4B.1.
- [ ] Chrome ở 1366×768, 1440×900, 1738×863: mở MixConsole, Key Editor bằng tab thật; kéo divider Lower Zone cao/thấp nhiều lần, không mất Timeline/Inspector và không reset saved workspace.
- [ ] MixConsole: chọn Track, filter Track được chọn, mute/solo, kéo fader, đọc meter, đặt volume 0 dB, Pan, Sends, Bus routing và Undo/Redo tương tự 4B.1.
- [ ] Key Editor: mở Clip MIDI có tên dài, Quantize, Vel ±10, Legato, thêm/chọn/kéo/xóa nốt, chuyển controller lane; kiểm 420px minimum width và không che nút.
- [ ] Dark/Light/System: tiêu đề, nốt và grid đủ tương phản; không thay màu Track/Clip/Nốt do người dùng chọn.
- [ ] Save/Open, Playback, Export, MIDI/Audio regression, hiệu ứng Rack 4A.3 và Track/Timeline 4A.4.
- [ ] Rust Cargo tests/clippy và Chrome audio QA cần chứng cứ riêng. **Cloudflare WASM build PASS không thay thế runtime kiểm thử chức năng.**

**Không merge main/production, không kích hoạt Actions thủ công, không TinyFish/phí/CMD Windows.**
