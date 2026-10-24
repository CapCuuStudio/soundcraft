#!/usr/bin/env bash
# Standalone ORVIA AI Arranger Web — no Rust, no npm install, no remote dependencies.
set -Eeuo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."
node tests/orvia-ai-arranger-smoke.cjs
mkdir -p dist/web
cp index.html dist/web/index.html
test -s dist/web/index.html
echo "[ORVIA AI] Static Web build PASS"
