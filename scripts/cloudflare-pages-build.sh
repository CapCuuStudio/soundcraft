#!/usr/bin/env bash
# ORVIA AI ARRANGER — lightweight Cloudflare Pages preview.
# Experimental AI branch only. Do not merge this script into ORVIA DAW.
set -Eeuo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
test -s apps/orvia-ai-arranger/index.html || {
  echo "[ORVIA AI] Missing browser app." >&2
  exit 1
}
echo "[ORVIA AI] Running deterministic MIDI smoke tests"
node tests/orvia-ai-arranger-smoke.cjs
echo "[ORVIA AI] Copying static app (no Rust, npm install, or network required)"
mkdir -p dist/web/ai-arranger
cp apps/orvia-ai-arranger/index.html dist/web/index.html
cp apps/orvia-ai-arranger/index.html dist/web/ai-arranger/index.html
test -s dist/web/index.html && test -s dist/web/ai-arranger/index.html
echo "[ORVIA AI] Build PASS"
