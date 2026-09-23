# Billing flow

End-to-end path from allowance approval to on-chain settlement.

## Steps

1. Subscriber approves SEP-41 allowance to the Sorobill contract.
2. Backend admin calls `execute_billing` when due.
3. On success: transfer_from, reset failures, clear grace, advance `next_billing`.
4. On insufficient balance: increment failures; at max, set grace or cancel.
5. On grace expiry: next billing attempt cancels and returns `Failed`.

## ASCII overview

```text
 Subscriber                Sorobill                 Merchant token acct
    |                         |                            |
    |-- approve(allowance) -->|                            |
    |                         |                            |
    |                    Admin (backend)                   |
    |                         |                            |
    |              execute_billing(sub, plan)              |
    |                         |                            |
    |                         |-- balance check -----------|
    |                         |-- transfer_from ---------->|
    |                         |-- emit payment_executed    |
    |                         |-- next_billing += interval |
```

## Guards (in order)

1. Caller must be admin (`Unauthorized` otherwise).
2. Subscription must exist, be active, and not paused.
3. If `grace_deadline` elapsed → cancel and return `Failed`.
4. If `now < next_billing` → `BillingNotDue`.
5. If balance &lt; price → record failure (possibly enter grace / cancel).
6. Else transfer and mark `Paid`.

See also `BILLING_OUTCOME.md` and `STATE_MACHINE.md`.
