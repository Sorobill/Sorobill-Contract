# WASM build targets

| Target | Command | Use |
|--------|---------|-----|
| `wasm32v1-none` | `stellar contract build` | Deploy to Stellar networks |
| `wasm32-unknown-unknown` | `cargo build --target ...` | Legacy; may fail host validation |

Always prefer `stellar contract build` / `make build`.

## Why wasm32v1-none?

Stellar's current host validates contracts built for the `wasm32v1-none` target.
Artifacts from `wasm32-unknown-unknown` may fail upload or invoke on recent
networks even if `cargo test` passes.

## Local fallback

The Makefile falls back to `cargo build --target wasm32-unknown-unknown --release`
only when the Stellar CLI is missing — useful for CI sandboxes, not for deploy.
