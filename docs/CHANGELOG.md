# Changelog

All notable changes to Substrata will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

### Planned
- Grace period before auto-cancel on billing failure
- Prorated billing when plan price changes mid-cycle
- Subscriber-side self-billing trigger
- Multi-sig admin / DAO governance
- Contract upgradeability via `update_current_contract_wasm`
- TypeScript SDK / client library

---

## [0.1.0] — 2026-05-15

### Added
- `initialize` — set admin (billing backend) address
- `create_plan` — merchant creates a subscription plan with price, interval, and token
- `update_plan_price` — merchant updates plan price (upgradeable pricing)
- `deactivate_plan` — merchant deactivates a plan; blocks new subscriptions
- `get_plan` — read plan state
- `subscribe` — user subscribes to a plan
- `cancel` — user cancels their subscription
- `pause` / `resume` — user pauses and resumes billing
- `get_subscription` — read subscription state
- `execute_billing` — admin triggers a billing cycle via token `transfer_from`
- Double-charge prevention via `next_billing` timestamp guard
- Auto-cancel after 3 consecutive billing failures
- On-chain events for all state transitions
- Multi-asset support (any SEP-41 token)
- Full integration test suite (10 tests)
