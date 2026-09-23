# Scripts

| Script | Purpose |
|---|---|
| `build.sh` | Build wasm artifact |
| `deploy.sh` | Deploy to configured network |
| `init.sh` | Call `initialize` after deploy |
| `test.sh` | Wrapper around cargo tests |
| `smoke-integration.sh` | Test + wasm smoke |
| `print-version.sh` | Print crate / contract version |
| `verify-admin.sh` | Sanity-check admin on a deploy |
| `list-docs.sh` | List markdown docs |

Prefer Makefile targets (`make verify`, `make deploy`) when available.
