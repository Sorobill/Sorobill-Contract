# Error catalog

`SorobillError` values (u32) returned by contract entrypoints.

| Code | Name | Meaning |
|---:|---|---|
| 1 | Unauthorized | Caller is not admin/merchant/subscriber as required |
| 2 | PlanNotFound | Unknown plan id |
| 3 | PlanInactive | Plan deactivated; new subscriptions blocked |
| 4 | AlreadySubscribed | Subscriber already has a record for plan |
| 5 | SubscriptionNotFound | No subscription for (subscriber, plan) |
| 6 | SubscriptionInactive | Subscription cancelled / inactive |
| 7 | BillingNotDue | `now < next_billing` |
| 8 | InsufficientBalance | Reserved; failures return `BillingOutcome::Failed` |
| 9 | InvalidInterval | Custom interval of 0 |
| 10 | InvalidPrice | Price ≤ 0 |
| 11 | SubscriptionPaused | Billing while paused |
| 12 | AlreadyPaused | Pause called twice |
| 13 | NotPaused | Resume on non-paused sub |
| 14 | InvalidPlanName | Empty plan name |
| 15 | AlreadyInitialized | `initialize` called twice |

Billing failures due to low balance return `Ok(BillingOutcome::Failed)` so state commits.
