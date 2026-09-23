# Authorization model

| Entrypoint | Auth |
|---|---|
| `initialize` | New admin |
| `set_grace_period` | Current admin |
| `create_plan` / plan updates | Merchant |
| `subscribe` / `cancel` / `pause` / `resume` | Subscriber |
| `execute_billing` | Admin |
| Views (`get_*`, `version`, `is_subscribed`) | None |

## Implementation notes

- Mutating entrypoints call `Address::require_auth()` on the authorized party.
- Plan mutations additionally compare `plan.merchant` to the caller.
- Billing compares `caller` to the stored admin before `require_auth`.
- View functions never require auth and never mutate storage.

## Common mistakes

- Invoking `execute_billing` with the merchant or subscriber identity → `Unauthorized`.
- Updating another merchant's plan → `Unauthorized`.
- Calling `set_grace_period` before `initialize` → storage panic / missing admin.
