# Integration smoke

Run from repo root:

```bash
./scripts/smoke-integration.sh
```

This validates unit tests and attempts a wasm build. Full testnet deploy
remains in `scripts/deploy.sh` + `DEPLOYMENTS.md`.

## Expected outcome

- `cargo test -p sorobill` passes
- Build step produces wasm (via Stellar CLI when available)
- Non-zero exit means do not promote the revision to testnet

## Related

- `make verify` — fast local gate (test + check)
- `docs/TESTING.md` — suite map
- Live UI: https://sorobill-app.vercel.app
