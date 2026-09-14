# Substrata Contract Demo

## Prerequisites

- Rust + `wasm32-unknown-unknown`
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli)
- Funded testnet identity: `stellar keys generate substrata-admin --network testnet --fund`

## Build & test

```bash
make test
make build
```

## Deploy to testnet

```bash
make deploy NETWORK=testnet SOURCE=substrata-admin
# note the Contract ID
make init NETWORK=testnet SOURCE=substrata-admin
# then update DEPLOYMENTS.md
```

## Manual invoke examples

```bash
# Create a plan (merchant signs)
stellar contract invoke --id $CONTRACT_ID --source merchant --network testnet -- \
  create_plan --merchant $MERCHANT --name "Pro" --price 10000000 \
  --interval '{"tag":"Monthly"}' --token $USDC

# Subscribe
stellar contract invoke --id $CONTRACT_ID --source subscriber --network testnet -- \
  subscribe --subscriber $SUB --plan_id 0

# Bill (admin only)
stellar contract invoke --id $CONTRACT_ID --source substrata-admin --network testnet -- \
  execute_billing --caller $ADMIN --subscriber $SUB --plan_id 0
```
