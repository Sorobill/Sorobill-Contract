# Grace period

After `MAX_FAILED_ATTEMPTS` (3) consecutive failed charges, Sorobill can keep the
subscription alive for a **grace window** before auto-cancel.

## Behaviour

1. Admin sets grace with `set_grace_period(secs)` (default `172800` = 48 hours).
2. On the Nth failed billing (`BillingOutcome::Failed`), if grace &gt; 0 the contract
   stores `grace_deadline = now + grace_secs` instead of immediately deactivating.
3. Further `execute_billing` calls before the deadline still return `Failed` and do
   not transfer funds.
4. Once `now >= grace_deadline`, the next billing attempt cancels the subscription
   (`active = false`) and returns `Failed`.

Set grace to `0` to restore immediate auto-cancel after 3 failures.
