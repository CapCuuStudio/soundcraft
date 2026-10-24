# ORVIA 4C.2 — Windows Portable Data Isolation & QA Staging

**Ngày:** 24/10/2026
**Cơ sở:** 4C.1 Draft PR #25, code `20d0568` Cloudflare Pages `completed/success` at immutable Preview https://db38f2e0.orvia-daw-preview.pages.dev . Web Preview không phải Windows Native PASS.
**Tình trạng:** implementation + source contract; **không có bản cài, không có executable Windows verified, NOT RELEASE READY**.

## Vì sao phải làm 4C.2 trước MSI?

Chủ dự án chọn Windows trên laptop cá nhân và không muốn thay đổi ổ C:; ưu tiên chạy thử trên E: hoặc F: mà không nhập CMD. Bản SoundCraft gốc có per-machine MSI ở Program Files, preferences theo `%APPDATA%/SoundCraft`, Windows executable/icon/metadata SoundCraft, không an toàn khi phát hành mang tên ORVIA.

4C.2 không bật release workflow, không đổi WiX/MSI gốc, không cài đặt hay khởi chạy từ máy của chủ dự án.

## Đã bổ sung mã thật

1. `apps/soundcraft/src/config.rs`: chỉ **Windows**, phát hiện file marker `.orvia-portable` **ngay bên cạnh executable**; khi có marker, **canonicalize đường dẫn thư mục thật để phát hiện junction/symlink dẫn về ổ hệ thống**, rồi kiểm tra không nằm trên system drive (`SystemDrive`, mặc định C:), chuyển thư mục cấu hình ứng dụng về `<exe folder>/ORVIA-Data`. Đây là đường dẫn chung cho settings `ui.json`, Autosave, Presets và logs trong logic Desktop hiện tại. Không thay `.scraft`, Session schema, MIDI, audio engine, DSP, plugin host hoặc dữ liệu cũ. Thêm Rust unit test E:/F:, system drive, relative/UNC và Windows verbatim path.
2. `apps/soundcraft/src/main.rs`: kiểm tra marker **trước khi attach file logger hoặc load preferences**; nếu marker sai định dạng/nằm trên ổ hệ thống thì ngừng khởi động và hiện **GUI error dialog Windows**, không im lặng chuyển về `%APPDATA%` trên C:. Không đổi cách mở bình thường khi marker vắng mặt.
3. `scripts/orvia-4c2-stage-windows-qa.ps1`: script staging **nội bộ, chưa chạy**. Chỉ nhận đường dẫn output trên **ổ E: hoặc F:**, từ chối thư mục đích đã tồn tại, xác nhận `ExpectedSha256` của binary, kiểm PE Machine x64 `0x8664` & GUI Subsystem `2`, kiểm CLI x64 console Subsystem `3` nếu có. Chỉ sau tất cả kiểm mới copy executable được kiểm vào thư mục đích, tạo marker, kèm LICENSE/NOTICE/ATTRIBUTION nếu có, và ghi JSON rõ `INTERNAL QA ONLY`. Không sửa registry, không build MSI, không gọi Actions, không chạy app.
4. `tests/orvia-4c2-portable-contract.mjs`: sáu nhóm static source contract cho marker/data path, GUI reject, Rust unit tests, disk/SHA/PE guard, branded release HOLD và Cloudflare source check. `scripts/cloudflare-pages-build.sh` thêm Node smoke trước Trunk WebAssembly build. Đây **chỉ là source audit**, không tự động chạy PowerShell, Windows, Cargo Native hoặc Kontakt.

## Những giới hạn quan trọng

- **Portable app data KHÔNG có nghĩa mọi thành phần Windows đều ngừng ghi vào C:**. Windows (shader caches, graphics driver, user profile) và third-party VST3/Kontakt có thể tự ghi vào thư mục riêng. Phải dùng process monitor trên Windows trước khi cam kết điều này.
- Không được dùng thư mục portable trên C: hoặc các đường dẫn network/UNC, junction/reparse chưa qua test; không tạo marker cho binary mới không có xác nhận checksum; staging chỉ khi build native đã được owner cho phép.
- Không tự động di chuyển dữ liệu SoundCraft trước đó, không đổi app ID/format session, không xóa/bỏ qua backup.
- `soundcraft.exe` và icon/version info Windows vẫn mang dấu gốc, WiX vẫn khai báo `Manufacturer="Learning Machines LLC"`. **Không phát hành hay giới thiệu bộ staging này là bộ cài ORVIA**.
- Không dùng dịch vụ trả phí, TinyFish, Windows CMD hay manual GitHub Actions; chưa thực hiện native QA trên laptop chủ dự án.

## QA/Evidence — gate được phân tách

| Gate | Nội dung | Bằng chứng chấp nhận | Tình trạng |
|---|---|---|---|
| P1 | Mã marker và GUI fail-closed | Review mã GitHub + Cargo `config_tests` Rust | Review/Node source contract; Rust **HOLD** |
| P2 | Script SHA256 và PE signature | PowerShell staging trên Windows test miễn phí được duyệt, so hash | **HOLD**, chưa chạy |
| P3 | Portable E:/F:, không phá dữ liệu | Mở GUI thật từ E:/F:, AutoSave/Logs/Preferences ở `ORVIA-Data`; không làm đổi data gốc | **HOLD** |
| P4 | Không ghi C: | Process Monitor ghi nhận filesystem của chính app/host/plugin và OS-cache exceptions | **HOLD**; chưa thể hứa 0 write |
| P5 | Windows native/ASIO/Kontakt 8 VST3 | Desktop executable x64 PASS, load plugin hợp lệ, NKI, phát/xuất audio | **HOLD** |
| P6 | Desktop branding | Logo/icon gốc ORVIA, đúng copyright/license/publisher, WinResource/MSI, ký số/test SHA | **HOLD** |
| P7 | Cloudflare Node + WASM | Check-run `completed/success` đúng SHA và URL immutable | Pending check for 4C.2 head |
| P8 | Chủ dự án nghiệm thu | Phản hồi trên Windows GUI app thật, kết quả hữu dụng và dự án 4–6 phút | **HOLD** |

**Không suy diễn Cloudflare build PASS thành Windows release PASS.**
**Không merge main/production khi chưa nhận quyết định riêng.** PR Draft base 4C.1.
