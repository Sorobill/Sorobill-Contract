# Events

Current events use `Env::events().publish` (deprecated in soroban-sdk 23 in favor of `#[contractevent]`).

Topics emitted today:
- plan_created, plan_updated
- subscribed, sub_cancelled, sub_paused, sub_resumed
- payment_executed, payment_failed

Migration to `#[contractevent]` is tracked for a follow-up release.
