# Billing flow diagram

ASCII companion to `BILLING_FLOW.md`.

```text
                    +------------------+
                    |  Admin backend   |
                    +--------+---------+
                             |
                             v
 +-----------+     +---------+----------+     +------------+
 | Subscriber|     | Sorobill contract  |     |  Merchant  |
 | allowance |---->| execute_billing    |---->|  receives  |
 +-----------+     |  - due?            |     |  tokens    |
                   |  - grace expired?  |     +------------+
                   |  - balance ok?     |
                   +---------+----------+
                             |
              +--------------+--------------+
              |                             |
              v                             v
        BillingOutcome::Paid      BillingOutcome::Failed
        next_billing += interval  failed_attempts++ / grace / cancel
```

## Decision order

1. Auth (admin)
2. Sub active & not paused
3. Grace expiry cancel
4. Due date
5. Balance / transfer
