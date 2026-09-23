# Monitoring guide

## On-chain signals

| Event | Alert idea |
|---|---|
| `payment_failed` | Rising failure rate per plan |
| `sub_cancelled` | Unexpected churn / grace expiry |
| `payment_executed` | Volume / revenue dashboards |
| `plan_deactivated` | Merchant ops change |

## Off-chain health

- Backend scheduler lag vs `next_billing`
- Admin key balance for fees
- RPC error rates on `execute_billing`
- Demo UI availability: https://sorobill-app.vercel.app

## Correlation

Join event topics with `get_subscription` snapshots to see
`failed_attempts` and `grace_deadline` after each `payment_failed`.
