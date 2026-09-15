# Billing flow

1. Subscriber approves SEP-41 allowance to the Sorobill contract.
2. Backend admin calls `execute_billing` when due.
3. On success: transfer_from, reset failures, clear grace, advance `next_billing`.
4. On insufficient balance: increment failures; at max, set grace or cancel.
5. On grace expiry: next billing attempt cancels and returns `Failed`.
