#!/usr/bin/env bash
# Build the Sorobill wasm artifact.
set -euo pipefail

rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
cargo build --target wasm32-unknown-unknown --release -p sorobill

WASM="target/wasm32-unknown-unknown/release/sorobill.wasm"
ls -lh "$WASM"
echo "Built: $WASM"
