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

**Why is Freighter failing to sign or showing network errors in the demo?**  
Freighter must be explicitly configured to **Testnet** under Settings > Network. If Freighter is set to Mainnet or Futurenet, transaction simulations and signature prompts targeting the Testnet deployment will fail. The live demo is available at [sorobill-app.vercel.app](https://sorobill-app.vercel.app).

**What if the demo app fails with contract not found or stale method errors?**  
Ensure your environment variable `NEXT_PUBLIC_SUBSCRIPTION_CONTRACT_ID` matches the active contract ID deployed in [`DEPLOYMENTS.md`](../DEPLOYMENTS.md). Stale or mismatched contract IDs will cause transaction invocations to revert.

**Where can I verify contract deployments and initialization transactions on-chain?**  
All deployed contract instances and initialization transactions can be inspected on [Stellar Expert Testnet](https://stellar.expert/explorer/testnet) by searching the contract ID or deployment account address documented in [`DEPLOYMENTS.md`](../DEPLOYMENTS.md).
