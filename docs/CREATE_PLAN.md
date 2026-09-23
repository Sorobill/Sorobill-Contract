# create_plan

```
create_plan(merchant, name, description, price, interval, token) -> plan_id
```

## Validation

- `name` must be non-empty (`InvalidPlanName`)
- `description` may be empty
- `price` must be &gt; 0 (`InvalidPrice`)
- `Custom(0)` interval rejected (`InvalidInterval`)

## Effects

- Allocates the next monotonic `plan_id` (starting at 0).
- Persists an active `Plan` with merchant ownership.
- Emits `plan_created` with price in the data payload.

## Follow-ups

- `update_plan_price` — merchant-only price change
- `deactivate_plan` / `reactivate_plan` — gate new subscriptions
- `get_plan` — read full plan including description
