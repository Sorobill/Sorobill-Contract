# Storage keep-alive

Soroban persistent / instance entries expire unless TTL is extended.

## What to bump

| Entry | When |
|---|---|
| Instance (`Admin`, `PlanCount`, `GraceSecs`) | On admin operations or a periodic keeper |
| `Plan(id)` | When merchants or indexers touch the plan |
| `Sub(subscriber, plan_id)` | On each successful/failed bill and lifecycle op |

## Operator pattern

Run a keeper that:

1. Lists active subscriptions from your indexer.
2. Invokes a no-op read path or dedicated bump helper (off-chain tooling).
3. Alerts when TTL drops below a safety threshold.

Failure to extend TTL presents as missing plans/subs — not as protocol errors.
