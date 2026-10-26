# ORVIA 4C.4 — Actual Windows x64 Native Build QA

**Date:** 27 October 2026. **Base:** 4C.3 Draft PR #28. **Status:** source implementation; Windows CI outcome pending; real hardware/release gates HOLD.

## Scope and safety

Build and test the **real native Rust desktop application on a GitHub-hosted Windows x64 runner**, not Cloudflare/WASM. Use a branch-only automatic push workflow (no workflow_dispatch, no upstream release workflow, no secrets and no manual GitHub Actions run). This does not touch the owner's laptop, Windows CMD, C: drive, or production.

Existing executable is still internally named soundcraft.exe and embeds upstream SoundCraft icon and publisher metadata. Do not present it as a publicly released ORVIA-branded program. The CI **does not upload the executables or publish an installer**; only short checks and SHA256/PE provenance are printed in logs. A completed build does not verify running Kontakt, ASIO, NKI, soundcards or drive isolation.

## Implementation

1. .github/workflows/orvia-4c4-windows-native.yml: branch-scoped automatic CI on windows-2022 with contents: read permissions, pinned checkout, timeout and cancellation, Rust 1.95.0. No new paid services or repository releases.
2. Run native cargo test --locked --release for engine 4C.1 smoke, MIDI SMF, and Windows application config_tests. Build soundcraft and soundcraft-cli native Windows x64 in release profile, using LTO=false solely to shorten the QA run.
3. scripts/orvia-4c4-pe-provenance.ps1: read-only checks DOS/PE32+, x64 machine, GUI subsystem for soundcraft.exe and console subsystem for soundcraft-cli.exe, plus SHA256; report output carries explicit native runtime/ASIO/Kontakt/branding/release HOLD.
4. tests/orvia-4c4-win-build-contract.mjs: independent source-only safety check before compiler. Node PASS alone is not native PASS.

## Gated QA evidence

| Gate | Required evidence | Status |
|---|---|---|
| A | Node source contracts exit 0 on exact revision | Pending CI |
| B | Windows runner cargo native test commands exit 0 | Pending CI |
| C | Actual Windows x64 build and PE/SHA check exit 0 | Pending CI |
| D | Start real GUI on owner-approved E:/F: portable drive, check own app data | HOLD |
| E | Soundcard output/input/latency/ASIO vendor driver on real Windows | HOLD |
| F | Native VST3 scanner, licensed Kontakt 8 GUI and owned NKI audible output | HOLD |
| G | 4–6-minute project, 20-minute soak, restart/Save/Open/bounce | HOLD |
| H | Original ORVIA logo, lawful attribution, truthful publisher/signature, owner GO | HOLD |

## When runner fails

If GitHub runners are blocked or Rust compilation fails, record the exact error. Do not infer Windows success from Cloudflare PASS. Resolve native compile failures in this branch without changing approved Web UI. ASIO is opt-in and license-dependent; this standard job does not claim it is enabled. No Kontakt installation on shared CI, no unbranded installer distribution and no user CMD instructions.

**Next gate:** inspect Windows CI result for exact head SHA, then plan a separately approved branded Portable GUI test, preserving the release HOLD until actual Windows/ASIO/Kontakt evidence exists.
