# Troubleshooting

| Symptom | Likely cause | Fix |
|---|---|---|
| `Unauthorized` on bill | Wrong signer | Use admin from `get_admin` |
| `BillingNotDue` | Early invoke | Wait until `next_billing` |
| `SubscriptionPaused` | User paused | Call `resume` |
| Always `Failed` | Low balance/allowance | Fund + re-approve SEP-41 |
| `AlreadySubscribed` after cancel | Row still keyed | Do not re-subscribe same pair (v0.3) |
| Missing plan/sub | Storage TTL expired | Run keep-alive (`KEEP_ALIVE.md`) |

Demo UI: https://sorobill-app.vercel.app
