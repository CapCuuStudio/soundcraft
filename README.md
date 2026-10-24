# ORVIA AI Arranger — Web Lab V0.1

Standalone branch with only a static MIDI arrangement application and a deterministic smoke test. **Not yet a trained AI model.**

## Scope
- Demo melody or user-supplied MIDI (SMF 0/1); chord input and rule-based 4-track accompaniment (piano, guitar, bass, drums).
- Browser WebAudio preview; export standard multitrack MIDI.
- No upload of MIDI, third-party AI APIs, paid GPU, pretrained weights, or Kontakt hosting.
- Current development gates G00–G16 are NOT automatically accepted by successful build.

## GitHub and Cloudflare deployment
This tree is intentionally isolated from the Rust/SoundCraft DAW source.

Cloudflare Pages settings: framework `None`, project root (blank), build command `bash scripts/cloudflare-pages-build.sh`, output `dist/web`; Node.js 22.16.0 pinned via `.node-version`. Standalone Pages project should select **this branch** as its production branch. Testing on the existing Cloudflare project creates only a branch preview, not a production release.

Local/CI smoke: `node tests/orvia-ai-arranger-smoke.cjs`. The build does not use npm packages, install Rust, contact a model API, or require Windows CMD on the owner's PC.

## Deployment safeguards
1. Keep `main` and all ORVIA DAW feature branches untouched.
2. Only send code to the dedicated `preview` branch after smoke passes. Restrict new Cloudflare project to that branch.
3. Exclude `docs/*`, `README.md`, and `*.md` from Cloudflare build watch paths, or prefix documentation-only commits with `[CF-Pages-Skip]`.
4. Do not merge experimental branch into SoundCraft `main`.
5. If a new standalone GitHub repository is created later, copy these five files there and connect the dedicated Cloudflare Pages project.

## Provenance
Based on 29-page "ORVIA AI Arranger — Kế hoạch phát triển tổng thể" (2026-10-24). The complete development plan remains authoritative and includes legal data gates, model training, expression and integration. This research sandbox is not a substitute for them.
