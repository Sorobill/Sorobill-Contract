# FAQ

**Why does failed billing return Ok?**  
So Soroban commits `failed_attempts` / grace state (errors roll back).

**Does pause stop grace?**  
Billing is blocked while paused; grace expiry is checked on the next
`execute_billing` attempt.

**Is description required?**  
Argument is required; empty string is allowed.

**Who can call execute_billing?**  
Only the address set in `initialize` (the billing admin / backend).

**Can a merchant change price mid-cycle?**  
Yes via `update_plan_price`. The new price applies on the next successful charge;
there is no on-chain proration in v0.3.

**What happens to existing subscribers when a plan is deactivated?**  
Existing subscriptions keep billing; only **new** `subscribe` calls are blocked
until `reactivate_plan`.

**How do I detect grace from an indexer?**  
Watch `payment_failed` with `failed_attempts >= 3`, then read
`grace_deadline` on the subscription.

**Which token standards are supported?**  
Any SEP-41 (Soroban token interface) asset the subscriber can approve.

**Where is the live demo UI?**  
https://sorobill-app.vercel.app (testnet). Contract IDs are in `DEPLOYMENTS.md`.
