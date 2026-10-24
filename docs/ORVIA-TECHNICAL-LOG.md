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

## Giai đoạn 2C — Ngăn xóa nhầm clip khi menu Delete (22/10/2026)

**Đã đọc trước khi sửa:** nhật ký LUNOR MUSIC Google Drive, tài liệu này, AGENTS.md, ROADMAP.md, docs/gaps.md và báo cáo QA 2C, cùng source `cmd/edit.rs`, `cmd/clip.rs`, `edit_window.rs`. Xác nhận Cloudflare bot PR #9 `0f9f0b8` Deploy successful tại https://765acb46.orvia-daw-preview.pages.dev, branch HEAD đúng SHA trước sửa; không truy cập được trực tiếp trang Cloudflare (DNS từ môi trường công cụ). Không có rustc/cargo tại môi trường nên Rust unit tests chưa chạy.

**BUG-2C-07 dữ liệu:** `Delete` trong menu chuột phải đang chọn một clip, rồi gọi `edit.clear` xóa mọi clip trong khoảng thời gian chọn trên track. Có nguy cơ làm mất/cắt clip chồng vùng. Sửa giới hạn: lệnh engine `clip.delete_one` xóa duy nhất target ClipId, từ chối clip bị khóa hoặc ID sai, xóa selection còn treo; GUI Delete chuyển sang lệnh này và disabled nếu locked; không sửa hành vi edit.clear cho xóa khoảng chủ ý. Test engine `orvia_stage2c_delete_one_preserves_overlaps_locks_and_undo` kiểm tra overlapping clip/track khác giữ nguyên, Undo/Redo và không biến đổi khi locked/missing id.

**QA gate:** source+tests committed không đồng nghĩa PASS; phải có Cloudflare deploy đúng commit sửa Delete, Cargo test và Chrome runtime QA với audio/save/open. Không merge main hoặc production, không Actions/TinyFish, không dùng dịch vụ phí hoặc CMD máy người dùng.

## Giai đoạn 2C — Bổ sung an toàn xóa clip và khoảng chọn (22/10/2026)

Đã đọc lại nhật ký dự án Google Drive, báo cáo QA và nhật ký repo; Cloudflare Pages bot PR #9 xác nhận **Deploy successful** đúng commit `4463fa519b865b430173fffe81ebe1b4f4e38c83`, Preview bất biến https://23b392b0.orvia-daw-preview.pages.dev; GitHub compare HEAD match. Cloudflare thành công chỉ là bằng chứng build/deploy, chưa xác minh browser/audio.
Audit `clip.delete_one` phát hiện trạng thái chọn có thể giữ lại `edit.selection` với khoảng thời gian của clip đã bị xóa. Vì người dùng sẽ tiếp tục edit sau Delete, vùng chọn mồ côi có thể khiến lệnh range-based tiếp theo xóa nhầm vật liệu còn lại. Sửa để collapse khoảng chọn về một điểm khi clip đang chọn cuối cùng bị xóa; cập nhật `timeline_selection` khi linked, còn lựa chọn chưa liên quan không đổi. Bổ sung test `orvia_stage2c_delete_selected_clip_clears_stale_range`; cập nhật báo cáo QA. Chưa chạy Cargo tests/Chrome runtime (không có cargo/rustc và môi trường không truy cập URL Cloudflare), không tự chạy Actions, TinyFish, CMD máy người dùng, hay phí. Nhánh PR #9 vẫn độc lập main/production. Cổng tiếp theo là Cloudflare build của commit mới và nghiệm thu thao tác thật (Delete/Undo, Duplicate/Split, Save/Open, Play, Export).


## ORVIA WEB — Giai đoạn 3A MixConsole (22/10/2026)

