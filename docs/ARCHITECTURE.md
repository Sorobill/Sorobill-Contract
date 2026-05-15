# Substrata — Architecture

This document describes the internal design of the Substrata smart contract: its module layout, storage model, data flow, and key design decisions.

---

## Table of Contents

- [High-Level Design](#high-level-design)
- [Module Layout](#module-layout)
- [Data Model](#data-model)
  - [Plan](#plan)
  - [Subscription](#subscription)
  - [DataKey](#datakey)
- [Storage Strategy](#storage-strategy)
- [Authorization Model](#authorization-model)
- [Billing Flow](#billing-flow)
- [Failure Handling](#failure-handling)
- [Event System](#event-system)
- [Gas Considerations](#gas-considerations)
- [Known Limitations & Future Work](#known-limitations--future-work)

---

## High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                     Substrata Contract                      │
│                                                             │
│  ┌──────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │ plans.rs │  │subscriptions │  │     payments.rs      │  │
│  │          │  │    .rs       │  │                      │  │
│  │ create   │  │ subscribe    │  │  execute_billing     │  │
│  │ update   │  │ cancel       │  │  → transfer_from     │  │
│  │ deactivate│  │ pause/resume │  │  → emit events       │  │
│  └────┬─────┘  └──────┬───────┘  └──────────┬───────────┘  │
│       │               │                     │              │
│       └───────────────┴─────────────────────┘              │
│                       │                                     │
│               ┌───────▼────────┐                           │
│               │   storage.rs   │                           │
│               │  (persistent + │                           │
│               │   instance)    │                           │
│               └────────────────┘                           │
└─────────────────────────────────────────────────────────────┘
         │                              │
   SEP-41 Token                   Billing Backend
   (transfer_from)                (admin address)
```

---

## Module Layout

| File | Responsibility |
|---|---|
| `lib.rs` | Public contract interface. Thin dispatch layer — no business logic. |
| `types.rs` | All shared types: `Plan`, `Subscription`, `BillingInterval`, `DataKey`, `Events`. |
| `errors.rs` | `SubstrataError` enum with `#[contracterror]`. |
| `storage.rs` | Typed read/write helpers over `env.storage()`. No logic. |
| `plans.rs` | Plan CRUD: create, update price, deactivate, get. |
| `subscriptions.rs` | Subscribe, cancel, pause, resume, get. |
| `payments.rs` | Billing execution: auth check, timing check, token transfer, failure tracking. |
| `tests.rs` | Integration tests using the Soroban test framework. |

---

## Data Model

### Plan

```rust
pub struct Plan {
    pub merchant: Address,   // Plan owner; receives payments
    pub price: i128,         // Amount per billing cycle (in token's smallest unit)
    pub interval: BillingInterval, // How often to bill
    pub token: Address,      // SEP-41 token contract address
    pub active: bool,        // False = no new subscriptions allowed
}
```

### Subscription

```rust
pub struct Subscription {
    pub subscriber: Address, // Who is subscribed
    pub plan_id: u64,        // Which plan
    pub next_billing: u64,   // Unix timestamp when next charge is due
    pub active: bool,        // False = cancelled (by user or auto-cancel)
    pub failed_attempts: u32,// Consecutive billing failures
    pub last_charged: u64,   // Timestamp of last successful charge
    pub paused: bool,        // True = billing blocked by subscriber
}
```

### DataKey

```rust
pub enum DataKey {
    Admin,           // instance storage — billing backend address
    PlanCount,       // instance storage — monotonic plan ID counter
    Plan(u64),       // persistent storage — keyed by plan ID
    Sub(Address, u64), // persistent storage — keyed by (subscriber, plan_id)
}
```

---

## Storage Strategy

Soroban has three storage tiers. Substrata uses two:

| Tier | Used for | Rationale |
|---|---|---|
| `instance` | `Admin`, `PlanCount` | Contract-global singletons; live as long as the contract |
| `persistent` | `Plan(id)`, `Sub(addr, id)` | Long-lived data that must survive ledger expiry; TTL can be extended |
| `temporary` | — | Not used; subscriptions must not expire silently |

> **TTL management:** Callers (or a keeper bot) should periodically call `extend_ttl` on persistent entries to prevent expiry. A future version may bundle TTL extension into `execute_billing`.

---

## Authorization Model

```
Subscriber ──► token.approve(contract, price * N, expiry_ledger)
                        │
Admin ──────────────────► execute_billing(subscriber, plan_id)
                        │
                        ▼
              contract.transfer_from(subscriber → merchant, price)
```

- **Merchant auth**: required on `create_plan`, `update_plan_price`, `deactivate_plan`.
- **Subscriber auth**: required on `subscribe`, `cancel`, `pause`, `resume`.
- **Admin auth**: required on `execute_billing`. Admin is set at `initialize` and stored in instance storage.
- **No admin key rotation** in v0.1 — planned for a future multi-sig upgrade.

---

## Billing Flow

```
execute_billing(caller, subscriber, plan_id)
        │
        ├─ assert caller == admin
        ├─ load subscription → assert active, not paused
        ├─ assert now >= next_billing          (double-charge guard)
        ├─ load plan
        ├─ check subscriber balance >= price
        │       └─ on fail: increment failed_attempts
        │                   if failed_attempts >= 3: set active = false
        │                   emit payment_failed
        │                   return Err(InsufficientBalance)
        │
        ├─ token.transfer_from(contract, subscriber, merchant, price)
        ├─ update: last_charged = now, next_billing = now + interval, failed_attempts = 0
        └─ emit payment_executed
```

---

## Failure Handling

| Scenario | Behaviour |
|---|---|
| Subscriber has insufficient balance | `failed_attempts++`; error returned |
| 3 consecutive failures | Subscription auto-cancelled (`active = false`) |
| Billing called before due date | `BillingNotDue` error; no state change |
| Billing called on paused subscription | `SubscriptionPaused` error; no state change |
| Billing called by non-admin | `Unauthorized` error; no state change |

Failed attempts reset to 0 on any successful billing cycle.

---

## Event System

Events are published via `env.events().publish(topics, data)`. Topics are tuples of `Symbol` + relevant addresses/IDs for efficient indexing by off-chain listeners.

| Event | Topics | Data |
|---|---|---|
| `plan_created` | `(symbol, merchant, plan_id)` | `price: i128` |
| `plan_updated` | `(symbol, plan_id)` | `new_price: i128` |
| `subscribed` | `(symbol, subscriber, plan_id)` | `timestamp: u64` |
| `payment_executed` | `(symbol, subscriber, plan_id)` | `amount: i128` |
| `payment_failed` | `(symbol, subscriber, plan_id)` | `failed_attempts: u32` |
| `sub_cancelled` | `(symbol, subscriber, plan_id)` | `timestamp: u64` |
| `sub_paused` | `(symbol, subscriber, plan_id)` | `timestamp: u64` |
| `sub_resumed` | `(symbol, subscriber, plan_id)` | `timestamp: u64` |

---

## Gas Considerations

- **No loops** in any hot path. All operations are O(1).
- `DataKey` variants are compact `contracttype` enums — minimal XDR overhead.
- `Symbol::new` is called once per event publish; symbols are short (≤ 32 chars).
- `instance` storage is cheaper to access than `persistent`; admin and plan counter are kept there.
- `transfer_from` is a single cross-contract call — the most expensive operation per billing cycle.

---

## Known Limitations & Future Work

| Limitation | Planned Fix |
|---|---|
| Single admin key | Multi-sig / DAO governance module |
| No grace period on failure | Configurable grace period before auto-cancel |
| No prorated billing | Price-change epoch tracking |
| No TTL auto-extension | Bundle `extend_ttl` into `execute_billing` |
| No subscriber self-billing | Optional self-trigger function |
| No contract upgradeability | Soroban upgrade path via `update_current_contract_wasm` |
