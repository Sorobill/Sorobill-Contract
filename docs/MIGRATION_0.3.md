# Migrating to 0.3.0

## Breaking

- `create_plan` gains a `description: String` argument after `name`.
  Pass `String::from_str(&e, "")` if unused.

## Behavioural

- Default grace is 48h after 3 failures. Call `set_grace_period(0)` for
  legacy immediate cancel.

## New reads

- `version()`, `get_grace_period()`, `is_subscribed(subscriber, plan_id)`

## Integrator checklist

1. Update all `create_plan` call sites with `description`.
2. Decide grace policy; set explicitly after `initialize` if not defaulting.
3. Handle `BillingOutcome::Failed` without treating it as a transport error.
4. Index `grace_deadline` and clear-on-pay behaviour.
5. Bump TypeScript stubs / regenerate bindings.
6. Point apps at the new wasm / contract id in `DEPLOYMENTS.md`.
