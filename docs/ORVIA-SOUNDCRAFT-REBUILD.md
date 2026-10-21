# ORVIA Studio — SoundCraft native DAW rebuild

**Baseline:** `feature/orvia-cubase-project-zone` (PR #4). **Working branch:** `feature/orvia-soundcraft-rebuild`.

## Why this branch exists

The owner chose the real SoundCraft DAW as the technical foundation of ORVIA, rather than
more standalone HTML mock-ups or the failed Zrythm/WebAssembly experiment. ORVIA should
use a Cubase-familiar workflow **without copying Steinberg assets, code, icons or UI files**.

SoundCraft is a **pre-alpha** Rust/egui DAW, not a fully functioning replacement for Cubase.
Its native desktop app, project model, MIDI, audio, command layer and project/session
format already exist, and must be preserved while the workflow is adapted.

## Changes actually made on this branch

- `crates/ui-egui/src/dialogs.rs`: redesigned the native *Add Track* dialog into a
  structured workflow with separate type, supported channel configuration, musical
  timebase, name and track count. It calls the **real** `track.new` engine command.
  Supported kinds are the real engine enums: Audio, Instrument, MIDI, Aux, Folder, VCA,
  Master and Video. Presets reset to appropriate defaults on kind change. Empty
  names and out-of-range counts are normalized; MIDI/instrument kinds default to
  tick-based time; two unit tests cover real engine track creation and validation.
- `apps/soundcraft/src/main.rs`: desktop window titles and version banner say
  **ORVIA Studio**. The executable/crate name, config/app ID and session extension
  remain SoundCraft internally for compatibility, intentionally.
- Reuses the prior Cubase-style Project Zone improvements from PR #4:
  one Inspector alongside the actual timeline track headers, optional Lower Zone
  and opening the existing MIDI editor by double-click.

## Not yet supported / not yet verified

- **Rust compiler, Windows installer, browser WASM build or functional audio QA
  have not been run for this branch.** The code is committed, not a verified release.
- Real audio-device dropdowns and MIDI ports were intentionally **not fabricated**
  in Add Track. SoundCraft's audio hardware and MIDI input gaps remain.
- Sampler and effect track *kinds* are not in `TrackKind` and have not been
  added deceptively. VST3/Kontakt on the user's Windows hardware is unverified.
- The old SoundCraft app icon remains in place and **must be replaced** with
  original ORVIA art and license attribution before a branded binary is distributed.
- The DAW has outstanding reliability issues listed in `docs/gaps.md`.

## QA requirements before user-facing installer/preview

1. `cargo test -p soundcraft-ui-egui` and `cargo check -p soundcraft`
   on a suitable free development environment; fix compile errors.
2. Windows native launch: verify Add Track for Audio, Instrument and MIDI;
   count=2; undo, open existing `.scraft`, save and reopen.
3. Confirm a real audible synth, MIDI editor draw/audition, mixer routing and bounce.
   VST3/Kontakt must pass a separate native host/plugin test before it is advertised.
4. Replace icon and audit dependencies/attribution. Keep original repo main,
   production Cloudflare and other LUNOR MUSIC projects untouched until approved.

**Restrictions:** No TinyFish; no paid runner; no GitHub Actions without approval;
no CMD on the user's Windows PC; no production merge/deploy; don't fake success.
