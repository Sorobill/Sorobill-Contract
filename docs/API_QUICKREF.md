# API quick reference

One-line cheat sheet for integrators. Full details in `EXPORTS.md`.

```text
initialize(admin)
get_admin() / version() / plan_count() / is_subscribed(sub, plan)
set_grace_period(admin, secs) / get_grace_period()

create_plan(merchant, name, description, price, interval, token) -> id
update_plan_price(merchant, id, price)
deactivate_plan / reactivate_plan / get_plan

subscribe / cancel / pause / resume / get_subscription
execute_billing(admin, subscriber, plan_id) -> Paid | Failed
```

Live UI: https://sorobill-app.vercel.app