**Quyết định:** chủ dự án báo nghiệm thu sử dụng GĐ2C UAT 12/12, yêu cầu thực hiện GĐ3A. Đọc nhật ký kỹ thuật LUNOR MUSIC trên Drive, nhật ký repo này, docs/ORVIA-CUBASE-UI-PLAN.md, AGENTS.md, ROADMAP.md, docs/gaps.md, QA 2C và source \`panels.rs\`, \`mix_window.rs\`, \`docking.rs\`, \`widgets.rs\` trước khi sửa. Baseline GĐ2C \`70e27562ba6df5123eef39454aa4b998ebcd60c4\` đã được Cloudflare Deploy successful tại https://e21f618c.orvia-daw-preview.pages.dev; Rust Cargo tests 2C chưa có kết quả.
**Nhánh:** \`feature/orvia-web-stage3a-mixconsole\`, tách trực tiếp từ 2C, PR thử nghiệm riêng. 
**Thay đổi:** cải tiến Lower Zone MixConsole hiện có thành channel strips thực trong \`crates/ui-egui/src/panels.rs\`: chọn Track để cập nhật Inspector, Mute/Solo qua engine; fader thẳng đứng widgets::fader và meter app.meters thật; Volume reset 0dB; Pan theo index, centering; Route Output Main/Bus qua popup engine; Send chọn Bus và chỉnh level; lọc chỉ Track được chọn nhưng không sửa Session. Thêm test helper lọc visible Tracks, bỏ hidden/Video và giữ thứ tự. Mix window đầy đủ, audio DSP, playback, transport, MIDI, session format và plugin host giữ nguyên. Tài liệu QA mới docs/ORVIA-WEB-STAGE3A-MIXCONSOLE.md.
**Giới hạn:** source commit không đồng nghĩa QA Rust hoặc browser đã chạy; chỉ gọi web build PASS khi Cloudflare trả success đúng commit; nếu môi trường không có Cargo, để unit tests pending; user cần thử giao diện/phát tiếng và Save/Open GĐ3A trên URL bất biến mới. Không merge production, không Actions/TinyFish/phí hoặc CMD máy chủ dự án.


## ORVIA 3B — Cập nhật 23/10/2026

Đã đối chiếu nhật ký trung tâm Drive, nhật ký repo, AGENTS, ROADMAP, gaps, UI plan và source trước khi tiếp tục. Kế thừa GĐ3A đã deploy ở `cd61200`. Nhánh độc lập `feature/orvia-3b` triển khai: MIDI Editor gọi `controller_lane::show`, các làn Velocity, CC1/7/10/11/64 chỉnh `Sequence.ctrls` thật; engine có `midi.cc_set`/`midi.cc_delete`, kiểm tra giá trị và khóa clip; unit regression có Undo/Redo, Save/Open. QA chi tiết tại `docs/ORVIA-WEB-STAGE3B-QA.md`. Đã mở Draft PR #11: https://github.com/CapCuuStudio/soundcraft/pull/11 để Cloudflare preview build, **chưa xác nhận build hoặc Chrome UAT / Cargo tests thành công**. Không đổi main/production hay bản 3A; không dùng Actions, TinyFish, phí mới, CMD người dùng. Trước nghiệm thu: cần Cloudflare build đúng SHA, UAT CC64/velocity/CC và hồi quy âm thanh/Save/Open/MIDI export. Tín hiệu pedal nghe được phụ thuộc synth.

**Cloudflare 3B (23/10/2026):** PR #11 Pages bot xác nhận **Deploy successful** đúng commit `07a569da5f01aa223706fab0547a0e73354c44b4` chứa toàn bộ mã 3B và tài liệu QA, preview bất biến https://551191aa.orvia-daw-preview.pages.dev, branch preview https://feature-orvia-3b.orvia-daw-preview.pages.dev. Commit kế tiếp `2c6c972` chỉ cập nhật nhật ký kỹ thuật (compare đã xác nhận), không sửa mã UI/engine. Browser không truy cập được Cloudflare từ công cụ (DNS), chưa test trực tiếp các thao tác; Rust Cargo tests chưa chạy. Giữ PR nháp, chờ user UAT trên trình duyệt và QA native; không merge production.


## ORVIA WEB — Giai đoạn 3C: Instrument & FX Rack (23/10/2026)

**Yêu cầu:** chủ dự án yêu cầu “tiếp tục làm 3C”. Trước khi sửa đã đọc nhật ký trung tâm LUNOR MUSIC Google Drive, log repo này, QA 3B, docs/ORVIA-CUBASE-UI-PLAN.md, AGENTS.md, ROADMAP.md, docs/gaps.md và source `panels.rs`, `docking.rs`, `mix_window.rs`, `cmd/instrument.rs`, `cmd/mix.rs`, `dsp/plugins/mod.rs`, `sampler_window.rs`. Baseline từ nhánh GĐ3B; Cloudflare GĐ3B đã xác nhận build PASS cho mã 3B `07a569d`; Rust unit tests và Chrome browser UAT GĐ3B chưa được người dùng xác nhận, không coi là nghiệm thu.

**Phát hiện:** GĐ2 đã có Browser rất sơ khai liệt kê `soundcraft_dsp::plugins()` và gọi `mix.instrument`/`mix.insert`, nhưng không chọn Track hoặc Insert slot một cách rõ ràng, thiếu tình trạng các inserts đang dùng, Bypass, xóa, chỉnh params từ rack; dễ khiến người dùng hiểu nhầm nhạc cụ web hỗ trợ native VST3/Kontakt. Cửa sổ plugin và engine âm thanh thật đã có, nên tái sử dụng chứ không dựng giả lập.

**Triển khai:** nhánh riêng `feature/orvia-3c-plugin-rack`, không sửa GĐ3B/GĐ3A/main. Thêm `crates/ui-egui/src/panels/orvia_rack.rs`, đổi `orvia_browser` trong `panels.rs` thành wrapper dùng module này: chọn Track, tạo Instrument Track, tìm và lọc plugin built-in theo nhóm, hiển thị nhạc cụ đang gán, kiểm soát slot Insert trống, Add/BYPASS/Remove effects qua engine commands thật, mở dialog tham số bằng `app.ui.plugin_windows`. Nếu nhạc cụ đang chọn giống plugin hiện tại, bấm “Chỉnh” chỉ mở cửa sổ và không reset thông số. Sampler báo cần tải sample; Chrome WASM chỉ cho plugin DSP built-in, không quảng cáo host VST3/Kontakt Windows native. Test unit cho đúng registry/categorization và an toàn Track type/Insert slot; không đổi DSP, session schema, MIDI, transport hay Cloudflare scripts. QA và cổng nghiệm thu tại `docs/ORVIA-WEB-STAGE3C-QA.md`.

**Cổng chất lượng:** mã committed không phải PASS. Cần Draft PR riêng với bot Cloudflare xác nhận build đúng commit, và người dùng kiểm thử Chrome Track/Instrument/FX/Bypass/Remove/Save/Open/Play, cùng hồi quy 2C/3A/3B. Rust Cargo tests hiện chưa chạy vì sandbox thiếu Cargo; không khẳng định là PASS. Không dùng TinyFish, GitHub Actions tự phát, trả phí, CMD người dùng; không merge production.


## ORVIA 3D — 2026-10-23

Đã đọc nhật ký Drive chung và nhật ký repo, AGENTS, ROADMAP, gaps, QA 3C, kế hoạch UI và mã Track Automation. GĐ3C xác nhận Cloudflare build PASS đúng 4a8f1cf, chưa nghiệm thu Chrome/Cargo. GĐ3D được chọn để hoàn thành Track Automation khác MIDI CC 3B. Nhánh feature/orvia-3d-track-automation kế thừa 3C, Draft PR #13. Mã: module panels/orvia_automation.rs tích hợp Inspector trong panels.rs, thêm/chỉnh/xóa Volume/Pan/Mute automation tại cursor và điểm hiện hữu qua lệnh track.view, automation.set_point, automation.clear, automation.write_range thật; sử dụng Track::automation, hỗ trợ Undo/Redo và Save/Open không đổi schema. QA: docs/ORVIA-WEB-STAGE3D-QA.md. Chưa xác nhận build Cloudflare 3D hoặc Chrome audio/SaveOpen và Rust tests tại thời điểm viết. Giữ main/production và các giai đoạn cũ nguyên vẹn, không Actions/TinyFish/phí/CMD Windows.


## ORVIA 3D nghiệm thu người dùng — 23/10/2026

Người dùng đã xác nhận “xong rồi”. GitHub PR #13 vẫn Draft, Cloudflare Pages Deploy successful đúng commit 6f192bc9b454f321d5bf248471b320f014785c8c, preview https://61fdbe3b.orvia-daw-preview.pages.dev. Ghi nhận xác nhận hoàn thành bản thử web 3D nhưng không suy diễn từng hạng mục QA đã được chạy riêng. Rust Cargo unit tests chưa xác nhận PASS, không merge main hoặc production. Giai đoạn 4 tiếp theo là polish giao diện, responsive, phím tắt và kiểm thử hồi quy audio/MIDI; chỉ triển khai khi có yêu cầu mới.


## ORVIA GĐ4A.1 — Nâng cấp Inspector (2026-10-23)

**Quyết định:** Người dùng đã xem kế hoạch GĐ4 và phê duyệt “ok làm 4A1 đi”; ảnh chụp giao diện 1738×863 cho thấy Inspector chỉ khoảng 200px, Mute/Solo, Volume/Pan và Automation dồn/chữ bị cắt. Đây là nhánh polish UI, không phải thay audio engine. Đã đọc nhật ký kỹ thuật Google Drive trung tâm và repo, AGENTS.md, ROADMAP.md, docs/gaps.md, docs/ORVIA-CUBASE-UI-PLAN.md, QA 3D và source thực trước khi sửa. Baseline GĐ3D `feature/orvia-3d-track-automation`: người dùng xác nhận hoàn tất bản thử, Cloudflare successful đúng `6f192bc`; nhật ký tiếp theo ở commit `5f33159` chỉ là docs; chưa có bằng chứng toàn bộ Rust unit tests PASS.

**Thay đổi trên nhánh riêng** `feature/orvia-4a1-inspector-layout`: ở `docking.rs`, mặc định width Inspector tăng 222px→270px, dải resize min190/max410px; workspace đã lưu vẫn được giữ, không tự ý reset. Ở `panels.rs`, Track name có wrap và thanh màu; các nút Mute/Solo/Ghi âm/Monitor 2 cột, Rename/Add Track chiếm toàn chiều ngang; Volume/Pan hiện thông số tách khỏi slider (slider không còn chen vào chữ), Routing/Instrument/Insert/Send/Automation là các nhóm, Insert mặc định thu gọn để tránh chiếm chỗ. `panels/orvia_automation.rs` đưa nút chọn lane, thêm điểm, ghi vùng chọn và slider xuống từng hàng, giúp đọc được khi sidebar hẹp; thao tác vẫn gửi engine command thật của 3D. Thêm unit regression kiểm tra kích thước mặc định và khả năng resize. QA kiểm tra và hướng dẫn legacy workspace ở `docs/ORVIA-WEB-STAGE4A1-INSPECTOR-QA.md`.

**Bảo toàn:** Không sửa audio/MIDI DSP, waveform, Timeline, session serialization, VST/Kontakt host, Web Build scripts, main/production hay thay đổi dữ liệu dự án. Các lệnh `mix.volume`, `mix.pan`, `mix.mute`, `mix.solo`, `mix.record_arm`, `mix.input_monitor`, `track.view`, `automation.set_point`, `automation.clear` và `automation.write_range` giữ nguyên.

**QA gate:** mã committed chưa đủ kết luận pass. Chỉ gửi link 4A1 sau Cloudflare Deploy successful **đúng SHA**; Rust Cargo tests và người dùng thử Chrome trên 1366×768/1440×900, gửi feedback. Không tự merge main/production, không GitHub Actions tự phát, TinyFish, trả phí hoặc CMD Windows.


## ORVIA GĐ4A.1 — Người dùng xác nhận xong, Cloudflare PASS (23/10/2026)

Người dùng thông báo “xong rồi” sau khi nhận bản Inspector 4A.1. Đã kiểm tra Cloudflare Pages check-run completed/success đúng SHA code 42d4bbc42f881582b3b45eeaa96e7a918b11435f (so sánh nhánh với SHA identical trước khi ghi nhật ký), immutable preview https://c63b6e8d.orvia-daw-preview.pages.dev và Draft PR #14 https://github.com/CapCuuStudio/soundcraft/pull/14. Ghi nhận người dùng xác nhận 4A.1 hoàn tất bản thử và giao diện đã sẵn sàng làm mốc 4A.2; không suy diễn checklist Chrome/âm thanh/Undo/SaveOpen đã được chạy đủ từng trường hợp. Các bài Rust Cargo tests chưa có chứng cứ PASS. Không merge main/production, không tự chạy Actions, không TinyFish, phí hoặc CMD Windows. 4A.2 tiếp theo: tinh chỉnh Toolbar khi người dùng yêu cầu.


## ORVIA GĐ4A.2 — Toolbar hai hàng (2026-10-23)

**Yêu cầu:** Người dùng phê duyệt 4A.2 tiếp theo bản Inspector 4A.1 đã xác nhận hoàn thành. Trước khi thao tác đã đọc nhật ký kỹ thuật trung tâm LUNOR MUSIC (Google Drive), nhật ký repo, AGENTS, ROADMAP, gaps, Cubase UI plan, QA 4A1 và mã thực `crates/ui-egui/src/toolbar.rs` + nơi hiển thị toolbar trong `edit_window.rs`. Baseline GĐ4A1 `feature/orvia-4a1-inspector-layout`: Cloudflare deployed mã 42d4bbc, người dùng xác nhận hoàn tất; các commit sau là nhật ký. Không tự merge base/main/production.

**Vấn đề:** Toolbar cũ cao 78px, gồm các nhóm Transport/Counter/Tempo/Edit Modes/Tools/Grid-Nudge/Zoom/Output Meter kéo trên cùng một hàng với scroll ngang; ở màn hình 1366px trở xuống nút Grid, Tools, Zoom khó tìm vì bị đẩy về bên phải. Bố cục và khả năng thao tác là phạm vi GĐ4A2, không phát triển audio engine mới.

**Triển khai:** nhánh `feature/orvia-4a2-toolbar-layout`. Sửa `toolbar.rs`: HEIGHT 78→140px và chia thành hai hàng độc lập có horizontal ScrollArea riêng. Hàng 1 `PHÁT / NHỊP`: Transport, Main/Sub counter, CountOff/Click/Tempo/Meter, Grid/Nudge, Level Meter. Hàng 2 `CHỈNH SỬA`: Edit Modes, Smart/Tool palette, Zoom, nhãn tên công cụ đang chọn. Nút vận chuyển tăng rộng 38→42px. `edit_window.rs` vốn dùng hằng `toolbar::HEIGHT` nên không cần chỉnh. Không thay cách gọi lệnh Play/Stop/Record, Tempo, Grid, Nudge, Tools, Snap, Zoom, Click/CountOff, không đụng DSP, session, waveform, MIDI, nhạc cụ/VST, Inspector, Mixer, project Save/Open hoặc Export.

**Kiểm thử:** bộ QA cho 1366×768, 1440×900, màn hình hẹp và regression controls tại `docs/ORVIA-WEB-STAGE4A2-TOOLBAR-QA.md`. Chỉ kết luận web build PASS nếu Cloudflare Pages Deploy successful đúng SHA và lấy immutable preview từ bot. Cargo unit tests/Chrome tương tác âm thanh và xuất MIDI không được tự đánh dấu PASS. Giữ Draft PR, không merge production/main, không tự chạy GitHub Actions, không TinyFish, dịch vụ phí hoặc CMD Windows.


## ORVIA 4A.2 — Chốt bản web thử ngày 23/10/2026

Người dùng xác nhận “xong”. Cloudflare Pages Deploy successful đúng mã 933b6a35560294ea8bce838befb1395aa1cb32fc, preview https://1446184d.orvia-daw-preview.pages.dev; GitHub Draft PR #15. Xác nhận hoàn thành bản thử 4A.2, nhưng chưa có kết quả Rust Cargo unit tests và biên bản Chrome QA chi tiết. Không merge main/production, không Actions/TinyFish/phí/CMD Windows. Bước đề xuất sau là 4A.3 Rack, chỉ khi có yêu cầu mới.


## ORVIA — Hotfix Toolbar 4A.2 responsive theo ảnh phản hồi (2026-10-23)

Người dùng gửi ảnh đánh dấu toàn bộ Toolbar hai hàng bằng đường đỏ và vùng khoảng trống lớn bên phải bằng đường vàng. Nhận diện: GĐ4A2 cố định chiều cao 140px ở mọi chiều ngang dù trên desktop rộng nội dung có thể vừa 1 hàng; Timeline bị mất chiều cao không cần thiết. Trước sửa đã đọc log kỹ thuật Drive chung và repo, AGENTS, roadmap, gaps, UI plan, QA 4A2 cùng source toolbar.rs và edit_window.rs. Tạo nhánh **fix/orvia-4a2-responsive-toolbar** từ mốc 4A2 đã Cloudflare PASS và người dùng xác nhận; không đụng nhánh gốc. Chỉnh toolbar.rs thêm height_for_width: >=1550 logical px dùng 1 hàng cao 78px, thứ tự Transport→Counter→Tempo→Grid→Edit Mode→Tools→Zoom→Meter; màn hình hẹp dùng 2 hàng 140px và cuộn độc lập. Edit window lấy kích thước TopPanel trực tiếp từ hàm trên. Đã đối chiếu giữ nguyên cả 20 lệnh app.run/execute_merged trong file Toolbar; thêm unit test breakpoint. QA: docs/ORVIA-WEB-STAGE4A2-RESPONSIVE-HOTFIX-QA.md. Cổng kiểm thử: chờ Cloudflare Deploy successful đúng commit hotfix mới gửi link, Chrome thực tế và Cargo Rust tests chưa xác minh. Không sửa DSP/audio/MIDI/session/production, không merge main, Actions tự phát/TinyFish/phí/CMD Windows.


## ORVIA 4A.2 responsive — người dùng xác nhận hoàn thành (23/10/2026)

Người dùng trả lời “xong” sau bản sửa Toolbar từ ảnh phản hồi. Đã xác minh Cloudflare Pages check-run completed/success đúng SHA daad2c84a10edd074dd5fe41afb4a09da8504669; GitHub compare nhánh fix/orvia-4a2-responsive-toolbar với SHA này là identical trước khi ghi nhật ký. Preview bất biến: https://da537bda.orvia-daw-preview.pages.dev. Draft PR #16: https://github.com/CapCuuStudio/soundcraft/pull/16 chưa merge main/production. Ghi nhận user chốt bản thử hotfix 4A.2: Toolbar 78px một hàng nếu đủ chiều rộng và 140px hai hàng khi hẹp, giữ đủ lệnh điều khiển. Không suy diễn Rust Cargo tests đã PASS hoặc toàn bộ QA audio/MIDI regression được xác minh riêng. Cập nhật chỉ nhật ký, không chỉnh mã sản phẩm; không tự chuyển sang giai đoạn 4A.3.


## ORVIA Design Language v1.0 — Thiết kế hướng thay đổi vừa phải (2026-10-24)

**Nguồn yêu cầu:** Chủ dự án chọn “thay đổi vừa phải” và yêu cầu thực hiện Bước 1: chốt Design Language ORVIA gồm màu nền/nhấn, nút, slider, panel, tab, track header, font và cỡ chữ. Phạm vi **chỉ là tài liệu quy chuẩn**, không được tự ý đổi giao diện, engine hoặc merge production trong bước này. Đã đọc nhật ký LUNOR MUSIC trên Google Drive, nhật ký kỹ thuật repo, AGENTS.md, ROADMAP.md, docs/gaps.md, UI plan, mã `theme.rs`, `widgets.rs`, `docking.rs`, `panels/orvia_rack.rs`, `fonts.rs`, `icons.rs`. Baseline hiện hành: nhánh `fix/orvia-4a2-responsive-toolbar`, mã web đã Cloudflare PASS `daad2c8` và người dùng xác nhận; `e43658e` là commit nhật ký, không thay mã.

**Nội dung giao nộp:** nhánh `design/orvia-design-language-v1` tạo từ baseline; tài liệu `docs/ORVIA-DESIGN-LANGUAGE-V1.md` mô tả chính xác 18 semantic color tokens (Graphite/Cyan, contrast >=4.5:1 cho text), font có sẵn (Be Vietnam Pro web, OS font desktop), font sizes, density/spacing/radius, button states, slider, panel, tab, Track/Timeline, browser Rack, mapping đến Rust/egui, QA gates và những phần giữ nguyên. Thêm `docs/ORVIA-DESIGN-TOKENS-V1.json` đọc máy với palette/geometry/breakpoint/bảo toàn engine. Đã kiểm tra file JSON parse thành công và 18 màu trong JSON đều có trong Markdown, không sửa code hoặc assets. Không tự thêm font file, không copy thành phần độc quyền từ Cubase/Pro Tools, không đổi Light/System theme. **Chưa triển khai palette vào runtime, chưa có Cloudflare preview v1 vì thay đổi chỉ là tài liệu**; xin phản hồi thiết kế trực quan nếu cần trước khi áp vào 4A3.

**Tiếp theo:** chỉ triển khai 4A.3 Rack theo spec này sau khi chủ dự án yêu cầu; code mới trên nhánh riêng, Cloudflare build đúng SHA và Chrome UAT trước nghiệm thu. Không merge main/production, không Actions tự phát, TinyFish, dịch vụ trả phí hay CMD Windows.


## ORVIA Web 4A.3 — Studio Rack (24/10/2026)

Người dùng đồng ý áp dụng Design Language v1 vào Rack nhạc cụ/hiệu ứng. Đã đọc và đối chiếu nhật ký trung tâm LUNOR MUSIC trên Drive, nhật ký repo, AGENTS, ROADMAP, gaps, bản thiết kế/tokens và source Rust/egui. Base `design/orvia-design-language-v1` trên Toolbar 4A2 đã được người dùng chốt. Nhánh riêng `feature/orvia-4a3-studio-rack`, Draft PR #18 https://github.com/CapCuuStudio/soundcraft/pull/18. Mã sửa giới hạn `panels/orvia_rack.rs`, `panels/rack_skin.rs`, `panels.rs`: màu Graphite/Cyan riêng cho Rack và Light theme giữ nguyên, nhận diện ORVIA Studio Rack, tab lớn, mặc định Audio Track→Hiệu ứng/Instrument Track→Nhạc cụ khi chưa có lựa chọn, Track đích/card plugin/Insert dễ nhìn hơn, Bypass/Bật lại và Gỡ khỏi Rack có nhãn. Reuse engine commands cũ, không sửa DSP/audio/MIDI/Session, không dùng VST3/Kontakt native trên Web. Cloudflare build và Chrome UAT phải xác minh riêng, Cargo Rust tests chưa chạy. Không merge main/production, không Actions tự phát, TinyFish, phí hoặc CMD Windows.

**4A.3 — rà soát Paint Scope:** Sau khi mở PR #18 đã bỏ lệnh tô `ui.max_rect()` trong Rack để không phủ lên header Browser; thay đổi code ở commit `3a141ca`, các token của thẻ/nút và lệnh MIDI/audio giữ nguyên. Cloudflare bản sửa mới chưa được xác nhận tại thời điểm ghi. Chỉ nghiệm thu sau Deploy successful đúng SHA code và browser UAT.


## ORVIA 4A.3 — Người dùng xác nhận bản thử, Cloudflare PASS (24/10/2026)

Người dùng nhắn “xong” sau khi thử giao diện Studio Rack mới. Đã đối chiếu lại nhật ký trung tâm LUNOR MUSIC trên Drive và nhật ký repo, GitHub PR #18 vẫn Draft/chưa merge, Cloudflare Pages check-run `completed/success` đúng SHA `3149e4722aeb5e45f7f4ec07f8e963c606e86066`, `compare` HEAD của `feature/orvia-4a3-studio-rack` là `identical` tại thời điểm kiểm tra. Preview bất biến: https://efde69f9.orvia-daw-preview.pages.dev. Ghi nhận người dùng chốt **bản thử web GĐ4A.3**, gồm Studio Rack Graphite/Cyan, tab, thẻ Track/plugin, nút Bypass/Gỡ; không suy diễn rằng Chrome UAT từng hạng mục, Cargo unit tests hoặc kiểm thử âm thanh/MIDI hồi quy độc lập đã PASS. Commit cập nhật nhật ký chỉ đổi tài liệu, không sửa runtime. Giữ bản 4A.3 và các nhánh trước nguyên vẹn; không tự chuyển 4A.4, không merge main/production, không GitHub Actions tự phát, TinyFish, dịch vụ phí hoặc CMD Windows.


## ORVIA 4A.4 — Track Header / Timeline Design Language v1 (24/10/2026)

**Quyết định người dùng:** “ok làm đi” cho 4A.4, tiếp theo GĐ4A.3 Studio Rack đã Cloudflare Deploy successful mã `3149e47`, người dùng xác nhận hoàn tất. **Trước khi sửa** đã đọc và đối chiếu nhật ký kỹ thuật chung LUNOR MUSIC Google Drive, nhật ký repo, AGENTS.md, ROADMAP.md, docs/gaps.md, tài liệu Design Language v1, kế hoạch Cubase UI và QA 4A.3; sau đó kiểm tra mã thực `crates/ui-egui/src/edit_window.rs`, `theme.rs`, icons, widgets và khu Track/Header/Clip/Grid.

**Nhánh:** `feature/orvia-4a4-track-timeline`, tách riêng từ `feature/orvia-4a3-studio-rack`. Không sửa 4A.3 gốc/main/production.

**Scope UI-only:** ở `crates/ui-egui/src/edit_window.rs` thêm semantic palette cục bộ theo ORVIA v1, track selected rail cyan 3px vẽ **sau** header meter để không bị che, nền field Track name selected bo 4px + chữ tương phản Dark/Light; Mute/Solo/Record/Input monitoring chuyển từ nút 3D sang nút phẳng bo 4px với màu chức năng (Dark: cyan, amber, coral, green), vùng bấm và command vẫn nguyên; clip border graphite, selected stroke cyan 1.5px và góc clip/name bar bo 4px; bar/grid colors Dark `#405363`/`#293744`, Light giữ theme trước. `theme::clip_colors` và Track::color, Clip::color, waveform/MIDI data không thay. Các hàm xử lý edit/transport/undo không sửa. Thêm unit test kiểm tra palette. QA checklist `docs/ORVIA-WEB-STAGE4A4-TIMELINE-QA.md`.

