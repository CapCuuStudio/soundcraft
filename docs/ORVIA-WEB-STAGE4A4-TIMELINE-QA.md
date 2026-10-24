# ORVIA Web — GĐ4A.4 Track Header / Timeline QA

Ngày: 2026-10-24. Base: feature/orvia-4a3-studio-rack (Cloudflare PASS cho 3149e47 và người dùng xác nhận). Branch: feature/orvia-4a4-track-timeline.

## Đã triển khai (UI-only)
- Phong cách ORVIA Design Language v1: Track được chọn hiển thị rail cyan 3 px; tên Track có nền selected bo 4 px và chữ dễ đọc trong Dark/Light.
- Mute / Solo / Record / Input Monitor dùng nút phẳng bo 4 px và trạng thái semantic (Dark: cyan / amber / coral / green); vẫn giữ mọi vùng bấm, tooltip, keyboard semantics và engine command như trước.
- Clip được chọn: viền cyan 1.5 px, clip thường viền graphite 1 px, góc clip/label bo 4 px. Không đổi màu Clip do người dùng chọn, waveform, MIDI notes, fades hay clip selection criteria.
- Grid Dark dùng bar lines #405363 và minor grid #293744; chế độ Light giữ màu hiện hữu. Không thay tính toán Snap, zoom, vị trí timeline.
- Rust unit test kiểm tra palette Dark/Light ở edit_window.rs.

## Cổng kiểm thử
1. Cloudflare Pages Deploy successful đúng SHA GĐ4A.4, lấy immutable Preview từ bot; không dùng link bản 4A.3.
2. Chrome 1366×768, 1440×900, 1738×863: Track selection cyan rail hiển thị không che level meter/controls, tên Track không bị mất; clip và Timeline rõ nhưng không quá chói, Grid/bar lines vẫn phân biệt.
3. Click Track, Ctrl/Shift multi-selection, đổi Track màu trong menu; màu Track và Clip lưu đúng sau Save/Open. Solo, Mute, Arm, Monitor, Input, resize Track, folder, reorder vẫn hoạt động.
4. Select/Deselect Clip, copy/paste, cut/split/trim, drag, resize, fade/crossfade, Waveform/MIDI, Snap/Grid, Undo/Redo và Save/Open không đổi dữ liệu.
5. Light/System theme: không bị bắt buộc chuyển sang Dark; selected text vẫn đọc được, đường viền không quá đậm.
6. Hồi quy MIDI/CC 3B, Rack 4A.3, Inspector 4A.1, Toolbar hotfix 4A.2, Automation 3D, MixConsole 3A, Playback/Export.
7. Cargo unit tests và Chrome audio UAT chưa được chạy độc lập trong môi trường này; build web không thay thế nghiệm thu từng thao tác.

Giữ nguyên engine/DSP/MIDI/project schema, bản trước, main/production. Không tự chạy GitHub Actions, TinyFish, phí hoặc CMD Windows.
