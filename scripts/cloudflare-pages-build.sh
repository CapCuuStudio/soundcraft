#!/usr/bin/env bash
# Build the *real* ORVIA Rust/WebAssembly DAW for Cloudflare Pages.
# Build setting: bash scripts/cloudflare-pages-build.sh
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

# ORVIA 4B.4: fail early on documented responsive/command regressions.
# This is a source-contract smoke test, NOT a browser/audio test.
if ! command -v node >/dev/null 2>&1; then
  echo '[ORVIA] Node.js missing; cannot run responsive regression checks.' >&2
  exit 1
fi
node tests/orvia-4b4-responsive-contract.mjs
# 4C: native release source audit only; NEVER mark Windows builds as passed here.
node tests/orvia-4c-desktop-readiness.mjs
# 4C.2: source-only portable data and x64 QA staging guard; not Windows execution.
node tests/orvia-4c2-portable-contract.mjs
# 4C.3: Windows staging/evidence source guard only; no Native Windows execution.
node tests/orvia-4c3-windows-qa-contract.mjs
# Compact workstation Track Header controls: source regression, no audio engine modifications.
node tests/orvia-track-header-compact-contract.mjs

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

echo "[ORVIA] Real DAW bundle validated: $DIST"