**QA gate:** Chỉ được cung cấp 4A.4 immutable preview sau Cloudflare Pages *Deploy successful đúng SHA*, phải thử Track selection/multi-selection, mute/solo/arm/monitor, clip/fade/crossfade, MIDI/Save/Open, Light/Dark Chrome (1366×768, 1440×900, 1738×863). Cargo unit tests và Chrome UAT chưa có bằng chứng PASS, không tự gán. Không chạy GitHub Actions tự phát, TinyFish, dịch vụ tính phí, CMD Windows, merge main/production.


## ORVIA 4A.4 — Người dùng chốt bản thử và Cloudflare PASS (2026-10-24)

Người dùng xác nhận “xong” sau khi thử bản 4A.4 Track Header / Timeline. Đã đọc và đối chiếu nhật ký kỹ thuật repo và nhật ký chung LUNOR MUSIC trên Google Drive **trước thao tác cập nhật**, đồng thời kiểm tra PR #19, Cloudflare Pages check-run và so sánh HEAD thực của nhánh.

**Xác minh:** Cloudflare Pages `completed/success` đúng commit `e383ebc81dc27967f62604a1285653b06cb3609e`; so sánh nhánh `feature/orvia-4a4-track-timeline` với commit này `identical` ở thời điểm kiểm tra. Preview bất biến từ Cloudflare: https://91cc60f9.orvia-daw-preview.pages.dev. Draft PR #19 https://github.com/CapCuuStudio/soundcraft/pull/19 vẫn chưa merge main/production.

