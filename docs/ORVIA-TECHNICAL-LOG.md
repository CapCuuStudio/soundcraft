# ORVIA — Nhật ký kỹ thuật (nhánh thử nghiệm)

**Ngày khởi tạo:** 2026-10-20 (Asia/Ho_Chi_Minh)
**Sản phẩm:** ORVIA — Digital Audio Workstation, thuộc LUNOR MUSIC
**Repository:** https://github.com/CapCuuStudio/soundcraft
**Cơ sở:** SoundCraft, upstream https://github.com/storytold/soundcraft
**Base tại thời điểm khởi tạo:** `main` — `3a28a657692c1b98b28c276d8f61df9471074f0f`
**Nhánh:** `feature/orvia-web-bootstrap`
**Website giới thiệu:** https://lunor-web.pages.dev/orvia.html (không phải DAW đang chạy)

## Phải đọc trước mỗi lượt phát triển

1. Nhật ký kỹ thuật chung LUNOR MUSIC trên Google Drive (nguồn quyết định dự án); đồng thời đọc tài liệu này.
2. `AGENTS.md`, `ROADMAP.md`, `docs/gaps.md` của SoundCraft và trạng thái GitHub/preview thực tế.
3. Trước khi sửa, đối chiếu trạng thái branch/PR; không tự merge `main`, không deploy production, không dùng TinyFish, không phí mới, không yêu cầu CMD trên máy người dùng.
4. Mỗi thay đổi phải có nguyên nhân, phạm vi, nguồn đối chiếu, bằng chứng test và việc tiếp theo.

## Khảo sát baseline — 20/10/2026

- Ba nhánh `main`, `craft-upstream`, `lunor-dev` cùng trỏ tới commit `3a28a657...`; chưa có thay đổi ORVIA.
- Workspace Rust với desktop (`apps/soundcraft`), web WASM (`apps/soundcraft-web`) và CLI; UI chia sẻ qua `crates/ui-egui`.
- Dự án gốc tự đánh giá **pre-alpha** (xem `ROADMAP.md` và `docs/gaps.md`), không được tuyên bố là sản phẩm hoàn chỉnh.
- `apps/soundcraft-web/index.html` dùng WebAssembly/Trunk, WebAudio được resume sau tương tác của người dùng. **Chưa kiểm thử runtime của ORVIA**.
- Thương hiệu ArtCraft trong `docs/brand` có giấy phép riêng và không được dùng như dấu nhận diện của ORVIA. Không sao chép logo/trademark vào sản phẩm.
- Phải giữ nguyên cơ chế session `.scraft` / `.scraftz` và tên crate nội bộ trong giai đoạn đầu để tránh phá tương thích trước khi có test.

## Phạm vi mốc 0 — Bootstrap ORVIA Web

- Tạo nhánh tách biệt, đưa nhãn ORVIA vào tiêu đề trang, trạng thái tải và bảng About/menu; ghi nhận nguồn gốc SoundCraft bằng văn bản, không ngụ ý ArtCraft bảo trợ ORVIA.
- Giữ nguyên audio engine, MIDI, định dạng session, playback, export, plugin host và đóng gói hệ thống; không đổi các ID HTML/rust nội bộ khi chưa cần.
- Chưa thêm logo thương hiệu mới; không dùng favicon SoundCraft trong ORVIA Web.
- **Không coi thay nhãn là bản DAW chạy được.** Chưa có build WASM, kiểm thử âm thanh hay preview ORVIA công khai.

## Chặng tiếp theo, theo thứ tự

1. Kiểm tra khả năng build WebAssembly và luồng WebAudio thực, import/export, MIDI, mixer.
2. Thiết kế lại thương hiệu người dùng và thay tất cả tài sản mang dấu ArtCraft trước khi phát hành ORVIA; giữ thông tin giấy phép/ghi công hợp lệ.
3. Xuất Web Preview của **ứng dụng DAW thật** để chủ dự án trực tiếp test, không phải mô phỏng HTML.
4. Chỉ sau nghiệm thu web mới cân nhắc desktop Windows/đóng gói.

## Kiểm thử / chưa xác minh

- Đã kiểm tra tệp nguồn qua GitHub connector; **chưa biên dịch hoặc chạy** ORVIA trên Chrome.
- Chưa có Cloudflare Preview hay file cài đặt cho ORVIA; mọi tuyên bố ngược lại là không chính xác.
- Chưa kiểm tra toàn bộ giấy phép tài sản và logo nhúng trong ứng dụng desktop; không phát hành bản đổi thương hiệu trước khi rà soát.

