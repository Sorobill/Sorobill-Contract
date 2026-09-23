# Event schema sketch

Topics are symbols; additional topics carry addresses / ids; data is the payload.

| Symbol | Extra topics | Data |
|---|---|---|
| `plan_created` | merchant, plan_id | price |
| `plan_updated` | plan_id | new_price |
| `plan_deactivated` | merchant, plan_id | false |
| `plan_reactivated` | merchant, plan_id | true |
| `subscribed` | subscriber, plan_id | timestamp |
| `sub_cancelled` | subscriber, plan_id | timestamp |
| `sub_paused` | subscriber, plan_id | timestamp |
| `sub_resumed` | subscriber, plan_id | timestamp |
| `payment_executed` | subscriber, plan_id | price |
| `payment_failed` | subscriber, plan_id | failed_attempts |

See `EVENTS.md` for indexer guidance.
