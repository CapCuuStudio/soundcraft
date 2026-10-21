# ORVIA — Kiểm kê SoundCraft theo quy trình Cubase
**Ngày:** 22/10/2026. **Nhánh:** feature/orvia-soundcraft-rebuild, PR #6. **Ưu tiên:** chạy ứng dụng Rust/WebAssembly thật trên web, bản Windows sau.

## Phạm vi và nguyên tắc
Đã đối chiếu nhật ký kỹ thuật LUNOR MUSIC và ORVIA, ROADMAP, docs/gaps.md, docs/ui-parity.md, docs/hardware-parity.md, docs/plugin-parity.md, docs/file-format-parity.md, TrackKind, track.new, dialogs, inspector, MIDI editor, edit window, toolbar, docking và web runner. Tham chiếu ảnh Cubase 13 của chủ dự án và tài liệu chính thức Steinberg.

**Chú thích:** Có mã ≠ đã QA trên Chrome. Kiểm tra này dựa trên mã nguồn, tài liệu và thông báo deploy; không chạy trình duyệt/âm thanh thật, không tự chạy GitHub Actions. SoundCraft là pre-alpha; tỷ lệ 40% mà upstream nêu là ước tính so với Pro Tools, không phải độ tương đồng Cubase.

Cloudflare Pages bot xác nhận deploy successful commit 3161a92: https://fa42e6e9.orvia-daw-preview.pages.dev. Script deploy đòi WASM thật, song cần kiểm thử runtime.

## Bảng kiểm kê (P0: bắt buộc; P1: trọng tâm; P2: mở rộng; P3: nâng cao; P4: Windows)