**Kết luận:** Người dùng đã chốt bản **web thử GĐ4A.4**, bao gồm Track Header selected cyan, phím Mute/Solo/Record/Monitor phẳng, viền Clip bo nhẹ và Grid Graphite/Cyan. Không suy diễn Cargo unit tests, Chrome QA chi tiết cho từng thao tác, audio/MIDI/Undo/Save/Open regression đều PASS khi chưa có chứng cứ riêng. Bản cập nhật nhật ký chỉ là docs-only, không chỉnh code hay sửa các bản đã nghiệm thu. Không tự chuyển sang giai đoạn 4B, không chạy GitHub Actions thủ công, không TinyFish, dịch vụ phí, CMD Windows hoặc merge production.

## ORVIA Web 4B.1 — Lower Zone Tabs (24/10/2026)

**Yêu cầu:** Chủ dự án duyệt “ok làm đi” cho 4B.1 — nâng cấp hàng tab dưới, không thay đổi engine hay những phần 4A.4 đã chốt. **Nguồn đối chiếu trước khi làm:** nhật ký kỹ thuật LUNOR MUSIC trên Google Drive (bản cập nhật 24/10), nhật ký repo này, AGENTS.md, ROADMAP.md, docs/gaps.md, Design Language v1, PR #19 (Draft, head docs-only `4eb63d8`, baseline code 4A.4 `e383ebc` Cloudflare PASS), mã `edit_window.rs` và `docking.rs`.

