# Billing intervals

| Variant | Seconds | Notes |
|---|---:|---|
| Daily | 86_400 | 24h wall clock |
| Weekly | 604_800 | 7 days |
| Monthly | 2_592_000 | 30-day month approximation |
| Yearly | 31_536_000 | 365-day year approximation |
| Custom(s) | `s` | Must be `s > 0` or `InvalidInterval` |

## Advancement rule

On successful payment:

```text
next_billing = now + plan.interval.as_secs()
```

On `resume` after pause, the same formula is applied from the resume timestamp
(pause does not accumulate “owed” cycles).

## Custom intervals

Use `Custom` for unusual cadences (e.g. biweekly = `1_209_600`). Integrators
should document the human-readable cadence in the merchant UI; the contract only
stores seconds.