| Nhóm Cubase | ORVIA / SoundCraft có gì | Thiếu hoặc cần xác minh | Ưu tiên |
|---|---|---|---|
| Project Window | Inspector + Timeline kèm track headers + Browser, từ PR #4 | QA phối hợp các panel, độ phân giải laptop, reset workspace | P1 |
| Lower Zone | Key Editor/MixConsole có tab mở đóng, double-click MIDI | QA splitter, đúng clip, chuyển tab không mất lựa chọn | P0 |
| Thanh công cụ | Zoom, Snap, edit mode, Pencil, Trim, các lệnh edit | Bố trí còn thiên Pro Tools (Shuffle/Spot/Slip/Grid), thiếu luồng công cụ thân thiện như Cubase | P1 |
| Info Line | Không xác minh được bộ thanh thuộc tính đầy đủ trong ORVIA Rust | Thiết kế gọn và chỉnh giá trị clip thật, có ẩn/hiện | P2 |
| Add Track | Hộp ORVIA mới dùng lệnh track.new: type, format, timebase, name, count | QA tạo thực, đổi loại, 2 track, Undo và tương thích save | P0 |
| Các loại Track | TrackKind Audio/Aux/Master/MIDI/Instrument/VCA/Folder/Video | Chưa có TrackKind riêng Sampler, FX, Group, Marker, Chord, Arranger; vài chức năng nằm ở bus/group/ruler | P1/P3 |
| Track Headers | Mute/Solo/Arm/Monitor, chọn track, tên, màu, độ cao | QA thao tác đa track/hidden/folder và phím tắt | P1 |
| Inspector | Mute/Solo, volume, pan, plugin insert, chọn instrument gọi lệnh thật | Routing và Sends chủ yếu label chỉ đọc trong panels.rs; chưa có EQ/Send controls đồng bộ | P1 |
| Right Zone/Media | Browser instrument/effect tích hợp với lệnh mix.instrument/mix.insert | Chưa có MediaBay thực: duyệt tệp, sample preview, collections/presets, drag/drop mượt | P1/P2 |
| Instrument playback | Engine có subtractive_synth và drum_synth, instrument track có plugin id | QA âm thanh sau khi vẽ note, đổi synth, volume, lưu/mở lại trên web | P0 |
| Piano Roll | Vẽ/xóa/kéo dài/di chuyển nốt, velocity, quantize, arrows | Cỡ hàng nốt thấp, zoom theo vùng, nhiều editor tools, grid UX cần cải thiện | P1 |
| Nghe phím piano | MIDI editor có vẽ phím, source ghi rõ audition cần live MIDI path | **Chưa có click preview note-on/off đúng nghĩa** | P0 |
| MIDI nâng cao | Engine có lệnh MIDI hơn UI, Velocity lane hiện hữu | CC lanes, sustain/pitchbend, scale/chord, step record, drum editor và roundtrip CC | P2 |
| MIDI keyboard thật | Upstream gap G-02 xác nhận chưa có MIDI device I/O | Web MIDI API tùy quyền trên trình duyệt HTTPS, đi vào synth/record thực | P2 |
| MixConsole | Channel strip cơ bản trên Lower Zone, full mixer, lệnh M/S/fader/pan | QA meter/fader/audio, Bus/Send/Automation/EQ có âm thật, latency/stems | P1 |
| FX | SoundCraft khai báo 26 built-in processors, insert engine | GUI preset/bypass, EQ graph và kiểm thử effect audio/bounce | P1 |
| VST3/Kontakt | Có mã native VST3 host và desktop plugin editor | **Không chạy VST3 DLL/Kontakt cài trên Windows bên trong browser**; để bản Windows | P4 |
| Sampler | Có sampler_window.rs | Không có Sampler TrackKind; cần QA sampler trên web trước khi thêm workflow | P2 |
| Audio clips | Waveform, trim/split/fade, import audio và engine edit | QA WAV thật, long file, Undo, crossfade/snap, export đúng | P1 |
| Thu âm | Native có đường recording nhưng chưa đầy đủ low-latency; web thiếu input/duplex | Không hiển thị Record giả; nếu cần web mic: getUserMedia, AudioWorklet, quyền truy cập + QA | P3 |
| Transport | Play/Stop/Record/Loop, rulers/tempo/locators | QA WebAudio unlock, playhead sync, metronome, tempo/time signature và loop | P0 |
| Save/Open Project | Session .scraft/.scraftz, web load/download bytes | **QA roundtrip kèm file audio** sau reload tab; khôi phục nếu đóng tab | P0 |
| Audio/MIDI import/export | WAV/AIFF/FLAC bounce, SMF import/export | QA duration/levels; docs báo lỗi controllers/filters khi export MIDI, chưa có MP3/AAC encode | P0/P2 |
| Browser autosave | Web hiện thiên về download file project | Chưa thấy IndexedDB/recover được xác minh; cần local autosave và restore có kiểm thử | P1 |
| Workspace | Docking hỗ trợ zone show/hide, layout | Thử 1366x768/1920x1080, nhớ panel size, không che Timeline | P1 |
| Ngôn ngữ | Có một số nhãn tiếng Việt ORVIA | Chưa có bộ bản địa hóa đồng bộ, tooltips/shortcuts lẫn tiếng Anh | P2 |
| Chord/Arranger/Drum/Tempo extras | Có tempo/marker/ruler, score và các lệnh edit; không đầy đủ loại track Cubase | Làm theo workflow thực sự cần, không thêm menu giả để giống hình | P3 |
| Templates/Presets | file.rs có session.save_template, plugin preset có phần code | Tài liệu upstream ghi thiếu templates, cần phân biệt lệnh và workflow được QA; thêm browser preset thực | P2 |
| Thương hiệu/giấy phép | Web có tên ORVIA; SoundCraft dual MIT/Apache, phần assets có quy tắc riêng | Rà logo/notice/attribution trước phát hành; không lấy Steinberg skin.srf, ảnh, GUI XML/crack | P0 release |

**Lưu ý các tương đương không hoàn toàn:** Aux bus khác FX Track chuẩn Cubase; Track Groups trong SoundCraft khác Group Channel Track; markers/tempo trên ruler khác TrackKind riêng. Sampler window không mặc nhiên là Sampler Track. Có lệnh Save Template không có nghĩa đã có thư viện template hoàn chỉnh.

