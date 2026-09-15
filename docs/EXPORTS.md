# Exported functions

## Admin / meta
- `initialize`, `get_admin`, `version`
- `set_grace_period`, `get_grace_period`
- `plan_count`, `is_subscribed`

## Plans
- `create_plan(merchant, name, description, price, interval, token)`
- `update_plan_price`, `deactivate_plan`, `reactivate_plan`, `get_plan`

## Subscriptions
- `subscribe`, `cancel`, `pause`, `resume`, `get_subscription`

## Billing
- `execute_billing` → `BillingOutcome`
