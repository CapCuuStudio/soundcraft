# ORVIA 4C — Native functional verification and Desktop release gates

**Ngày:** 2026-10-24. **Nền tảng:** 4B.4 owner-approved, code `282639b` Cloudflare PASS, acceptance docs-only `4aaf64a`.
**Trạng thái tổng thể:** **RELEASE HOLD**. This document is a test plan and evidence ledger, not a native test certification.

## 4C scope / policy

The web preview and its existing Graphite/Cyan UI are preserved. ORVIA Desktop is based on the native `soundcraft` Rust/egui application and its upstream MIT/Apache-2.0 basis, which must retain correct attribution. Do not publish upstream's app name, original icon or publisher/manufacturer metadata as an ORVIA-branded release. Do not introduce dummy Kontakt VST3 support.

**Execution constraints:** Free approved infrastructure only; no TinyFish; no ad-hoc Actions, production merge or paid cloud runner. Never ask the owner to use Windows CMD or change their C: drive. The Windows MSI currently defaults to Program Files (usually C:) and native preferences normally use `%APPDATA%\SoundCraft` — incompatible with the owner's E:/F: preference without a verified install/config plan. A portable test build on E:/F: does not automatically imply that nothing is written to C:.

## 4C.1 — Integration and source-contract gates (this PR)

### Code added

- `crates/engine/tests/orvia_4c_release_smoke.rs` — real native Engine test fixture, 3 tests: (a) Instrument/MIDI note/CC64, rendered nonzero finite PCM, built-in Gain insert and Undo/Redo; (b) `session.save_as` to a temporary .scraft, fresh `session.open`, retained notes/pedal and peak; (c) live-recording session-protection commands.
- `scripts/orvia-4c-native-qa.sh` — non-distributing, reproducible Cargo checks: 4C integrated tests; existing Bounce, Recording, Transport, MIDI Offset, Standard MIDI Files, Docking UI, Quantize, Save As, Sampler/Plugin views; final Desktop crate check. The script is **not executed by Cloudflare** or manually triggered in GitHub Actions. Native user hardware tests remain separate.
- `tests/orvia-4c-desktop-readiness.mjs` — dependency-free Node source audit of Native test coverage, package source and release blockers; reports `SOURCE_PREPARED, DESKTOP_RC_NOT_APPROVED`. Exits nonzero for missing release prerequisites; does **not** incorrectly claim Windows binaries or audio PASS.
- `scripts/cloudflare-pages-build.sh` — run this Node preflight after existing 4B.4 responsive source test and before the existing WebAssembly build. Cloudflare success establishes source audit and WASM bundle, **not the native Rust tests**.

### Matrix (exact minimum expected evidence)

| Gate | Target | PASS evidence | Status |
|---|---|---|---|
| G1 Source contracts | Cloudflare / Node | Node script PASS and blockers printed honestly | Pending current Cloudflare check |
| G2 Engine integrated | Native Rust | `cargo test --locked -p soundcraft-engine --test orvia_4c_release_smoke` exit 0 | HOLD — not executed |
| G3 Audio bounce | Native Rust | `crates/engine/tests/bounce.rs` and `midi_offset.rs` pass and PCM nonzero | HOLD — not executed on 4C |
| G4 MIDI SMF | Native Rust | `crates/midi/tests/smf.rs` pass MIDI note/tempo/controller roundtrip | HOLD |
| G5 UI/Save/Plugin | Native Rust | Docking, Quantize, Save As, Sampler Window, Plugin Window tests PASS | HOLD |
| G6 Native compile | Windows x64 MSVC | `soundcraft.exe` real Windows GUI build SHA and PE x64 subsystem 2 | HOLD |
| G7 Branded packaging | Windows | Original ORVIA icon, verified legal publisher, correct installer metadata, signing security, SHA256, uninstall | HOLD |
| G8 Device audio | Owner Windows device | Output, input, level meters, 48 kHz, buffers, ASIO if supported, no dropout under load | HOLD |
| G9 VST3 / Kontakt 8 | Owner native Windows | Native VST3 host loads installed Kontakt, MIDI reaches engine, NKI can be opened, GUI appears, output audible and exported correctly | HOLD |
| G10 Regression/soak | Owner native Windows | 4–6 minute projects, CPU/RAM, 20+ minutes playback, Save/Open/recover, MIDI edits, plugin reopen, export comparison | HOLD |

