# FAQ

**Why does failed billing return Ok?**  
So Soroban commits `failed_attempts` / grace state (errors roll back).

**Does pause stop grace?**  
Billing is blocked while paused; grace expiry is checked on the next
`execute_billing` attempt.

**Is description required?**  
Argument is required; empty string is allowed.
