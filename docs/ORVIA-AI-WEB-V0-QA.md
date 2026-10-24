# ORVIA AI ARRANGER — Web-first V0.1 QA & scope

**Date:** 2026-10-24 · **Branch:** `feature/orvia-ai-arranger-web-v0` · **State:** sandbox; G00 has not passed G00 acceptance.

## Provenance / decisions
- Source: owner-provided 29-page ORVIA AI ARRANGER plan v1.0 (2026-10-24), G00–G16.
- Read central LUNOR MUSIC technical log on Google Drive and ORVIA repository technical log before code changes; reviewed `AGENTS.md`, `ROADMAP.md`, `docs/gaps.md`.
- Root DAW source and sessions are untouched. Experimental Cloudflare preview branch exposes rule-based web app at `/` and `/ai-arranger/` **only when CF_PAGES_BRANCH matches this branch**; all other branches continue to build the real Rust WASM DAW.
- Browser uses no AI API, no preload weights, no paid GPU, no database, no MIDI upload.
- Built-in sample melody is programmatically created, not downloaded from copyrighted work.

## V0 prototype features
1. Generate synthetic example melody (8 bars) or import SMF 0/1 MIDI.
2. Choose melody source track, chord sequence, BPM, seed, density and expressiveness.
3. Deterministic rule accompaniment as piano, guitar, bass, drums (GM channel 10).
4. A/B inspection via canvas piano-roll-like overview, volume/mute/solo; simple oscillator playback.
5. Export multitrack SMF type 1 with melody notes, program changes, tempo and meter.
6. Browser-only transformation, no upload or storage of user MIDI.

## NOT yet complete
- This is *not a trained AI model*. It is an executable baseline sandbox for later comparisons, corresponding to early exploration of G05.
- G00, G01, G02, G03, G04 are not passed merely because this prototype runs. No rights-cleared training corpus exists.
- No chord detection from MIDI, no tempo-map roundtrip, only simple 4/4 bars (SMF source can be longer than 64 bars, capped); no performance-level pedal CC64 or genuine strumming; no Kontakt/VST3 host.
- Import does not retain original meta-events/CC/track routing in exported file; only chosen melody notes (pitch, note time, duration, velocity) are used as condition.
- Audio preview uses WebAudio oscillators, not exported audio or professional sample instruments.
- No external browser UAT or recorded listening tests yet.
- No new separate GitHub repository (G00.04 remains open).

## Test matrix (branch smoke test, no paid services)
- T1: JS syntax; supported chord spellings Am/F/C/G/D7/Cmaj7/Bm7/Dsus4.
- T2: Synthetic melody 64 notes; generates 5 tracks, including original melody.
- T3: 4 accompaniment roles populated; drums use channel index 9.
- T4: Same seed produces deterministic note lists.
- T5: Export → parse SMF type 1: keep pitches/timing/duration/velocity and note count.
- T6: Malformed MIDI rejected.
- Browser manual UAT outstanding: open Cloudflare Preview root and `/ai-arranger/`, try Play/Stop, M/S/volume, MIDI import, export, keyboard resize, audible preview.

## Quality / release gates
- Web branch CI smoke alone is not proof of audio sound quality, full DAW compatibility, legal clearance, or G00 completion.
- Never merge this Draft PR into DAW main automatically. Before any integration into ORVIA Desktop: maintain separate repo, legal/data gates, model training and QA as described in the owner plan.
- Preserve original SoundCraft engine/plugin host/session formats and production website.

## Reproducibility
- Run `node tests/orvia-ai-arranger-smoke.cjs` on a CI runner. No owner PC terminal required.
- Cloudflare Pages build config uses `bash scripts/cloudflare-pages-build.sh` and output `dist/web`.
- Branch-specific cloud deploy needs a verified successful check and its real immutable preview URL before saying publicly deployed.
