# Substrata Deployments

Record of on-chain Substrata contract deployments.

## Testnet

| Field | Value |
|-------|-------|
| Network | Stellar Testnet |
| Passphrase | `Test SDF Network ; September 2015` |
| RPC | `https://soroban-testnet.stellar.org` |
| Contract ID | `CDROWFTFKXEMSWGAOFAWGVCNSPDDGUAW2NSEW2O5JCHUSW6P6RCU2C2E` |
| Admin | `GDAGKJZDVIQU2IOSB7M6EI4NBFP6VGI6E2C7P7ARRM4ZPSFGAMKQ4J3Q` |
| Wasm hash | `c354c6f0880b1f4b244c41726581914b82dd7317b8c2d1266c032e547482a1e7` |
| Deploy tx | [14fff0f2…](https://stellar.expert/explorer/testnet/tx/14fff0f2511db1d5f56110146292656a8f71a3b582ff73b240fe4b43319c8e74) |
| Deployed at | 2026-09-14 |
| Lab | [Open in Stellar Lab](https://lab.stellar.org/r/testnet/contract/CDROWFTFKXEMSWGAOFAWGVCNSPDDGUAW2NSEW2O5JCHUSW6P6RCU2C2E) |

## Mainnet

Not deployed. Audit required before mainnet.

## How to update this file

After deploying:

```bash
make build
make deploy NETWORK=testnet SOURCE=substrata-admin
./scripts/init.sh --network testnet --source substrata-admin --id <CONTRACT_ID>
```
