# Glossary

| Term | Meaning |
|---|---|
| Admin | Address authorized to `execute_billing` and set grace |
| Merchant | Plan owner; receives `transfer_from` proceeds |
| Subscriber | User who approves allowance and manages sub lifecycle |
| Plan | On-chain product definition (price, interval, token) |
| Subscription | Per `(subscriber, plan_id)` billing state |
| Grace deadline | Unix time after which a failed sub is cancelled |
| BillingOutcome | `Paid` or `Failed` committed result of a bill attempt |
| SEP-41 | Soroban token interface used for allowances/transfers |
