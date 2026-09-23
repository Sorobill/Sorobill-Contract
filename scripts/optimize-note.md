`stellar contract build` already optimizes the wasm.
Prefer it over raw `cargo build --target wasm32-unknown-unknown`.

Prefer `make build` (Stellar CLI / wasm32v1-none) before any size optimization
experiments. Deployed wasm hash is recorded in `DEPLOYMENTS.md`.
