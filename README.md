# Substrata

> Stripe for recurring global payments — built on Stellar Soroban.

Substrata is an open-source, decentralized subscription payment protocol. It lets merchants create on-chain subscription plans and lets users authorize recurring payments — all without a centralized payment processor.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-soroban-blueviolet)](https://soroban.stellar.org)

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Build](#build)
  - [Test](#test)
  - [Deploy](#deploy)
- [Contract API](#contract-api)
  - [Initialization](#initialization)
  - [Plan Management](#plan-management)
  - [Subscription Management](#subscription-management)
  - [Payment Execution](#payment-execution)
- [Authorization Model](#authorization-model)
- [Events](#events)
- [Error Reference](#error-reference)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [Security](#security)
- [License](#license)

---

## Overview

Substrata solves a fundamental gap in Web3: **recurring payments**. Traditional DeFi is transactional — every payment requires an active user signature. Substrata introduces a trust-minimized model where:

1. A **merchant** creates a subscription plan (price, interval, token).
2. A **subscriber** approves the contract to spend tokens on their behalf (standard token allowance).
3. An **authorized billing backend** triggers payment execution at the correct interval.

No custodial wallets. No wrapped assets. No off-chain state.

---

## Features

| Feature | Status |
|---|---|
| Create / update / deactivate subscription plans | ✅ |
| Multi-asset support (any SEP-41 token) | ✅ |
| Subscribe / cancel | ✅ |
| Pause / resume subscriptions | ✅ |
| Allowance-based payment execution | ✅ |
| Double-charge prevention | ✅ |
| Auto-cancel after 3 failed billing attempts | ✅ |
| On-chain events for all state changes | ✅ |
| Upgradeable plan pricing | ✅ |

---

## Architecture

See [ARCHITECTURE.md](docs/ARCHITECTURE.md) for a full breakdown of the contract design, storage layout, and data flow.

---

## Project Structure

```
Substrata-Contract/
├── Cargo.toml                          # Workspace manifest
├── contracts/
│   └── substrata/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs                  # Contract entry point & public API
│           ├── types.rs                # Shared types: Plan, Subscription, DataKey, Events
│           ├── errors.rs               # SubstrataError enum
│           ├── storage.rs              # Thin storage helpers
│           ├── plans.rs                # Plan CRUD logic
│           ├── subscriptions.rs        # Subscribe / cancel / pause / resume
│           ├── payments.rs             # Billing execution
│           └── tests.rs                # Integration tests
└── docs/
    ├── ARCHITECTURE.md
    ├── CONTRIBUTING.md
    ├── SECURITY.md
    ├── CHANGELOG.md
    └── CODE_OF_CONDUCT.md
```

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable, 1.74+)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked soroban-cli
```

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled `.wasm` will be at:
```
target/wasm32-unknown-unknown/release/substrata.wasm
```

### Test

```bash
cargo test
```

### Deploy

```bash
# Deploy to Testnet
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/substrata.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet

# Initialize (set admin / billing backend)
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <ADMIN_SECRET_KEY> \
  --network testnet \
  -- initialize \
  --admin <ADMIN_ADDRESS>
```

---

## Contract API

### Initialization

```rust
fn initialize(e: Env, admin: Address)
```

Sets the billing backend address. Must be called once after deployment. The `admin` is the only address authorized to trigger `execute_billing`.

---

### Plan Management

#### `create_plan`

```rust
fn create_plan(
    e: Env,
    merchant: Address,
    price: i128,
    interval: BillingInterval,
    token: Address,
) -> Result<u64, SubstrataError>
```

Creates a new subscription plan. Returns the plan ID. `merchant` must sign.

**`BillingInterval` variants:**
- `Daily` — 86,400 seconds
- `Weekly` — 604,800 seconds
- `Monthly` — 2,592,000 seconds
- `Yearly` — 31,536,000 seconds
- `Custom(u64)` — any non-zero duration in seconds

#### `update_plan_price`

```rust
fn update_plan_price(e: Env, merchant: Address, plan_id: u64, new_price: i128) -> Result<(), SubstrataError>
```

Updates the price of an existing plan. Only the plan's merchant may call this.

#### `deactivate_plan`

```rust
fn deactivate_plan(e: Env, merchant: Address, plan_id: u64) -> Result<(), SubstrataError>
```

Prevents new subscriptions to this plan. Existing subscriptions continue until cancelled.

#### `get_plan`

```rust
fn get_plan(e: Env, plan_id: u64) -> Result<Plan, SubstrataError>
```

---

### Subscription Management

#### `subscribe`

```rust
fn subscribe(e: Env, subscriber: Address, plan_id: u64) -> Result<(), SubstrataError>
```

Subscribes the caller to a plan. The subscriber must have pre-approved the contract to spend at least `plan.price` of `plan.token` via the token's `approve` function.

#### `cancel`

```rust
fn cancel(e: Env, subscriber: Address, plan_id: u64) -> Result<(), SubstrataError>
```

Cancels an active subscription immediately.

#### `pause`

```rust
fn pause(e: Env, subscriber: Address, plan_id: u64) -> Result<(), SubstrataError>
```

Pauses billing. The subscription remains active but `execute_billing` will be rejected.

#### `resume`

```rust
fn resume(e: Env, subscriber: Address, plan_id: u64) -> Result<(), SubstrataError>
```

Resumes a paused subscription. Resets `next_billing` to `now + interval`.

#### `get_subscription`

```rust
fn get_subscription(e: Env, subscriber: Address, plan_id: u64) -> Result<Subscription, SubstrataError>
```

---

### Payment Execution

#### `execute_billing`

```rust
fn execute_billing(
    e: Env,
    caller: Address,
    subscriber: Address,
    plan_id: u64,
) -> Result<(), SubstrataError>
```

Triggers a billing cycle. Only the `admin` may call this. The contract uses `transfer_from` to move `plan.price` tokens from `subscriber` to `plan.merchant`.

**Safety checks performed:**
1. Caller must be admin
2. Subscription must be active and not paused
3. `now >= next_billing` (prevents double-charge)
4. Subscriber balance must be ≥ plan price

On failure, `failed_attempts` is incremented. At 3 failures the subscription is auto-cancelled.

---

## Authorization Model

Substrata uses the **token allowance pattern** (SEP-41 `approve` / `transfer_from`):

```
Subscriber → approve(contract, amount, expiry)  [once, off-chain or on-chain]
Admin      → execute_billing(...)               [each billing cycle]
Contract   → transfer_from(subscriber, merchant, price)
```

Subscribers retain full custody of their funds. The contract can only pull funds up to the approved allowance, and only when billing is due.

---

## Events

| Event | Topic | Data |
|---|---|---|
| `plan_created` | `(plan_created, merchant, plan_id)` | `price` |
| `plan_updated` | `(plan_updated, plan_id)` | `new_price` |
| `subscribed` | `(subscribed, subscriber, plan_id)` | `timestamp` |
| `payment_executed` | `(payment_executed, subscriber, plan_id)` | `amount` |
| `payment_failed` | `(payment_failed, subscriber, plan_id)` | `failed_attempts` |
| `sub_cancelled` | `(sub_cancelled, subscriber, plan_id)` | `timestamp` |
| `sub_paused` | `(sub_paused, subscriber, plan_id)` | `timestamp` |
| `sub_resumed` | `(sub_resumed, subscriber, plan_id)` | `timestamp` |

---

## Error Reference

| Code | Name | Description |
|---|---|---|
| 1 | `Unauthorized` | Caller is not permitted to perform this action |
| 2 | `PlanNotFound` | No plan exists with the given ID |
| 3 | `PlanInactive` | Plan has been deactivated |
| 4 | `AlreadySubscribed` | Subscriber already has an active subscription to this plan |
| 5 | `SubscriptionNotFound` | No subscription found for this (subscriber, plan_id) pair |
| 6 | `SubscriptionInactive` | Subscription has been cancelled |
| 7 | `BillingNotDue` | `next_billing` timestamp has not been reached yet |
| 8 | `InsufficientBalance` | Subscriber balance is below the plan price |
| 9 | `InvalidInterval` | Custom interval must be > 0 |
| 10 | `InvalidPrice` | Price must be > 0 |
| 11 | `SubscriptionPaused` | Billing attempted on a paused subscription |
| 12 | `AlreadyPaused` | Subscription is already paused |
| 13 | `NotPaused` | Resume called on a subscription that is not paused |

---

## Roadmap

- [ ] Grace period before auto-cancel
- [ ] Prorated billing on plan price changes
- [ ] Subscriber-side billing self-trigger
- [ ] Multi-sig admin / DAO governance
- [ ] Soroban contract upgrade path
- [ ] SDK / client library (TypeScript)

---

## Contributing

We welcome contributions of all kinds. See [CONTRIBUTING.md](docs/CONTRIBUTING.md) to get started.

---

## Security

Please do not open public issues for security vulnerabilities. See [SECURITY.md](docs/SECURITY.md) for the responsible disclosure process.

---

## License

[MIT](LICENSE) © Substrata Contributors
