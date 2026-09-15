#!/usr/bin/env bash
# Local smoke: build wasm + run unit tests. Does not deploy.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
echo "== cargo test -p sorobill =="
cargo test -p sorobill
echo "== stellar contract build (if available) =="
if command -v stellar >/dev/null 2>&1; then
  stellar contract build || cargo build -p sorobill --target wasm32v1-none --release
else
  echo "stellar CLI missing; skipping wasm build"
fi
echo "OK"
