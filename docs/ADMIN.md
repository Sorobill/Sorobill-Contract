# Admin role

The address passed to `initialize` is the only caller allowed to invoke:

- `execute_billing`
- `set_grace_period`

Views such as `get_admin`, `get_grace_period`, and `version` are public.

## Production guidance

In production this should be the Sorobill-Backend treasury keypair (or a multisig
controlling it). Treat the admin key as a hot operational secret:

1. Generate and fund the identity offline.
2. Call `initialize` exactly once after deploy.
3. Record the address in `DEPLOYMENTS.md` and backend env (`ADMIN_SECRET` / source name).
4. Prefer hardware or multisig before mainnet (roadmap).

## Checklist after deploy

```bash
stellar contract invoke --id $CONTRACT_ID --source sorobill-admin --network testnet -- \
  get_admin
stellar contract invoke --id $CONTRACT_ID --source sorobill-admin --network testnet -- \
  get_grace_period
```

Unexpected admin or grace values should block go-live.
