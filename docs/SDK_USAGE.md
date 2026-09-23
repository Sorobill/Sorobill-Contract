# SDK usage sketch

```ts
import { CONTRACT_VERSION, METHODS } from "../clients/typescript/src/types";

console.log(CONTRACT_VERSION, METHODS.includes("is_subscribed"));
```

Regenerate full clients after wasm publish.

## Method checklist (0.3.0)

Ensure your app/backend knows about:

- Meta: `version`, `get_admin`, `plan_count`, `is_subscribed`
- Grace: `set_grace_period`, `get_grace_period`
- Plans: `create_plan` (includes `description`), `reactivate_plan`
- Billing: `execute_billing` → `Paid` | `Failed`

## Live UI

The testnet demo app is at https://sorobill-app.vercel.app and consumes the
contract id from `DEPLOYMENTS.md`.
