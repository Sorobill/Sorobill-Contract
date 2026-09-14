# WASM build targets

| Target | Command | Use |
|--------|---------|-----|
| `wasm32v1-none` | `stellar contract build` | Deploy to Stellar networks |
| `wasm32-unknown-unknown` | `cargo build --target ...` | Legacy; may fail host validation |

Always prefer `stellar contract build` / `make build`.
