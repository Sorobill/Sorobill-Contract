# Sorobill TypeScript client types

Shared type definitions for Frontend and Backend integrations.

```ts
import { TESTNET, SOROBILL_CONTRACT_METHODS } from "./types";
```

After deploying, set `SUBSCRIPTION_CONTRACT_ID` / `NEXT_PUBLIC_SUBSCRIPTION_CONTRACT_ID`
from `DEPLOYMENTS.md`.

## Layout

| Path | Contents |
|---|---|
| `types.ts` | Shared method list + network constants |
| `typescript/` | 0.3 stub types (`Plan`, `Subscription`, `METHODS`) |
| `index.ts` | Re-exports for package-style imports |

## Live demo

https://sorobill-app.vercel.app — configure it with the testnet contract id.
