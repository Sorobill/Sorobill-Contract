# TypeScript bindings

Generated Soroban clients live under `clients/` after:

```bash
stellar contract bindings typescript \
  --wasm target/wasm32v1-none/release/sorobill.wasm \
  --output-dir clients/typescript
```

Until regenerated for 0.3.0, use the hand-written stub in
`clients/typescript/src/types.ts` for app/backend compile checks.

## Stub vs generated

| Artifact | Purpose |
|---|---|
| `clients/typescript/src/types.ts` | Hand-written 0.3 shapes + `METHODS` |
| `clients/types.ts` | Shared helpers for Backend/App |
| Generated client | Full RPC wrappers after wasm publish |

Keep stub `CONTRACT_VERSION` and method lists aligned with `constants.rs` and
`lib.rs` when adding entrypoints.
