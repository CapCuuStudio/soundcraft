# ORVIA 4C.3 — Windows Desktop acceptance matrix

**Date:** 27/10/2026. **Base:** owner-approved Compact Track Header PR #27 (code cf7b29d; docs-only 5df209f), 4C.2 Portable PR #26, 4C.1 PR #25. **Decision: RELEASE HOLD.**

## Scope

4C.3 prepares safe native Windows x64 acceptance checks without altering engine, mixer, MIDI, session schema, VST3 host, or approved Web UI. There is no approved Windows installer or signed ORVIA desktop binary yet. The internal executable is still named soundcraft.exe, and the upstream MSI contains SoundCraft branding/publisher. Neither the upstream MSI nor an unbranded staging folder may be distributed as a released ORVIA app. No main/production merge, manual GitHub Actions, TinyFish, paid runner or Windows CMD for the owner.

## Changes

1. scripts/orvia-4c2-stage-windows-qa.ps1: reject drive-relative destination strings and existing ancestor junctions/symlinks (Windows ReparsePoint), before creating the staging directory. This avoids a known E:/F: path redirection risk; it does not prove zero C: writes or prevent every filesystem race.
2. scripts/orvia-4c3-inspect-windows-qa.ps1: **read-only** internal staging provenance inspection for approved QA staff. Validates absolute E:/F:, no known reparse ancestors, marker, manifest, expected binary SHA256 and PE x64 GUI header. Never launches executable, scans plugins, installs, connects to network or edits registry. Optional JSON evidence output only to a new file under an existing E:/F: folder; no usernames, paths, audio or plugin inventory in report.
3. tests/orvia-4c3-windows-qa-contract.mjs: dependency-free source preflight; Cloudflare executes it before Rust/WASM. PASS here only establishes that expected source checks exist, not that PowerShell or native hardware tests ran.

## G1–G10 Gates

| Gate | Test | Accept only | Status |
|---|---|---|---|
| G1 | Source contracts | Node 4C.1/4C.2/4C.3 tests and approved Track Header source gate exit 0 | Pending CI |
| G2 | Native Rust integration | Cargo real engine, bounce, SMF, docking and plugin UI tests and logs | **HOLD** |
| G3 | Windows x64 GUI | Authenticated binary SHA, PE x64 GUI and successful startup on approved Windows | **HOLD** |
| G4 | Portable app data | E:/F: app settings/autosave/logs in ORVIA-Data; old data untouched | **HOLD** |
| G5 | Disk write audit | Actual Windows file monitoring including driver/third-party caches; no false zero-C claim | **HOLD** |
| G6 | Audio / ASIO | Output/input, 48 kHz, buffer, record and playback stability, verified ASIO driver if eligible | **HOLD** |
| G7 | VST3 / Kontakt 8 | Native scan, licensed Kontakt GUI, owned NKI loads, audible MIDI instrument | **HOLD** |
| G8 | MIDI / mixer / bounce | Velocity, CC64, gain/M/S and exported WAV audible/reopen comparison | **HOLD** |
| G9 | Long-run regression | Project 4–6 minutes, 20-minute soak, Save/Open/AutoSave, plugin state persists | **HOLD** |
| G10 | Branding / signing / legal | Original ORVIA artwork, truthful publisher, licenses, file hash and separate user release approval | **HOLD** |

## Owner acceptance using only the Windows GUI (after a verified branded artifact is available)

1. **Startup.** Double-click the vetted portable executable on a new E:/F: test folder, check native UI/About and ORVIA-Data. Do not use the old upstream MSI or copy anything over an existing app folder.
2. **Storage.** QA staff should separately monitor real writes on E:/F: and C: with consent; OS, soundcard and Kontakt caches may still write to C:. Verify existing projects/configs remain intact.
3. **Audio.** Setup > Playback Engine: select the soundcard output, sample rate 48 kHz and an accepted buffer. Play one minute WAV, then test input recording. Check for glitches. WASAPI success is not evidence of ASIO success.
4. **VST3.** Scan installed VST3s inside the native app; instantiate Kontakt 8, open its real editor, load one legally owned NKI, play MIDI, confirm audible stereo, Save/Open and plugin-state restoration. A Kontakt Player-incompatible NKI is a blocker, not a reason to bypass its license.
5. **MIDI/Mixer.** Import a MIDI file, edit velocity/CC64, Mute/Solo, gain, bounce WAV to E:/F:; listen and compare level/attack/pedal tail after reopening.
6. **Stability.** Use a disposable 4–6 minute test project, play 20 minutes, edit notes, restart app, Save/Open, confirm UI state, plugin state, exported audio, CPU/RAM and any dropouts. Never test destructively on irreplaceable data.
7. **Release.** Owner must separately approve after all gates have evidence. Until then, Desktop RELEASE HOLD.

## QA log template

- Reviewed commit / executable SHA256:
- Gate ID G1–G10 and evidence type (source / Cargo / Windows native / GUI / audio):
- PASS / FAIL / HOLD:
- Exact observation and reproduction steps:
- Generic Windows version/device class (no personal identifiers):
- Redacted screenshots/log files on E:/F::
- Remaining blocker and next action:

## Current result

4C.3 adds the source-level staging guard, internal read-only provenance inspection and QA matrix. **No Windows x64 binary was compiled or run in this chat.** Windows PowerShell, native Cargo tests, actual soundcard/ASIO, Kontakt VST3/NKI, drive monitoring and 4–6 minute soak are NOT RUN. A Cloudflare PASS may be recorded only against the exact reviewed SHA and never replaces native results. **RELEASE HOLD.**
