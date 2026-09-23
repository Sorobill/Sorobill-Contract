# Contract events

Sorobill emits classic Soroban topics via `Env::events().publish`
(deprecated in soroban-sdk 23 in favor of `#[contractevent]`; migration tracked).

## Topics

| Topic | When | Data |
|---|---|---|
| `plan_created` | Merchant creates a plan | topics: merchant, plan_id; data: price |
| `plan_updated` | Merchant updates price | topics: plan_id; data: new_price |
| `plan_deactivated` | Merchant deactivates a plan | topics: merchant, plan_id; data: false |
| `plan_reactivated` | Merchant reactivates a plan | topics: merchant, plan_id; data: true |
| `subscribed` | User subscribes | topics: subscriber, plan_id; data: timestamp |
| `sub_cancelled` | Cancel or grace expiry | topics: subscriber, plan_id; data: timestamp |
| `sub_paused` | User pauses | topics: subscriber, plan_id; data: timestamp |
| `sub_resumed` | User resumes | topics: subscriber, plan_id; data: timestamp |
| `payment_executed` | Successful charge | topics: subscriber, plan_id; data: price |
| `payment_failed` | Failed charge recorded | topics: subscriber, plan_id; data: failed_attempts |

## Indexer notes

- Prefer matching on the first topic symbol.
- Grace-period cancel reuses `sub_cancelled` (same as voluntary cancel).
- `payment_failed` does **not** roll back; counters are committed on-chain.
- Pair `payment_failed` with `Subscription.failed_attempts` and optional `grace_deadline`
  to detect subscriptions entering the grace window.
- Successful payment clears grace: after `payment_executed`, expect
  `failed_attempts = 0` and `grace_deadline = 0` on the next read.

## Suggested indexer filters

```text
topic[0] in {
  plan_created, plan_updated, plan_deactivated, plan_reactivated,
  subscribed, sub_cancelled, sub_paused, sub_resumed,
  payment_executed, payment_failed
}
```

Downstream apps (Sorobill-Backend) typically project these into a relational
ledger for dunning and merchant dashboards.