## Triển khai Cloudflare Pages — 20/10/2026 (chờ kết nối Git)

**Yêu cầu:** Chủ dự án muốn URL Cloudflare Pages chạy ORVIA DAW thật.
**Đã làm:** thêm `scripts/cloudflare-pages-build.sh` (Rust 1.95.0 + wasm32 + Trunk v0.21.14, kiểm SHA256 và output WASM) và `docs/ORVIA-CLOUDFLARE-PAGES.md` (hướng dẫn cấu hình dự án Pages thử nghiệm tách biệt). Cloudflare đặt repo root, build command `bash scripts/cloudflare-pages-build.sh`, output `dist/web`, nhánh `feature/orvia-web-bootstrap`. Không khởi chạy GitHub Actions.
**Kiểm thử:** Kiểm tra nội dung commit qua GitHub connector, script đối chiếu Bash syntax cơ bản trong môi trường độc lập; chưa biên dịch Rust/WASM. Cloudflare Free có giới hạn 20 phút/build và 25 MiB/asset nên build còn có thể thất bại. Chưa có Cloudflare project/URL được xác minh; không được báo đã deploy.
**Vướng mắc:** Không có quyền truy cập tài khoản Cloudflare trong kết nối ChatGPT hiện tại; Cloudflare cần chủ tài khoản thực hiện GitHub OAuth qua dashboard một lần. Không sử dụng TinyFish.
**Việc tiếp theo duy nhất:** Chủ dự án mở Cloudflare Workers & Pages, tạo Pages project mới, Connect to Git → `CapCuuStudio/soundcraft`, nhập cấu hình trong `docs/ORVIA-CLOUDFLARE-PAGES.md`, xem log thành công/thất bại; sau đó kiểm thử ORVIA trực tiếp tại URL thực. Không chạm site LUNOR chính.

## Giai đoạn UI kiểu Cubase — Stage 1 (20/10/2026)

**Yêu cầu:** Chủ dự án thích bố cục Cubase hơn Pro Tools; đồng ý làm và hỏi chia giai đoạn. Chọn 4 giai đoạn, ghi trong `docs/ORVIA-CUBASE-UI-PLAN.md`. Tuyệt đối không dùng asset/mã nguồn độc quyền Cubase.
**Đã đọc trước khi làm:** Nhật ký LUNOR MUSIC Google Drive, file ORVIA này và AGENTS.md, ROADMAP.md, docs/gaps.md; kiểm tra PR #1 và Cloudflare Preview Stage 0.
**Đã triển khai source trên branch:** `feature/orvia-cubase-layout-stage1` (base `feature/orvia-web-bootstrap`), Draft PR #2 https://github.com/CapCuuStudio/soundcraft/pull/2. Thêm panel Inspector hiển thị dữ liệu track đang chọn, nút Open Mixer/Open MIDI Editor/Add Track gọi lệnh có sẵn; workspace có tab Inspector/Tracks bên trái 224px, Timeline thực giữa, Media & Clips bên phải 256px và MIDI dock mở/tắt ở dưới; thứ tự toolbar ưu tiên Transport/Counter/Tempo; thêm 2 Rust unit tests kiểm tra workspace edit và mix độc lập.
**Không thay đổi:** Engine/DSP/audio callback/MIDI events/import-export/session file, Cloudflare build script, nhánh main/production hay site giới thiệu.
**QA thật:** Đã so GitHub compare về danh sách 4 file; chưa có test Rust/WASM build được xác minh, chưa có Cloudflare bot xác nhận PR #2, chưa nghe thử audio trên Chrome. Không được gọi là phiên bản UI hoàn thiện.
**Việc tiếp theo:** Chờ Cloudflare Pages deploy preview commit PR #2. Nếu build fail, sửa theo log. Khi thành công, đưa link immutable cho chủ dự án thử và chờ nghiệm thu trước Stage 2.

## Giai đoạn 1 — Project Zone đúng Cubase (20/10/2026; nhánh thử PR #4)

