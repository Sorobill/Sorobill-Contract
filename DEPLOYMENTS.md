# Substrata Deployments

Record of on-chain Substrata contract deployments.

## Testnet

| Field | Value |
|-------|-------|
| Network | Stellar Testnet |
| Passphrase | `Test SDF Network ; September 2015` |
| RPC | `https://soroban-testnet.stellar.org` |
| Contract ID | _pending deploy — run `make deploy`_ |
| Admin | _set during `make init`_ |
| Deployed at | — |
| Wasm hash | — |

## Mainnet

Not deployed. Audit required before mainnet.

## How to update this file

After deploying:

```bash
make build
make deploy NETWORK=testnet SOURCE=substrata-admin
make init NETWORK=testnet SOURCE=substrata-admin
```

Paste the printed contract ID and admin address into the table above and commit.
