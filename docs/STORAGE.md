# Storage layout

## Instance
| Key | Type | Notes |
|---|---|---|
| `Admin` | Address | Billing backend |
| `PlanCount` | u64 | Next plan id / total created |
| `GraceSecs` | u64 | Global grace; defaults to 172800 if unset |

## Persistent
| Key | Type | Notes |
|---|---|---|
| `Plan(u64)` | Plan | Includes `name`, `description`, price, interval, token, active |
| `Sub(Address, u64)` | Subscription | Includes `grace_deadline`, pause/fail counters |
