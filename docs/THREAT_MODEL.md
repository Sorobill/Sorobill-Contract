# Threat model (Sorobill contract)

## Assets

- Subscriber token balances (via allowance)
- Merchant revenue stream integrity
- Admin billing authority

## Trust boundaries

- **Admin**: trusted billing backend; can call `execute_billing` and `set_grace_period`
- **Merchant**: creates/updates own plans only
- **Subscriber**: manages own subscription; must approve SEP-41 allowance

## Risks & mitigations

| Risk | Mitigation |
|---|---|
| Double charge | `next_billing` guard; billing not due → error |
| Admin key compromise | Operational key hygiene; future multi-sig roadmap |
| Allowance drain | Subscriber sets allowance + expiry; only plan price pulled per cycle |
| Griefing via pause | Pause is subscriber-controlled; billing blocked while paused |
| Premature cancel on failed payments | Configurable grace period (default 48h) |
| Unauthorized plan edits | Merchant address checked on update/deactivate |
| Replay of billing txs | Ledger timestamp + `next_billing` monotonic advance |

## Abuse scenarios

- **Dust failures**: attacker underfunds to burn scheduler cycles — mitigate off-chain with backoff.
- **Plan spam**: merchants can create many plans — mitigate with app-level rate limits.
- **Stale allowance**: billing returns `Failed` until allowance renewed.

## Out of scope (v0.3)

- On-chain governance of admin
- Formal verification
