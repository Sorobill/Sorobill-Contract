# Makefile targets

| Target | Purpose |
|---|---|
| `make verify` | `cargo test` + `cargo check` (default CI-style gate) |
| `make test` | Run `cargo test -p sorobill` |
| `make check` | Run `cargo check -p sorobill` |
| `make build` | Prefer `stellar contract build`; fallback to wasm32 cargo |
| `make fmt` | `cargo fmt --all` |
| `make clean` | `cargo clean` |
| `make deploy` | Build then `scripts/deploy.sh` |
| `make init` | `scripts/init.sh` against deployed id |
| `make help` | Print this summary |

## Variables

- `NETWORK` — default `testnet`
- `SOURCE` — default `sorobill-admin` (Stellar CLI identity)
- `CONTRACT` — crate/package name `sorobill`