**Nhánh thử:** `feature/orvia-4b1-lower-tabs`, tách trực tiếp từ 4A.4 `4eb63d8ae9facfb43986539dc872fefcb98267c0`. Không merge main/production.

**Đã viết code UI-only:** `crates/ui-egui/src/edit_window.rs` tăng thanh footer `lower_dock_tabs` 22→32px, nhãn Tab 12pt dễ đọc, nền chọn Graphite + gạch cyan 2px, hover/focus rõ và màu Light riêng. Bốn tab MixConsole/Key Editor/Memory Locations/Undo History giữ engine command gốc, hai tab editor vẫn loại trừ nhau; vùng tab dùng egui 0.36 horizontal ScrollArea khi hẹp, `Đặt lại bố cục` giữ bên phải gọi `ui.dock.reset`. Thêm 2 Rust unit tests palette/height. Mã ứng dụng có commit `a5bb141` và chỉnh đường dẫn API scroll-area public `4b7611d`. Checklist tại `docs/ORVIA-WEB-STAGE4B1-LOWER-TABS-QA.md`, commit `69bdc99`. Không sửa mixer logic, DSP, MIDI, audio/session, VST3/CLAP hay Saved Workspace.

**Đã xác nhận:** GitHub ghi thành công các file/commit trên nhánh riêng; kiểm tra source giữ nguyên số vị trí gọi `app.run("ui.dock",...)` trong `edit_window.rs` (5 trước/sau). API `Ui::scope_builder`, `UiBuilder::max_rect`, `ScrollArea::scroll_bar_visibility` đối chiếu tài liệu egui 0.36. **Chưa có bằng chứng Cargo unit tests, build WASM Cloudflare, kiểm thử Chrome/audio/MIDI thành công** tại thời điểm ghi log. Không coi PR đã deploy/được nghiệm thu trước khi kiểm tra đúng HEAD.

