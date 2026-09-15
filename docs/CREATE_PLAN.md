# create_plan

```
create_plan(merchant, name, description, price, interval, token) -> plan_id
```

- `name` must be non-empty
- `description` may be empty
- `price` must be > 0
- `Custom(0)` interval rejected
