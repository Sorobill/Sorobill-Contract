# Storage layout

| Key | Persistence | Purpose |
|-----|-------------|---------|
| Admin | instance | Billing backend address |
| PlanCount | instance | Monotonic plan id counter |
| Plan(id) | persistent | Plan record |
| Sub(subscriber, plan_id) | persistent | Subscription record |

Persistent entries should be extended with TTL bumping in a future release for long-lived mainnet deployments.