**Bước kế tiếp:** tạo Draft PR nhánh 4B.1 base nhánh 4A.4, kiểm Cloudflare check-run đúng commit cuối, chỉ chia sẻ immutable preview nếu Deploy successful; sau đó UAT tab, responsive, focus, audio/MIDI và Save/Open. Không tự chạy Actions, không sử dụng TinyFish/phí mới/CMD trên máy chủ dự án.

## ORVIA Web 4B.1 — Chủ dự án xác nhận nghiệm thu bản thử (24/10/2026)

Chủ dự án phản hồi “xong” sau khi triển khai 4B.1. Ghi nhận nghiệm thu **tổng quát** giao diện Lower Zone tabs, không suy diễn đã kiểm từng kịch bản browser hoặc chất lượng âm thanh.

**Bằng chứng live:** Cloudflare Pages check-run #120813047115 cho commit mã và tài liệu `a57faeb8b1c4844d84981e8e7cddefa2eccee521` trả `status=completed, conclusion=success` và `Deploy successful`. Preview bất biến được Cloudflare công bố: https://357d9957.orvia-daw-preview.pages.dev ; nhánh Preview: https://feature-orvia-4b1-lower-tabs.orvia-daw-preview.pages.dev . PR #21 https://github.com/CapCuuStudio/soundcraft/pull/21 đang `open/Draft/unmerged`, base `feature/orvia-4a4-track-timeline`; không thay production.

**Chốt:** Thanh tab Lower Zone 32px, chữ 12pt, selected/hover/focus Graphite/Cyan và Light riêng, cuộn ngang, Reset Layout; giữ các command docking hiện có. Rust Cargo unit tests và kiểm thử Chrome audio/MIDI, Save/Open và responsive chi tiết chưa có bằng chứng PASS riêng; chỉ Cloudflare build PASS và phản hồi nghiệm thu tổng quát.

**Quyết định:** giữ nguyên source 4B.1 đã chốt, chỉ thêm nhật ký (docs-only). Chưa triển khai 4B.2, chờ chỉ đạo của chủ dự án. Không merge main/production, không tự chạy Actions hoặc dịch vụ mất phí/TinyFish/CMD Windows.

## ORVIA Web 4B.2 — MixConsole & Key Editor UI (24/10/2026)

**Chỉ đạo chủ dự án:** “ok làm đi” cho 4B.2. Trước thao tác đã đọc nhật ký trung tâm Google Drive (gồm nghiệm thu 4B.1), log repo này, AGENTS, ROADMAP, gaps, Design Language v1 và QA 4B.1; đối chiếu live GitHub PR #21 HEAD `259c49e1a1cd7b9deeec5a01b1e9877fd07233e4` và Cloudflare success mã `a57faeb8`. Đọc source thật `panels.rs` (MixConsole Stage 3A), `midi_editor.rs` (Key Editor) và `docking.rs` (splitter/layout persistence).

**Nhánh riêng:** `feature/orvia-4b2-mixer-key-editor`, xuất phát từ 4B.1 `259c49e1` (mã không đổi sau chủ dự án chốt). Không merge Main/Production.

**Đã triển khai UI-only:**
- `crates/ui-egui/src/panels.rs`: header MixConsole 13pt với ghi chú kéo divider, palette Graphite/Cyan Dark và Light riêng, track strip được chọn có border cyan, chữ Track/volume dễ đọc, Mute/Solo semantic buttons và khoảng cách kênh rộng hơn. Giữ nguyên fader, pan, meter, sends, route, engine command & Undo merge; code commit `2fd85dc7`.
- `crates/ui-egui/src/midi_editor.rs`: Key Editor chia header thành title/meta và hàng nút thật cao tổng 56px, Clip name dài được clip không che nút; giữ nguyên Quantize/Vel +/-10/Legato, note editing, note Track color. Grid/Piano Roll Dark #293744/#405363 và Light palette riêng; Việt hóa gợi ý trống. Code commit `3e59d1b9`.
- Thêm Rust unit tests palette/header và checklist `docs/ORVIA-WEB-STAGE4B2-MIXER-KEY-EDITOR-QA.md` commit `16a9dfa1`. Native `craft_ui::docking` splitter và saved workspace, MIDI `minimum_body` giữ nguyên, không thêm cơ chế resize song song.

