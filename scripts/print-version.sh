#!/usr/bin/env bash
set -euo pipefail
grep 'CONTRACT_VERSION' contracts/sorobill/src/constants.rs
grep '^version' contracts/sorobill/Cargo.toml | head -1
