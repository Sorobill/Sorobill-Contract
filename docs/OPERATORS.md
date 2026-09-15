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
