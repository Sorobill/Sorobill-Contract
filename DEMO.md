# Sorobill Contract Demo

## Sister projects

| Repo / surface | Role |
|---|---|
| This repo (`Sorobill-Contract`) | On-chain subscription protocol |
| Sorobill-Backend | Billing scheduler / admin invoker |
| Sorobill-App | Merchant & subscriber UI |
| Live UI | https://sorobill-app.vercel.app |

See `DEPLOYMENTS.md` for the testnet contract id consumed by the live UI.

## Prerequisites

- Rust + `wasm32-unknown-unknown` (and Stellar CLI for `wasm32v1-none`)
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli)
- Funded testnet identity: `stellar keys generate sorobill-admin --network testnet --fund`

## Build & test

```bash
make verify
make build
```

## Deploy to testnet

```bash
make deploy NETWORK=testnet SOURCE=sorobill-admin
# note the Contract ID
make init NETWORK=testnet SOURCE=sorobill-admin
# then update DEPLOYMENTS.md
```

## Manual invoke examples

```bash
# Create a plan (merchant signs)
stellar contract invoke --id $CONTRACT_ID --source merchant --network testnet -- \
  create_plan --merchant $MERCHANT --name "Pro" --description "" --price 10000000 \
  --interval '{"tag":"Monthly"}' --token $USDC

# Subscribe
stellar contract invoke --id $CONTRACT_ID --source subscriber --network testnet -- \
  subscribe --subscriber $SUB --plan_id 0

# Bill (admin only)
stellar contract invoke --id $CONTRACT_ID --source sorobill-admin --network testnet -- \
  execute_billing --caller $ADMIN --subscriber $SUB --plan_id 0
```
