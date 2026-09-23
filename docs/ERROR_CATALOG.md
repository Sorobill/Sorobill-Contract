# Error catalog

`SorobillError` values (u32) returned by contract entrypoints.

| Code | Name | Meaning | Typical recovery |
|---:|---|---|---|
| 1 | Unauthorized | Caller is not admin/merchant/subscriber as required | Sign with the correct address |
| 2 | PlanNotFound | Unknown plan id | Use a valid `plan_id` from `create_plan` |
| 3 | PlanInactive | Plan deactivated; new subscriptions blocked | Merchant calls `reactivate_plan` |
| 4 | AlreadySubscribed | Subscriber already has a record for plan | Use existing sub or cancel first |
| 5 | SubscriptionNotFound | No subscription for (subscriber, plan) | Subscribe first |
| 6 | SubscriptionInactive | Subscription cancelled / inactive | Create a new subscription (same key may still exist) |
| 7 | BillingNotDue | `now < next_billing` | Wait until due or adjust ledger time in tests |
| 8 | InsufficientBalance | Reserved; failures return `BillingOutcome::Failed` | Do not expect this as `Err` on billing |
| 9 | InvalidInterval | Custom interval of 0 | Pass `Custom(s)` with `s > 0` |
| 10 | InvalidPrice | Price ≤ 0 | Use a positive price |
| 11 | SubscriptionPaused | Billing while paused | Subscriber must `resume` |
| 12 | AlreadyPaused | Pause called twice | No-op; already paused |
| 13 | NotPaused | Resume on non-paused sub | Only resume when paused |
| 14 | InvalidPlanName | Empty plan name | Provide a non-empty name |
| 15 | AlreadyInitialized | `initialize` called twice | Deploy a fresh instance or skip init |

## Billing failures vs errors

Billing failures due to low balance return `Ok(BillingOutcome::Failed)` so state
commits. Hard errors (`Unauthorized`, `BillingNotDue`, `SubscriptionPaused`, …)
abort without mutating failure counters.
