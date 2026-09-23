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

## TTL guidance

- Instance data (admin, counters, grace) should be extended on every admin op.
- Persistent plan/sub rows should be bumped by a keeper when nearing expiry.
- Expired entries become unreadable; treat as operational outage, not a protocol bug.

## Keying

Subscription primary key is `(subscriber, plan_id)`. A cancelled subscription
row remains; a second `subscribe` to the same pair fails with `AlreadySubscribed`.