**QA nguồn:** Đã đối chiếu số lượng/thứ tự engine invocation bên trong MixConsole và Key Editor trước/sau là tương đương (MixConsole 10, Key Editor 8), không sửa engine/DSP/MIDI/session/export, chưa có chứng cứ Cargo check/test, Cloudflare deploy hay Chrome audio UAT ở thời điểm ghi. Chưa tuyên bố hoàn thành nghiệm thu; kiểm tra check-run Cloudflare đúng commit mới và PR Draft trước khi giao preview.

**Bước tiếp theo:** Nếu Cloudflare build fail, điều tra/sửa đúng lỗi; chỉ giao URL bất biến được check-run xác nhận success. Giữ 4B.1 và production nguyên vẹn. Không GitHub Actions tự phát/TinyFish/phí/CMD Windows.

## ORVIA 4B.2 — Người dùng xác nhận nghiệm thu và Cloudflare PASS (24/10/2026)

Chủ dự án phản hồi “xong” sau khi xem bản Web 4B.2, ghi nhận **nghiệm thu tổng quát giao diện** MixConsole và Key Editor. Trước khi cập nhật đã đối chiếu nhật ký kỹ thuật repo/Google Drive, PR #22 và live GitHub Cloudflare check-run.

**Chứng cứ triển khai:** Cloudflare Pages check-run của `48d35a5856b95696019129fdbb5caae29c2948e2` trả `completed/success`, “Deploy successful”; immutable Preview https://c2216b68.orvia-daw-preview.pages.dev ; branch Preview https://feature-orvia-4b2-mixer-key.orvia-daw-preview.pages.dev . PR #22 https://github.com/CapCuuStudio/soundcraft/pull/22 vẫn `open/Draft/unmerged`, base `feature/orvia-4b1-lower-tabs`.

**Phạm vi chốt:** MixConsole và Key Editor đồng bộ Graphite/Cyan, layout tiêu đề/hàng nút Key Editor, strip/mute/solo Mixer và Light/Dark. Source UI-only không sửa DSP/MIDI/audio engine/session, docking saved layout, track/note color; vẫn chưa có xác nhận riêng Cargo Rust tests, Chrome QA chi tiết âm thanh/MIDI/Undo/Save/Open.

**Quyết định:** giữ nguyên code 4B.2 đã nghiệm thu, chỉ ghi nhật ký docs-only. Không merge main/production, không tự bắt đầu 4B.3. Không GitHub Actions thủ công/TinyFish/phí/CMD Windows.

## ORVIA Web — GĐ4B.3 Brand Mark / Icons / Control States (24/10/2026)

**Chỉ đạo:** chủ dự án yêu cầu “làm đi” cho 4B.3 — hoàn thiện biểu tượng, nhận diện thương hiệu và trạng thái nút. Trước triển khai đã đọc nhật ký trung tâm Google Drive, log repo này, AGENTS.md/ROADMAP.md/docs/gaps.md, ORVIA Design Language v1, PR #22 (4B.2 accepted, Cloudflare success code `48d35a5`, docs-only head `1ffa967`), và source thật `icons.rs`, `menus.rs`, `widgets.rs`, `toolbar.rs`, `edit_window.rs` cùng cơ chế theme.

**Nhánh:** `feature/orvia-4b3-brand-control-states` tách từ `feature/orvia-4b2-mixer-key-editor` tại commit `1ffa9672f03985a3bc1fad44d1300ea4e40cf1de`. Không sửa/merge 4B.2, main, production.

**Code UI-only đã lưu:**
1. `icons.rs`: biểu tượng nguyên bản `orvia_mark` (sound-ring/wave) tự vẽ procedural bằng egui Painter, không dùng logo/hình ảnh/copyright từ Cubase/Pro Tools.
2. `menus.rs`: wordmark menu ORVIA được bổ sung icon 16px bên cạnh, chữ/menu/actions giữ nguyên.
3. `widgets.rs`: toolbar `icon_button` từ kiểu nổi 3D sang flat Graphite/Cyan, các trạng thái normal/hover/pressed/selected/focus phân biệt, selected underline 2px, Light palette độc lập. Giữ command dispatch của callers; những widget không thuộc toolbar không đổi.
4. `toolbar.rs`: Play/Stop/Record dùng flat semantic painter, Play active cyan, Record active coral, Stop neutral; không đổi event handlers, shortcuts, toolbar responsive rộng 78px/hẹp 140px hoặc phím bật Loop/Seek. Thêm unit tests palette và trạng thái.
5. Checklist `docs/ORVIA-WEB-STAGE4B3-BRAND-CONTROLS-QA.md`. So sánh source trước/sau xác nhận số engine/control call không đổi ở mỗi file có lệnh (widgets 3, toolbar 24, menus 10).

**Chứng cứ/giới hạn:** thay đổi đã ghi lên GitHub qua connector. Rust Cargo tests, Cloudflare deploy HEAD, Chrome playback/MIDI/UI keyboard UAT chưa được xác minh ở thời điểm ghi; không tuyên bố PASS. Giữ nguyên các bước 4A.2–4B.2 đã nghiệm thu; không động session/DSP/audio/MIDI/editor/VST3/driver.

**Tiếp theo:** tạo PR Draft vào base `feature/orvia-4b2-mixer-key-editor`; kiểm Cloudflare Deploy successful đúng SHA. Nếu build fail, sửa theo chứng cứ; chỉ sau success mới giao URL Preview 4B.3 bất biến để chủ dự án nghiệm thu. Không tự chạy Actions, dịch vụ phí, TinyFish, CMD Windows.

## ORVIA 4B.3 — Nghiệm thu Web Preview và Cloudflare PASS (24/10/2026)

**Người dùng:** phản hồi “xong” sau triển khai GĐ4B.3, ghi nhận nghiệm thu tổng quát **bản Web thử nghiệm** đối với logo ORVIA, biểu tượng Toolbar và trạng thái Play/Stop/Record. Không suy diễn chủ dự án đã kiểm thử từng trường hợp tương tác hay audio/MIDI regression.

