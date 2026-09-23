# Sorobill Deployments

Record of on-chain Sorobill contract deployments.

## Live UI

Testnet demo application: **https://sorobill-app.vercel.app**

Point the app at the Contract ID below (or your own deploy) via
`NEXT_PUBLIC_SUBSCRIPTION_CONTRACT_ID`.

## Testnet

| Field | Value |
|-------|-------|
| Network | Stellar Testnet |
| Passphrase | `Test SDF Network ; September 2015` |
| RPC | `https://soroban-testnet.stellar.org` |
| Contract ID | `CDENNEELMOUKIJGCSQUQ535FP53KRKNYA2PO7TOCI6O6IZVWZBYFML4W` |
| Admin | `GALOSD22UK656K2CP4VP4I45I3GSAZQXSEBFSO6CPZTCLU2QBJXZSZFI` |
| Wasm hash | `760bf344a0d874dfe3b7b47326d05bfd7c699880a789d4ae560c1807aee7008c` |
| Deploy tx | [5f1defa9…](https://stellar.expert/explorer/testnet/tx/5f1defa93b781d6c0957e743b0f0cbe23d63fec983bb79290426ecbc96af67e1) |
| Init tx | [9a83f193…](https://stellar.expert/explorer/testnet/tx/9a83f193b88fffbff06a3fd2bcd0c542260669cef8241a4f843d632f64e9d120) |
| Deployed at | 2026-09-15 |
| Lab | [Open in Stellar Lab](https://lab.stellar.org/r/testnet/contract/CDENNEELMOUKIJGCSQUQ535FP53KRKNYA2PO7TOCI6O6IZVWZBYFML4W) |
| Demo UI | [sorobill-app.vercel.app](https://sorobill-app.vercel.app) |

## Mainnet

Not deployed. Audit required before mainnet.

## How to update this file

After deploying:

```bash
make build
make deploy NETWORK=testnet SOURCE=sorobill-admin
./scripts/init.sh --network testnet --source sorobill-admin --id <CONTRACT_ID>
```

Then update the table fields (contract id, wasm hash, deploy/init txs) and
verify the live UI still resolves the new id.
