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

## Out of scope (v0.3)
- On-chain governance of admin
- Formal verification
