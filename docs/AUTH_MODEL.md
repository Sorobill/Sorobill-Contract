# Authorization model

| Entrypoint | Auth |
|---|---|
| `initialize` | New admin |
| `set_grace_period` | Current admin |
| `create_plan` / plan updates | Merchant |
| `subscribe` / `cancel` / `pause` / `resume` | Subscriber |
| `execute_billing` | Admin |
| Views (`get_*`, `version`, `is_subscribed`) | None |
