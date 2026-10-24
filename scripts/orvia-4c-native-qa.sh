#!/usr/bin/env bash
# ORVIA 4C.1: a repeatable native test gate for an approved free CI runner.
# This script does not publish a release, build MSI, run GitHub Actions, or
# claim to test physical Windows audio devices.
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
command -v cargo >/dev/null 2>&1 || {
  echo "[ORVIA 4C] HOLD: Rust Cargo not available in this native runner" >&2
  exit 1
}

export CARGO_TERM_COLOR=always

echo "[ORVIA 4C] 1/5 — Real MIDI + synth + mixer + disk Save/Open + recording safety"
cargo test --locked -p soundcraft-engine --test orvia_4c_release_smoke

echo "[ORVIA 4C] 2/5 — Core audio bounce, recording guard and transport"
cargo test --locked -p soundcraft-engine --test bounce
cargo test --locked -p soundcraft-engine --test recording
cargo test --locked -p soundcraft-engine --test session_transport
cargo test --locked -p soundcraft-engine --test midi_offset

echo "[ORVIA 4C] 3/5 — Standard MIDI File round-trip"
cargo test --locked -p soundcraft-midi --test smf

echo "[ORVIA 4C] 4/5 — Real egui docking, MIDI, plugin UI and Save As"
cargo test --locked -p soundcraft-ui-egui --test docking_workflows
cargo test --locked -p soundcraft-ui-egui --test midi_quantize
cargo test --locked -p soundcraft-ui-egui --test save_as_replace
cargo test --locked -p soundcraft-ui-egui --test sampler_window
cargo test --locked -p soundcraft-ui-egui --test plugin_window

echo "[ORVIA 4C] 5/5 — Native Desktop crate compile"
cargo check --locked -p soundcraft

echo "[ORVIA 4C] PASS native Rust test subset / compile on this runner."
echo "[ORVIA 4C] HOLD Windows release: MSI branding, signed installer, native Windows x64 build,"
echo "             physical soundcard/ASIO and Kontakt 8 VST3 still need separate verification."