### Session/data safety

- No changes to session schema, audio files, MIDI data, routing, Undo history or the existing Web UI are authorized by 4C.1.
- Reuse source-controlled tests for aborting record safely and preserving an existing file on bounce failure.
- Never assert `DESKTOP_RC_READY` while branding, native build, plugin licensing or hardware gate is missing.
- Avoid silently changing `%APPDATA%` directories or storing files on C:. Evaluate an explicit portable data mode or approved folder selector in a later stage only after code review and tests.

## 4C.2 — Windows Desktop pre-release (not started)

**Current blockers found by live source inspection:**
1. `apps/soundcraft/Cargo.toml` still produces binary `soundcraft`; internal name may intentionally remain for project-file compatibility, but downloadable artifact must truthfully identify itself.
2. `packaging/windows/soundcraft.wxs` declares `Name="SoundCraft"`, `Manufacturer="Learning Machines LLC"`, installer folder and Start Menu `SoundCraft`, and upstream marketing/support URLs. These must not be falsely carried into an ORVIA-branded installer.
3. `packaging/windows/package.ps1` packages `soundcraft.exe` and original `assets/app-icon/soundcraft.ico`. A new ORVIA icon and publisher/about/licensing review are required before distribution.
4. `apps/soundcraft/src/config.rs` defaults Windows preferences to `%APPDATA%\SoundCraft` (usually system drive). Owner prefers no C: writes; need controlled test for all persistent data including logs, recovery, plugins and caches.
5. Existing `.github/workflows/release.yml` triggers on `release` or manual dispatch and calls an upstream reusable workflow at `@main` with signing secrets; not approved for use now. No release build or upload attempted.

**Windows RC criteria:** Build in an approved isolated runner; installer/portable artifact must display correct name/icon, install/uninstall without overwriting unrelated SoundCraft, pass PE/architecture checks, signed/unsigned provenance and SHA256 validation; copy test project to allowed drive; launch, audio I/O, MIDI, plugin host, export; review author/disclaimer. Release only on separate explicit owner approval.

## 4C.3 — Practical test script for the owner (GUI only, when a verified RC exists)

1. Launch portable ORVIA from E:/F: after verification; check title, menus, Save/Open path, and whether any data is written to C:. **Do not install or test unrebranded MSI now.**
2. Audio: import WAV, play, adjust track mixer, Mute/Solo/Pan/FX, bounce and compare peaks, tails and audible level.
3. MIDI: import `.mid`, add/delete notes, velocity variations, CC64 pedal, use built-in synth and verify exported MIDI opens correctly.
4. Plugin: verify built-in FX. Test third-party Kontakt 8 only in the **native host** (not Cloudflare browser) with its user-owned installed license and NKI; inspect plugin scan/logs and audible output.
5. Reliability: reopen a 4–6-minute project, monitor actual CPU/RAM/audio underruns on the user's Windows setup; ensure no loss across restart, Save, Undo, Autosave.
6. Record results in this ledger; FAIL means do not issue installer download. Current result: **HOLD for all owner-native checks**.

## Evidence discipline

- Source review and GitHub commits show code exists, not runtime tests.
- Cloudflare check success is a WebAssembly build, not Windows compilation/hardware pass.
- An existing upstream test is not a PASS until it is run against this branch and its test log is attached.
- `Desktop release prepared` is intentionally **not** the same as `Desktop release released`.
