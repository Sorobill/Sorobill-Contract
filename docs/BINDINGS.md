# TypeScript bindings

Generated Soroban clients live under `clients/` after:

```bash
stellar contract bindings typescript \
  --wasm target/wasm32v1-none/release/sorobill.wasm \
  --output-dir clients/typescript
```

Until regenerated for 0.3.0, use the hand-written stub in
`clients/typescript/src/types.ts` for app/backend compile checks.
