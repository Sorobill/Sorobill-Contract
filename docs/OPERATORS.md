# Operator guide

## Grace period

```text
set_grace_period(caller=admin, secs)
get_grace_period() -> u64
```

- Default when unset: 172800 (48h)
- `0` = immediate cancel after `MAX_FAILED_ATTEMPTS`

## Billing

Only the initialized admin may call `execute_billing`. Keep the admin
secret in the Sorobill backend treasury/env.

## Monitoring

| Signal | Action |
|---|---|
| Spike in `payment_failed` | Check subscriber balances / allowances |
| `sub_cancelled` after grace | Notify merchant; offer re-subscribe flow |
| `BillingNotDue` from backend | Clock skew or duplicate scheduler ticks |
| Unexpected admin | Halt billing jobs; rotate keys |

## Storage TTL

Persistent `Plan` / `Sub` entries need periodic bumping (keeper). See
`SECURITY.md` known limitations and `KEEP_ALIVE.md` if present.
