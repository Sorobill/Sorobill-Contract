# ADR 001: Grace period before auto-cancel

## Status

Accepted (0.3.0)

## Context

Immediate cancel after 3 failed charges was harsh for temporary underfunding
(payroll timing, allowance expiry, short liquidity gaps).

## Decision

Admin-configurable global grace seconds; on Nth failure set `grace_deadline`
instead of deactivating when grace &gt; 0. Grace of `0` restores legacy
immediate cancel.

## Alternatives considered

1. **Per-plan grace** — more flexible, denser storage; deferred.
2. **Automatic retry without cancel** — unbounded zombie subs; rejected.
3. **Off-chain-only dunning** — weaker on-chain guarantees; rejected for core path.

## Consequences

- Softens churn; requires indexers to watch grace + cancel events.
- Successful pay clears grace atomically with failure counters.
- `create_plan` description field landed in the same release (unrelated).
