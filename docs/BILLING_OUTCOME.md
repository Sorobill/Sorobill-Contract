# BillingOutcome

`execute_billing` returns `Result<BillingOutcome, SubstrataError>`.

| Variant | Meaning |
|---------|---------|
| `Paid` | Tokens transferred; `next_billing` advanced |
| `Failed` | Insufficient balance recorded; `failed_attempts` incremented |

## Why not `Err(InsufficientBalance)`?

Soroban **rolls back all storage writes** when a contract function returns `Err`.
Recording failed attempts therefore requires returning `Ok(Failed)` after
persisting the counter (and optionally emitting `payment_failed`).
