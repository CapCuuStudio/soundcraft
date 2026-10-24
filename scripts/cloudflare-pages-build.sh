#!/usr/bin/env bash
# Build the *real* ORVIA Rust/WebAssembly DAW for Cloudflare Pages.
# Build setting: bash scripts/cloudflare-pages-build.sh
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# Dedicated branch-only Web Lab. Other branches keep the actual DAW WASM build below.
if [[ "${CF_PAGES_BRANCH:-}" == "feature/orvia-ai-arranger-web-v0" ]]; then
  echo '[ORVIA-AI] Standalone Web-first sandbox preview branch.'
  command -v node >/dev/null 2>&1 || { echo '[ORVIA-AI] Node.js is required for smoke tests' >&2; exit 1; }
  node "$ROOT/tests/orvia-ai-arranger-smoke.cjs"
  mkdir -p "$ROOT/dist/web/ai-arranger"
  cp "$ROOT/apps/orvia-ai-arranger/index.html" "$ROOT/dist/web/index.html"
  cp "$ROOT/apps/orvia-ai-arranger/index.html" "$ROOT/dist/web/ai-arranger/index.html"
  echo '[ORVIA-AI] Smoke-tested static MIDI app; no DAW production changes.'
  exit 0
fi

export CARGO_INCREMENTAL=0
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
export CARGO_TERM_COLOR=always
export PATH="$HOME/.cargo/bin:$HOME/.local/bin:$PATH"

RUST_TOOLCHAIN="1.95.0"
TRUNK_VERSION="v0.21.14"
mkdir -p "$HOME/.local/bin"

if ! command -v rustup >/dev/null 2>&1; then
  echo '[ORVIA] Installing Rust using the official rustup installer.'
  curl --fail --silent --show-error --location --retry 2 https://sh.rustup.rs -o /tmp/orvia-rustup-init.sh
  sh /tmp/orvia-rustup-init.sh -y --profile minimal --default-toolchain "$RUST_TOOLCHAIN"
fi
rustup toolchain install "$RUST_TOOLCHAIN" --profile minimal --target wasm32-unknown-unknown
export RUSTUP_TOOLCHAIN="$RUST_TOOLCHAIN"

if ! command -v trunk >/dev/null 2>&1; then
  echo "[ORVIA] Installing pinned Trunk $TRUNK_VERSION."
  archive="$(mktemp /tmp/orvia-trunk-XXXXXX.tar.gz)"
  checksum="$(mktemp /tmp/orvia-trunk-checksum-XXXXXX)"
  base="https://github.com/trunk-rs/trunk/releases/download/$TRUNK_VERSION/trunk-x86_64-unknown-linux-gnu.tar.gz"
  curl --fail --silent --show-error --location --retry 2 "$base" -o "$archive"
  curl --fail --silent --show-error --location --retry 2 "${base}.sha256" -o "$checksum"
  expected="$(awk '{print $1}' "$checksum")"
  actual="$(sha256sum "$archive" | awk '{print $1}')"
  if [[ -z "$expected" || "$expected" != "$actual" ]]; then
    echo '[ORVIA] Trunk SHA256 mismatch. Refusing to run downloaded binary.' >&2
    exit 1
  fi
  tar -xzf "$archive" -C "$HOME/.local/bin" trunk
  chmod +x "$HOME/.local/bin/trunk"
  rm -f "$archive" "$checksum"
fi

echo '[ORVIA] Compiling Rust/WebAssembly DAW (not a placeholder website).'
cd "$ROOT/apps/soundcraft-web"
trunk build --release

DIST="$ROOT/dist/web"
[[ -s "$DIST/index.html" ]] || { echo '[ORVIA] Missing index.html' >&2; exit 1; }
find "$DIST" -type f -name '*.wasm' -print -quit | grep -q . || {
  echo '[ORVIA] Missing WASM bundle. Refusing to deploy HTML shell.' >&2
  exit 1
}
while IFS= read -r -d '' file; do
  bytes="$(wc -c < "$file")"
  if (( bytes > 25 * 1024 * 1024 )); then
    echo "[ORVIA] Cloudflare Pages 25 MiB per-asset limit exceeded: $file ($bytes bytes)" >&2
    exit 1
  fi
done < <(find "$DIST" -type f -print0)

# Isolated rule-based AI Arranger research UI; no changes to Rust/DAW engine.
# This subpath only exists in this experimental branch and preserves the real DAW root.
ARRANGER_SOURCE="$ROOT/apps/orvia-ai-arranger/index.html"
[[ -s "$ARRANGER_SOURCE" ]] || { echo '[ORVIA-AI] Missing research UI entrypoint' >&2; exit 1; }
mkdir -p "$DIST/ai-arranger"
cp "$ARRANGER_SOURCE" "$DIST/ai-arranger/index.html"
echo "[ORVIA-AI] Standalone browser MIDI arranger copied to $DIST/ai-arranger"

echo "[ORVIA] Real DAW bundle validated: $DIST"
