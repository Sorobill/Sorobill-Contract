# Integration Smoke Verification Guide

> **Important**: This guide is **strictly Testnet-only**. Never execute these test commands against Stellar Public/Mainnet network passphrases.

This checklist provides step-by-step verification commands to validate the end-to-end subscription lifecycle on **Stellar Testnet** using active contract deployments documented in [`DEPLOYMENTS.md`](../DEPLOYMENTS.md).

---

## 1. Prerequisites & Testnet Configuration

Ensure your environment is configured for Testnet before invoking contract functions:

- [ ] Stellar CLI installed (`stellar --version` >= 21.0)
- [ ] Network configured for Testnet:
  ```bash
  stellar network add testnet \
    --rpc-url https://soroban-testnet.stellar.org \
    --network-passphrase "Test SDF Network ; September 2015"
  ```
- [ ] Active test identity funded via Friendbot:
  ```bash
  stellar keys generate --network testnet alice
  stellar keys fund --network testnet alice
  ```
- [ ] Active Contract ID verified from [`DEPLOYMENTS.md`](../DEPLOYMENTS.md):
  ```bash
  export CONTRACT_ID="CDENNEELMOUKIJGCSQUQ535FP53KRKNYA2PO7TOCI6O6IZVWZBYFML4W"
  ```

---

## 2. Core Lifecycle Smoke Checklist

### Step 1: Create Plan
- [ ] Call `create_plan` as merchant admin:
  ```bash
  stellar contract invoke \
    --id $CONTRACT_ID \
    --source-account alice \
    --network testnet \
    -- \
    create_plan \
    --plan_id 1 \
    --merchant $(stellar keys address alice) \
    --token $(stellar contract id asset --asset native --network testnet) \
    --amount 10000000 \
    --interval 86400 \
    --description "Test Daily Plan"
  ```
  **Expected Outcome**: Function returns `Ok(())` or commits plan struct to contract instance storage.

### Step 2: Subscribe
- [ ] Call `subscribe` with subscriber authorization:
  ```bash
  stellar contract invoke \
    --id $CONTRACT_ID \
    --source-account alice \
    --network testnet \
    -- \
    subscribe \
    --subscriber $(stellar keys address alice) \
    --plan_id 1
  ```
  **Expected Outcome**: Returns new `subscription_id` (e.g. `1`), sets `status: Active`, and initial period timestamp.

### Step 3: Execute Billing
- [ ] Call `execute_billing` as billing executor:
  ```bash
  stellar contract invoke \
    --id $CONTRACT_ID \
    --source-account alice \
    --network testnet \
    -- \
    execute_billing \
    --subscription_id 1
  ```
  **Expected Outcome**: Transfers token amount to merchant, emits `payment_success` contract event, and advances `next_billing_time`.

---

## 3. Expected Outcomes & Error Diagnostics

| Return / Error Code | Diagnostic & Resolution |
|---|---|
| `Ok(())` | Transaction committed successfully on Testnet ledger. |
| `Error(Contract, #1)` (`PlanNotFound`) | The requested `plan_id` does not exist or has been deactivated. |
| `Error(Contract, #2)` (`SubscriptionNotFound`) | The specified `subscription_id` has not been initialized. |
| `Error(Contract, #3)` (`GracePeriodExpired`) | Subscriber allowance depleted and grace window expired. |
| `Error(Contract, #4)` (`InsufficientAllowance`) | Subscriber token approval balance is lower than plan amount. |

---

## 4. On-Chain Auditing & Explorers

- [ ] Inspect deployment transactions on [Stellar Expert Testnet](https://stellar.expert/explorer/testnet):
  - View contract initialization and event streams by searching `$CONTRACT_ID`.
- [ ] Verify frontend demo parity at [https://sorobill-app.vercel.app](https://sorobill-app.vercel.app).