**Yêu cầu:** Người dùng duyệt bố cục Cubase gồm Inspector bên trái, track headers đi kèm timeline ở trung tâm, VST/Media bên phải, Lower Zone thu gọn được và nhấp đúp MIDI mở Key Editor.
**Đã đọc trước khi sửa:** Nhật ký kỹ thuật LUNOR MUSIC Google Drive; ORVIA repo log, Cubase UI plan, SoundCraft AGENTS.md, ROADMAP/docs/gaps các lượt trước, mã docking/ui/edit_window và tình trạng Draft PR #3 đã Deploy successful commit `d4dd11e`.
**Base:** `feature/orvia-cubase-stage1-rework` tại `d4dd11ebcc3ac69eb46eec5a6bc6db755245bc24`.
**Branch:** `feature/orvia-cubase-project-zone` (Draft PR #4 https://github.com/CapCuuStudio/soundcraft/pull/4).
**Phạm vi triển khai:** `crates/ui-egui/src/docking.rs` bỏ Panel::Tracks khỏi default Edit layout, giữ các track headers trong timeline thực, loại Lower Zone mặc định; reset trở về project layout chuẩn. `crates/ui-egui/src/lib.rs` đặt defaults không ép Tracks/MIDI, thêm `orvia_layout_version=2` và migration cho UI state từ nhánh cũ (không động dữ liệu Session). `crates/ui-egui/src/edit_window.rs` thay footer toggle MixConsole/Key Editor, hai trạng thái không tranh nhau, Reset Layout và double-click clip MIDI gọi lệnh docking mở piano roll. Thêm Rust unit test Lower Zone mở/đóng và khung layout mặc định.
**Chưa thay đổi:** `main`, website LUNOR production, engine audio, MIDI data, plugin host, export/session, Cloudflare build script. Không dùng ảnh/mã có bản quyền của Steinberg/Cubase.
**QA thực tế:** Source đã ghi qua GitHub connector; **chưa xác nhận Cargo test hoặc Cloudflare WASM build cho HEAD PR #4; chưa có URL immutable bản này**. Không được tuyên bố đã chạy trên web/đã đạt Cubase hoặc chất lượng audio chỉ từ source. Kiểm tra sau build: timeline toàn chiều cao, Inspector + track headers một cột, bật/tắt Lower MixConsole/Key Editor và kéo splitter, nhấp đúp MIDI, phát âm thanh, save/undo.
**Việc tiếp theo:** Theo dõi bình luận Cloudflare Pages bot trên PR #4; nếu failed phải đọc log và sửa; nếu successful đúng HEAD mới gửi đường dẫn preview cho người dùng kiểm thử.


## ORVIA — quay về SoundCraft DAW thật (22/10/2026)

**Chỉ đạo:** Chủ dự án chọn SoundCraft làm nền kỹ thuật xây lại ORVIA;
không tiếp tục các bản HTML mock hoặc Zrythm. Yêu cầu workflow gần Cubase,
không copy mã/tài nguyên độc quyền, ưu tiên miễn phí, không CMD và tránh
thao tác gây tốn quota. Trước khi sửa đã đọc nhật ký LUNOR MUSIC Google Drive,
nhật ký này, AGENTS.md, ROADMAP.md, docs/gaps.md, mã Rust và PR #4/#5.

**Nền:** `feature/orvia-cubase-project-zone` PR #4, giữ nguyên audio, MIDI,
VST3 host, session .scraft, runtime native/web. Không lấy nhánh PR #5 vốn chỉ
deploy ảnh và dashboard tĩnh. **Nhánh mới:** `feature/orvia-soundcraft-rebuild`.

**Code đã commit:**
- `crates/ui-egui/src/dialogs.rs`: thay New Tracks dàn hàng bằng hộp
  **ORVIA · Thêm Track** có lựa chọn 8 `TrackKind` thật của engine, cấu hình
  kênh thực có hỗ trợ, MIDI timebase, tên và số lượng; `track.new` được gọi
  trực tiếp. Tự đổi preset tên/kênh/timebase theo loại; thêm 2 unit tests
  cho payload/engine track creation; không tạo dropdown thiết bị giả.
- `apps/soundcraft/src/main.rs`: đổi tiêu đề cửa sổ desktop thành ORVIA Studio,
  giữ IDs, tên crate/binary nội bộ và file data để bảo toàn tương thích.
- `docs/ORVIA-SOUNDCRAFT-REBUILD.md`: mô tả phạm vi, giới hạn, QA gates.

**Chưa xác minh:** Môi trường container không có `cargo`/`rustc`, GitHub Actions
không được kích hoạt do giới hạn quota. Chưa compile Rust mới, chưa có .exe hoặc
web build mới, chưa test thực VST3/Kontakt hoặc hardware I/O. Không gọi đây là
đã hoàn thành DAW, không gửi URL preview như đang chạy phần mềm. Icon
SoundCraft hiện chưa được thay bằng icon ORVIA gốc.

**Bảo toàn:** Không merge `main`, không thay PR #4/#5 hoặc các repo khác,
không dùng TinyFish, dịch vụ phí hay CMD máy chủ dự án.
**Tiếp theo:** PR Draft nhánh mới; khi có runner miễn phí được duyệt phải
chạy Cargo check/test và test native chính xác, rồi nghiệm thu luồng
tạo Track / Piano Roll / Play / Undo / VST3.

## Ưu tiên ORVIA Web thật — 22/10/2026

Chỉ đạo người dùng: **trước mắt làm trên web**, Windows installer xếp sau. Đã đọc lại nhật ký Drive/nhánh và kiểm tra code SoundCraft WebAssembly (`apps/soundcraft-web/src/web.rs`, `index.html`, `Trunk.toml`, scripts/cloudflare-pages-build.sh) cùng PR #6 và Cloudflare Pages bot.

**Bằng chứng thật:** Cloudflare bot PR #6 báo `Deploy successful` tại commit `ee61e06`; immutable preview https://76d548d3.orvia-daw-preview.pages.dev, branch preview https://feature-orvia-soundcraft-reb.orvia-daw-preview.pages.dev. Script deploy đòi có file `.wasm`, không phải website gallery/HTML mô phỏng. **Chưa kiểm thử trực tiếp audio, MIDI, save/open trong Chrome**; không kết luận DAW đã hoạt động đầy đủ.

**Bổ sung mã cho web (sau commit được bot xác nhận):** `apps/soundcraft-web/index.html` kiểm tra `navigator.gpu` trước khi khởi tạo; `apps/soundcraft-web/src/web.rs` tự chọn backend WebGL2 trong Rust egui/wgpu nếu không có WebGPU, vẫn cho phép `?webgl` thủ công. Khi runtime khởi động lỗi, hiện thông báo text an toàn. Mã web dùng chung hộp Add Track thật đã sửa ở nhánh này. Các commit này **chưa được xác minh Cloudflare build/Chromium cho HEAD**, không gộp với bằng chứng commit ee61e06.

Không khởi chạy GitHub Actions; chưa có Rust/Cargo trong container; không CMD máy người dùng, phí hay TinyFish. Không merge main, không thay production `lunor-web`, không triển khai bản Windows. Tiếp theo: xác minh Cloudflare bot thành công đúng HEAD của nhánh mới, rồi QA trải nghiệm trình duyệt và audio/MIDI trước khi nghiệm thu. Browser không host được VST3/Kontakt native.


## Giai đoạn 2 — Project Workflow theo Cubase, đợt 1 (22/10/2026)

**Yêu cầu mới:** Người dùng xác nhận bản ORVIA Web GĐ1 chạy và *phát nhạc bình thường*, đồng ý bắt đầu GĐ2. Không đánh dấu GĐ1 toàn phần đã nghiệm thu: lưu/mở project .scraftz, nhập/xuất WAV/MIDI vẫn chưa có bằng chứng thao tác thực. Trước thay đổi đã đọc nhật ký Drive, file này, audit 30 nhóm, kế hoạch Project Window, AGENTS.md và PR #6.

**Đường phát triển:** tạo nhánh `feature/orvia-web-stage2-project-workflow` từ `feature/orvia-soundcraft-rebuild` HEAD `c2ea6ab`; Draft PR **#7** https://github.com/CapCuuStudio/soundcraft/pull/7 (base nhánh GĐ1), không sửa/merge main.

**Thay đổi Rust UI dùng chung desktop + WebAssembly:**
- `crates/ui-egui/src/orvia_project_bar.rs` (mới): dải Project Tools nhỏ Select/Move/Draw/Trim/Zoom/Smart và Grid gọi `edit.tool`/`edit.mode` có thật. Info Line (có ẩn nội dung) lấy clip thực đang chọn: tên, Track, Start/Length giây với lệnh `edit.move_clips`/`edit.trim_clip`, Gain cho Audio qua `clip.gain`, Mute qua `edit.mute_clips`, Rename qua dialog đang có. Tôn trọng `clip.edit_locked` cho các trường chỉnh sửa. Có unit test tham số clip target.
- `crates/ui-egui/src/lib.rs` và `edit_window.rs`: gắn module vào top panel cao **33 px**, giữ nguyên main toolbar, timeline, transport, MixConsole/Key Editor.
- `crates/ui-egui/src/panels.rs`, `mix_window.rs`: Routing output chọn Main/bus thật dùng menu và `track.output` sẵn có; Routing input chỉ cho chọn ở native, **web thể hiện chưa hỗ trợ thu âm đầu vào**. Sends hiện toàn bộ slot thật, chọn bus, sửa level/mute/remove qua lệnh `mix.send`, `mix.send_level`, `mix.send_remove`. Thêm đổi tên Track qua dialog, sửa nhãn browser built-in khỏi gây hiểu nhầm web hỗ trợ VST3 Windows.

**Kiểm thử:** mã đã commit, xem diff PR #7. Cloudflare tự động build preview từ GitHub; tại lúc cập nhật log chưa có bằng chứng **Deploy successful** đúng HEAD Stage2. Môi trường hiện không có cargo/rustc, không tự chạy GitHub Actions hoặc sử dụng TinyFish nên unit test chưa được chạy. Không báo GUI/audio/saving stage2 đã QA xong.

**Cần nghiệm thu GĐ2:** build WASM HEAD thành công; mở Project Window thực tại 1366×768, Tool chọn/di chuyển/vẽ/trim/Grid gọi engine; chọn Audio và MIDI clip, Info Line phản ánh đúng; sửa Start, Length, Gain, Mute, Rename, Undo/Redo, khóa clip; routing Main/Bus và Sends tác động đến mixer; lưu/mở .scraftz không mất dữ liệu; GĐ1 playback vẫn có tiếng. Chỉ sau browser QA mới chuyển tiếp nâng cấp GĐ2.

**Bảo toàn:** không dùng mã/icon/skin độc quyền Cubase; không chạm engine DSP, session format, bản main/production; không GitHub Actions, TinyFish, dịch vụ phí hay CMD máy người dùng.

## Giai đoạn 2A — Font tiếng Việt và bố cục gọn (22/10/2026)

Người dùng duyệt Bước 2A; đã đọc nhật ký chung/nhánh, fonts.rs, Info Line, Inspector, AGENTS.md và ATTRIBUTION.md. Nhúng Be Vietnam Pro Regular .ttf từ Google Fonts, SIL OFL 1.1, vào `FontDefinitions` của egui chỉ ở wasm32 để dấu tiếng Việt hiển thị trong canvas thực, thêm `assets/fonts/OFL.txt` và attribution. Thêm test header TTF và kiểm tra đăng ký font trong 3 family trên WASM. Đưa nhãn tiếng Việt vào Info Line và Inspector/Right Zone, thu Info Line 33→31px và đưa thông báo input browser xuống hàng riêng để tránh tràn tại 1366x768. Bảo toàn transport/engine/audio/MIDI/session và toolbar chức năng. Cần Cloudflare deploy successful đúng HEAD, test chữ dấu tiếng Việt trong Chrome và QA chức năng trước khi đánh dấu PASS. Không tự chạy Actions/CMD/TinyFish hoặc merge production.

## Giai đoạn 2C — Rà soát tiếp và sửa context menu (22/10/2026)

**Trước sửa:** Đã đọc nhật ký LUNOR MUSIC trên Google Drive, nhật ký ORVIA này, AGENTS.md, ROADMAP.md, docs/gaps.md, QA 2C và source PR #9. GitHub Pages bot bình luận PR #9 cho commit `b7137e4` **Deploy successful**, immutable https://75f57c9c.orvia-daw-preview.pages.dev; compare branch xác nhận HEAD cũ là chính commit đó. Bản 2C có bằng chứng build thành công, **chưa có browser runtime QA/Cargo test**.

**Nguyên nhân lỗi mới BUG-2C-06:** Context menu ở `edit_window.rs` lưu ClipId khi nhấp phải nhưng nhiều lệnh clip vẫn có precondition `has_selection`, gây không thực thi nếu chưa chọn Track/Clip. **Sửa giới hạn:** khi người dùng bấm hành động context clip, `edit.select` noundo chọn đúng ClipId (`exact:true`) trước khi gọi lệnh, bảo vệ việc target không nhầm sang clip đã chọn trước đó. Bổ sung test engine cho Mute/Edit Lock đúng clip, clip track khác giữ nguyên. QA report 2C cập nhật bằng chứng Cloudflare cùng lưu ý version.

**Chưa xác minh:** Rust tests và Chrome hành động Mute/Lock/Quantize/context mới. Commit mới cần Cloudflare deploy đúng SHA riêng; không đánh đồng với build thành công `b7137e4`. Không chạy GitHub Actions, TinyFish, CMD máy người dùng, không sửa main/production, không dịch vụ trả phí.

**Bước tiếp theo:** theo dõi Cloudflare PR #9 của commit mới; nếu success, người dùng mở preview để thử context-menu Mute/Lock/Quantize, duplicate/split/Undo, lưu/mở .scraftz và phát audio. Chưa chuyển GĐ3 trước cổng nghiệm thu.
