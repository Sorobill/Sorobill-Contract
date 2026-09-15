# Migrating to 0.3.0

## Breaking
- `create_plan` gains a `description: String` argument after `name`.
  Pass `String::from_str(&e, "")` if unused.

## Behavioural
- Default grace is 48h after 3 failures. Call `set_grace_period(0)` for
  legacy immediate cancel.

## New reads
- `version()`, `get_grace_period()`, `is_subscribed(subscriber, plan_id)`
