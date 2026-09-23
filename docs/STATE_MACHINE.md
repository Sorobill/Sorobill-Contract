# Subscription state machine

```
[none] --subscribe--> ACTIVE
ACTIVE --pause--> PAUSED
PAUSED --resume--> ACTIVE
ACTIVE|PAUSED --cancel--> INACTIVE
ACTIVE --3x fail + grace>0--> ACTIVE(grace_deadline set)
ACTIVE(grace) --deadline reached on bill--> INACTIVE
ACTIVE --3x fail + grace=0--> INACTIVE
ACTIVE --paid--> ACTIVE (counters cleared)
```

`is_subscribed` is true only while `active == true` (paused still counts).

## Field map

| Logical state | `active` | `paused` | `grace_deadline` |
|---|---|---|---|
| ACTIVE | true | false | 0 (or &gt;0 in grace) |
| PAUSED | true | true | unchanged |
| INACTIVE | false | * | usually cleared on grace cancel |
| none | (no storage row) | — | — |

## Notes

- Double `subscribe` on the same `(subscriber, plan_id)` key fails with
  `AlreadySubscribed` even if the prior row is inactive.
- Grace is orthogonal to pause: a paused sub is not billed, so grace expiry
  is evaluated only when billing is attempted.