## Chuỗi nghiệm thu bắt buộc — ưu tiên web trước UI nâng cao
1. Mở Cloudflare Preview bằng Chrome/Edge, xác nhận có canvas Rust/WASM chạy thật, không màn đen; thử WebGPU và tùy chọn ?webgl.
2. Tạo 1 Instrument Track và 2 MIDI/Audio Track qua hộp Add Track. Đổi loại/số lượng, Undo/Redo không hỏng session.
3. Chọn nhạc cụ built-in, vẽ 4 nốt MIDI, Play nghe đúng nhịp; chỉnh fader/mute/solo nghe thấy thay đổi.
4. Click phím đàn MIDI để nghe pitch tương ứng; nhả phím phải dừng tiếng, không tự tạo ghi chú.
5. Nhập WAV và MIDI thật; trim, split, snap, velocity, quantize; Undo/Redo từng bước.
6. Lưu session bundle .scraftz, reload trang, mở lại, so note/clip/nhạc cụ/volume và playback. Thử export WAV và MIDI roundtrip.
7. Kiểm thử nhiều track, project 3–5 phút, trình duyệt 1366x768, hiệu suất tải/CPU/memory, báo lỗi có ý nghĩa.
8. Nếu một nút không có xử lý engine thực, tắt nút hoặc gắn chỉ báo rõ; không bao giờ ghi “đã xong” chỉ vì có giao diện.

## Thứ tự phát triển khuyến nghị
- **Mốc A — Nền web đáng tin:** boot, phát synth, editor MIDI, nhập WAV/MIDI, save/reopen kèm media, export WAV, crash/Undo QA. Không mở rộng UI trước khi đạt.
- **Mốc B — Làm việc như Cubase, phong cách ORVIA:** Add Track theo loại, Inspector thao tác Routing/Sends thực, các tool Timeline, Piano Roll nghe preview nốt, browser nhạc cụ; nhất quán vùng Project/Lower.
- **Mốc C — Phối nhạc nghiêm túc:** EQ/bus/effect/preset/automation, CC MIDI, sample libraries/preview, thử project dài.
- **Mốc D — Nâng cao:** sampler track, drum editor, chord/arranger, metronome/record web nếu khả thi, tiếng Việt và UX/accessibility.
- **Mốc E — Windows sau cùng:** driver ASIO/WASAPI, VST3/Kontakt, plugin editor, latency, thu âm dài/đĩa, bộ cài. Không hứa tính năng desktop trên browser.

## Tài liệu đối chiếu
- ORVIA PR: https://github.com/CapCuuStudio/soundcraft/pull/6
- Trục engine/model: crates/model/src/track.rs, crates/engine/src/cmd/track.rs, crates/engine/src/cmd/file.rs
- UI: crates/ui-egui/src/dialogs.rs, panels.rs, midi_editor.rs, edit_window.rs, toolbar.rs, docking.rs
- Web: apps/soundcraft-web/src/web.rs, apps/soundcraft-web/index.html, scripts/cloudflare-pages-build.sh
- Upstream audit: docs/gaps.md, docs/ui-parity.md, docs/hardware-parity.md, docs/plugin-parity.md, docs/file-format-parity.md
- Cubase 13 chính thức: https://archive.steinberg.help/cubase-manuals/cubase-pro/cubase-pro-13/
- Cubase Project Window: https://www.steinberg.help/r/cubase-pro/15.0/en/cubase_nuendo/topics/project_window/project_window_overview_r.html

## Nhật ký
Rà soát theo source và thông báo triển khai. Không sửa engine, DSP hay UI trong lượt này. Không tự bật Actions, không chạy build mới, không merge main, không can thiệp Cloudflare production, không dùng TinyFish/phí mới/CMD trên máy người dùng.
