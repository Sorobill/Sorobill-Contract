# ADR 001: Grace period before auto-cancel

## Status
Accepted (0.3.0)

## Context
Immediate cancel after 3 failed charges was harsh for temporary underfunding.

## Decision
Admin-configurable global grace seconds; on Nth failure set `grace_deadline`
instead of deactivating when grace > 0.

## Consequences
- Softens churn; requires indexers to watch grace + cancel events
- `create_plan` unrelated; separate description field landed in same release
