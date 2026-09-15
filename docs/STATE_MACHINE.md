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
