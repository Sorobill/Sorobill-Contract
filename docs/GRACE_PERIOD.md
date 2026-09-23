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

## Timeline

```text
t0  subscribe (active, failed_attempts=0, grace_deadline=0)
t1  bill fail #1  → failed_attempts=1
t2  bill fail #2  → failed_attempts=2
t3  bill fail #3  → failed_attempts=3, grace_deadline=t3+grace
t4  bill attempt while t4 < grace_deadline → Failed (still active)
t5  bill attempt while t5 >= grace_deadline → cancel, Failed
```

## Successful recovery

If a charge succeeds while grace is active, the contract clears
`failed_attempts` and `grace_deadline` and advances `next_billing`.

## Related

- ADR: `ADR_001_GRACE.md`
- Constants: `DEFAULT_GRACE_SECS`, `MAX_FAILED_ATTEMPTS`
- Operators: `OPERATORS.md`
