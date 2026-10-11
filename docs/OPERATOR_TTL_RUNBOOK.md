# Sorobill Operator: Ledger TTL & State Extension Runbook

Soroban Smart Contract instance and persistent storage entries require proactive TTL extension to prevent expiration.

## Storage Types & Expiration Behavior
- **Instance Storage**: Stores contract configuration and admin addresses.
- **Persistent Storage**: Stores active subscriber plans and balances.
- **Temporary Storage**: Ephemeral transaction nonces.

## Manual State Bump CLI Command
`ash
soroban contract bump \
  --id <CONTRACT_ID> \
  --network testnet \
  --source admin-key \
  --durability persistent \
  --ledgers-to-extend 500000
`

## Keeper Automation
Operators should maintain a periodic keeper job checking ledger sequence and executing extend_ttl before threshold falls below 10,000 ledgers.