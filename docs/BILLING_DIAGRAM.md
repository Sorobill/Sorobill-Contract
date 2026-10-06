# Billing flow diagram

Visual companion to `BILLING_FLOW.md` detailing the Sorobill recurring subscription lifecycle.

## Mermaid Sequence Diagram

The following sequence diagram outlines the end-to-end lifecycle: subscription initialization, SEP-41 token allowance approval, scheduled `execute_billing` execution (happy path vs. failure path), grace period triggering, and recovery vs. automatic cancellation.

```mermaid
sequenceDiagram
    autonumber
    actor Subscriber
    participant TokenContract as SEP-41 Token Contract
    participant Sorobill as Sorobill Contract
    actor Admin as Merchant / Admin Backend
    actor MerchantAcct as Merchant Token Account

    Note over Subscriber, Sorobill: 1. Subscription & Allowance Setup
    Subscriber->>Sorobill: subscribe(subscriber, plan_id)
    Sorobill-->>Subscriber: emit event: subscribed
    Subscriber->>TokenContract: approve(spender: Sorobill, amount, live_until)
    TokenContract-->>Subscriber: allowance confirmed

    Note over Admin, Sorobill: 2. Scheduled Billing Execution
    Admin->>Sorobill: execute_billing(subscriber, plan_id)

    alt Happy Path: Sufficient Allowance & Balance
        Sorobill->>TokenContract: transfer_from(spender: Sorobill, from: Subscriber, to: MerchantAcct, price)
        TokenContract-->>Sorobill: transfer success
        Sorobill->>Sorobill: Reset failed_attempts = 0, grace_deadline = 0
        Sorobill->>Sorobill: next_billing += interval.as_secs(), last_charged = now
        Sorobill-->>Admin: BillingOutcome::Paid
        Sorobill-->>Subscriber: emit event: payment_executed
    else Failure Path: Insufficient Balance / Allowance
        Sorobill->>TokenContract: transfer_from(price)
        TokenContract-->>Sorobill: transfer error (balance or allowance)
        Sorobill->>Sorobill: failed_attempts += 1
        alt failed_attempts < MAX_FAILED_ATTEMPTS
            Sorobill-->>Admin: BillingOutcome::Failed
            Sorobill-->>Subscriber: emit event: payment_failed
        else failed_attempts >= MAX_FAILED_ATTEMPTS (Enter Grace Period)
            Sorobill->>Sorobill: Set grace_deadline = now + grace_secs
            Sorobill-->>Admin: BillingOutcome::Failed (Grace Period Active)
            Sorobill-->>Subscriber: emit event: payment_failed
        end
    end

    Note over Subscriber, Admin: 3. Grace Expiry vs Recovery
    alt Recovery within Grace Window (now < grace_deadline)
        Subscriber->>TokenContract: top up balance & update allowance
        Admin->>Sorobill: execute_billing(subscriber, plan_id)
        Sorobill->>TokenContract: transfer_from(price)
        TokenContract-->>Sorobill: transfer success
        Sorobill->>Sorobill: Reset failed_attempts = 0, grace_deadline = 0
        Sorobill->>Sorobill: next_billing += interval.as_secs(), last_charged = now
        Sorobill-->>Admin: BillingOutcome::Paid (Recovered)
        Sorobill-->>Subscriber: emit event: payment_executed
    else Grace Period Expires (now >= grace_deadline)
        Admin->>Sorobill: execute_billing(subscriber, plan_id)
        Sorobill->>Sorobill: Guard detects now >= grace_deadline
        Sorobill->>Sorobill: Set active = false (Subscription Cancelled)
        Sorobill-->>Admin: BillingOutcome::Failed
        Sorobill-->>Subscriber: emit event: sub_cancelled
    end
```

---

## ASCII Overview

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

1. **Auth check**: Caller must match contract admin address.
2. **Subscription status**: Subscription must exist, be active (`active == true`), and not paused (`paused == false`).
3. **Grace expiry check**: If `grace_deadline > 0` and `env.ledger().timestamp() >= grace_deadline`, subscription is cancelled (`active = false`, emits `sub_cancelled`) and returns `BillingOutcome::Failed`.
4. **Due date check**: If current timestamp `< next_billing`, aborts with error `BillingNotDue`.
5. **Token transfer & settlement**:
   - Calls `token_client.transfer_from(spender, subscriber, merchant, price)`.
   - On success: clears failure count and grace deadline, advances `next_billing`, emits `payment_executed`, and returns `BillingOutcome::Paid`.
   - On failure: increments `failed_attempts`, optionally initializes `grace_deadline`, emits `payment_failed`, and returns `BillingOutcome::Failed`.
