# BillingOutcome

`execute_billing` returns `Result<BillingOutcome, SorobillError>`.

| Variant | Meaning | Events |
|---------|---------|--------|
| `Paid` | Tokens transferred; `next_billing` advanced | `payment_executed` |
| `Failed` | Insufficient balance recorded **or** grace cancel | `payment_failed` or `sub_cancelled` |

## Why not `Err(InsufficientBalance)`?

Soroban **rolls back all storage writes** when a contract function returns `Err`.
Recording failed attempts therefore requires returning `Ok(Failed)` after
persisting the counter (and optionally emitting `payment_failed`).

## Backend handling

```ts
const outcome = await invokeExecuteBilling(...);
if (outcome.tag === "Paid") {
  // advance off-chain schedule mirror
} else {
  // inspect failed_attempts / grace_deadline; queue dunning
}
```

Do not retry immediately on `Failed` unless balances changed; wait for the next
due window or operator policy.