**Kiểm chứng live trước cập nhật:** PR #23 https://github.com/CapCuuStudio/soundcraft/pull/23 đang `open/Draft/unmerged`, nhánh `feature/orvia-4b3-brand-control-states`, mã head `d7dd37dce701752d9b7b91005d68314e10459728` và base `feature/orvia-4b2-mixer-key-editor`. Cloudflare Pages check-run trên **chính SHA này** trả `completed/success` (completed 2026-10-24T10:47:39Z), `Deploy successful`. Preview bất biến: https://38e413d4.orvia-daw-preview.pages.dev ; branch Preview https://feature-orvia-4b3-brand-cont.orvia-daw-preview.pages.dev .

**Chốt phạm vi:** `orvia_mark` được vẽ procedural; toolbar buttons Graphite/Cyan có hover/pressed/selected/focus; transport có Play active cyan, Record coral, Stop neutral; Light/Dark riêng, giữ nguyên chiều cao toolbar 78/140px và engine commands. Không có bằng chứng Cargo Rust unit tests hay Chrome UI/audio/MIDI QA độc lập đã PASS; Cloudflare build thành công không thay thế kiểm thử đó.

**Quyết định:** bảo toàn source 4B.3, cập nhật docs-only để ghi nghiệm thu; không merge main/production, không tự triển khai 4B.4 hoặc đổi các bản đã chốt, không Actions thủ công, TinyFish, dịch vụ phí, CMD Windows.

## ORVIA Web 4B.4 — Responsive Audit & Smoke Gate (24/10/2026)

**Chỉ đạo:** chủ dự án duyệt “ok làm đi” GĐ4B.4 để kiểm thử tổng thể và hoàn thiện responsive từ 4B.3. Trước sửa đã đọc nhật ký trung tâm Google Drive (bao gồm nghiệm thu 4B.3), nhật ký repo, AGENTS.md, ROADMAP.md, docs/gaps.md, Design Language v1 và 4B.3 QA; đối chiếu PR #23 head docs-only `6cc460e6a52a7a18de5188e9e51b8f5367733d4e`, Cloudflare build successful code `d7dd37d`. Đọc source thật `panels.rs`, `panels/orvia_rack.rs`, `docking.rs`, `midi_editor.rs`, `edit_window.rs`, `toolbar.rs`, `scripts/cloudflare-pages-build.sh`.

**Nhánh thử:** `feature/orvia-4b4-responsive-qa` tách đúng từ docs-only HEAD 4B.3 `6cc460e`. Giữ nguyên source 4B.3, production, không merge.

**Lỗi responsive và khắc phục bằng mã:**
1. `panels.rs`: MixConsole panel có min-height 180 nhưng scroll chỉ ngang, fader/pan/send/output có thể bị cắt theo chiều dọc. Thay bằng `ScrollArea::both()` + scrollbar VisibleWhenNeeded. Giữ nguyên filter theo Track, fader/meter/pan, engine commands và history merge.
2. `panels.rs` + `panels/orvia_rack.rs`: Rack header, chọn Track, Insert/FX có thể nằm ngoài viewport khi Browser min-height 160 vì chỉ catalog có vùng cuộn. Đặt một ScrollArea dọc ở Browser bao trọn Rack; đổi nested ScrollArea catalog thành container vertical, giữ handler, chức năng gán hiệu ứng và nhạc cụ nguyên trạng.
3. `tests/orvia-4b4-responsive-contract.mjs`: thêm 7 nhóm source contract cho MixConsole, Studio Rack, Inspector, Key Editor, Toolbar 78/140px, Timeline/docking commands và brand/control states. `scripts/cloudflare-pages-build.sh` bắt buộc Node script PASS trước khi Rust toolchain + Trunk WASM build. Node built-ins, không thêm dịch vụ/trình duyệt hoặc package phí.

**Bằng chứng đã có:** GitHub ghi thành công các file; so số call `app.run` trong 2 file P1 trước/sau giữ nguyên (panels.rs 35, orvia_rack.rs 5). Đối chiếu 8 nhóm ký hiệu source/docking ở HEAD mới khớp (preflight source review); chưa được quyền đồng nhất preflight này với Node chạy trên Cloudflare. Checklist đầy đủ ở `docs/ORVIA-WEB-STAGE4B4-RESPONSIVE-QA.md`.

**Giới hạn và việc tiếp:** Cloudflare check-run/preview 4B.4 cần kiểm đúng HEAD cuối; Cargo unit tests, ảnh thật Chrome nhiều viewport, actual playback, MIDI/note editing, Save/Open và plugin runtime chưa xác minh PASS. Không tự tuyên bố nghiệm thu; chỉ trả Preview bất biến Cloudflare xác nhận success. Không GitHub Actions thủ công, không TinyFish, phí mới, CMD Windows hoặc merge main/production.

## ORVIA 4B.4 — Owner Acceptance & Verified Cloudflare Deploy (24/10/2026)

**Người dùng:** phản hồi “xong” cho GĐ4B.4 (responsive tổng thể), được ghi nhận là **nghiệm thu tổng quát bản Web thử nghiệm**. Không tự suy diễn mọi kiểm thử manual Chrome, nghe audio, MIDI hay Save/Open đều đã PASS.

**Xác minh thực tế trước khi chốt:** PR #24 https://github.com/CapCuuStudio/soundcraft/pull/24 tại commit mã `282639bc37e75cbf81aa384d1639044dd3103f53`, base `feature/orvia-4b3-brand-control-states`, vẫn `open/Draft/unmerged`. Cloudflare Pages check-run #120856664504 đúng code SHA này trả `completed/success`, hoàn thành 2026-10-24T11:21:26Z, `Deploy successful`. Immutable Preview chính thức https://ef7761c3.orvia-daw-preview.pages.dev ; branch preview https://feature-orvia-4b4-responsive.orvia-daw-preview.pages.dev . Script build được sửa để chạy Node source-contract trước Rust/Trunk, nên check-run thành công chứng thực build pipeline không exit lỗi; không có log test chi tiết độc lập từ Chrome/Cargo.

**Mã được chốt không đổi:** MixConsole cuộn cả ngang/dọc trong Lower Zone hẹp, Studio Rack cuộn toàn bộ Browser/Track/Insert/catalog; giữ lệnh thực, các giai đoạn 4A–4B.3, breakpoint Toolbar, saved docking và dữ liệu người dùng. Các QA source contract/Cloudflare build PASS ở phạm vi nêu trên, không coi đó là kết quả thực nghiệm playback/recording/Chrome nhiều viewport, Rust Cargo unit tests hoặc audio/MIDI regression.

**Quyết định:** chỉ cập nhật nhật ký nghiệm thu docs-only; giữ nguyên code 4B.4 đã chốt và không merge main/production. Chưa tự bắt đầu giai đoạn khác, không sử dụng GitHub Actions thủ công/TinyFish/phí/CMD Windows.
