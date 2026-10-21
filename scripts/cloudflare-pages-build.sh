#!/usr/bin/env bash
# Temporary ORVIA Desktop progress portal for Cloudflare Pages, NOT the DAW.
# Scoped exclusively to feature/orvia-zrythm-desktop-transition.
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INPUT="$ROOT/preview/orvia-progress"
OUTPUT="$ROOT/dist/web"
mkdir -p "$OUTPUT"
test -s "$INPUT/index.html" || { echo "[ORVIA] Missing truthful progress page" >&2; exit 1; }
test -s "$INPUT/status.json" || { echo "[ORVIA] Missing progress status" >&2; exit 1; }
cp "$INPUT/index.html" "$OUTPUT/index.html"
cp "$INPUT/status.json" "$OUTPUT/status.json"
echo "[ORVIA] Temporary native Windows development monitor prepared; this is not a web DAW."
echo "[ORVIA] Output: $OUTPUT"
