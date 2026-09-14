#!/usr/bin/env bash
# Build the Substrata wasm artifact.
set -euo pipefail

rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true
cargo build --target wasm32-unknown-unknown --release -p substrata

WASM="target/wasm32-unknown-unknown/release/substrata.wasm"
ls -lh "$WASM"
echo "Built: $WASM"
