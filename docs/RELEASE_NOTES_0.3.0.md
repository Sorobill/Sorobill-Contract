# Release notes — 0.3.0

Grace periods, plan descriptions, and new view helpers for integrators.

## Highlights

- **Grace period**: after 3 failed charges, subscriptions stay active until
  `grace_deadline` (default 48h) unless grace is set to `0`.
- **Plan description**: `create_plan` accepts an on-chain `description` string
  (may be empty).
- **Views**: `version()`, `get_grace_period()`, `is_subscribed(subscriber, plan_id)`.

## Upgrade

Follow `MIGRATION_0.3.md`. Update TypeScript stubs under `clients/typescript`.

## References

- `CHANGELOG.md`
- `GRACE_PERIOD.md` / `ADR_001_GRACE.md`
- Testnet deploy: `DEPLOYMENTS.md`
- Live UI: https://sorobill-app.vercel.app
