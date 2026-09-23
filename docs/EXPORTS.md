# Exported functions

## Admin / meta

| Function | Returns |
|---|---|
| `initialize(admin)` | `Result<(), SorobillError>` |
| `get_admin()` | `Result<Address, SorobillError>` |
| `version()` | `String` |
| `set_grace_period(caller, secs)` | `Result<(), SorobillError>` |
| `get_grace_period()` | `u64` |
| `plan_count()` | `u64` |
| `is_subscribed(subscriber, plan_id)` | `bool` |

## Plans

- `create_plan(merchant, name, description, price, interval, token) -> Result<u64, _>`
- `update_plan_price`, `deactivate_plan`, `reactivate_plan` → `Result<(), _>`
- `get_plan(plan_id) -> Result<Plan, _>`

## Subscriptions

- `subscribe`, `cancel`, `pause`, `resume` → `Result<(), _>`
- `get_subscription(subscriber, plan_id) -> Result<Subscription, _>`

## Billing

- `execute_billing(caller, subscriber, plan_id) -> Result<BillingOutcome, _>`

See `ERROR_CATALOG.md` for failure codes.
