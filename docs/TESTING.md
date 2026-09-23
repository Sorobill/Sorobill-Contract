# Testing

```bash
make verify          # cargo test + cargo check
cargo test -p sorobill
./scripts/smoke-integration.sh
```

## Suite map

| Area | Examples |
|---|---|
| Plan CRUD | create, invalid price/name, deactivate/reactivate |
| Subscribe lifecycle | subscribe, double-subscribe, cancel, pause/resume |
| Billing success | transfer, advance `next_billing`, clear failures |
| Billing failure | increment failures, auto-cancel, grace enter/expiry |
| Auth | unauthorized price update / billing |
| Meta | version, plan_count, is_subscribed, default grace |

## Tips

- Advance ledger time with `e.ledger().with_mut(...)` (see existing tests).
- Mint + approve via `setup_token` helper before successful billing tests.
- Prefer `try_*` client methods when asserting error codes.
