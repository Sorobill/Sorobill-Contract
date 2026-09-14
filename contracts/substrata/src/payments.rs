use soroban_sdk::{token, Address, Env};

use crate::{
    errors::SubstrataError,
    storage,
    types::{BillingOutcome, Events},
};

const MAX_FAILED_ATTEMPTS: u32 = 3;

/// Execute billing for a subscriber. Called by the authorized backend (admin).
/// Uses the token allowance the subscriber pre-approved to this contract.
///
/// Returns `BillingOutcome::Paid` on success or `BillingOutcome::Failed` when
/// the charge could not complete but failure state was recorded on-chain.
/// Returning `Ok` (not `Err`) for insufficient balance is intentional so the
/// failed-attempt counter is committed (Soroban rolls back state on `Err`).
pub fn execute_billing(
    e: &Env,
    caller: Address,
    subscriber: Address,
    plan_id: u64,
) -> Result<BillingOutcome, SubstrataError> {
    let admin = storage::get_admin(e);
    if caller != admin {
        return Err(SubstrataError::Unauthorized);
    }
    caller.require_auth();

    let mut sub = storage::load_sub(e, &subscriber, plan_id)
        .ok_or(SubstrataError::SubscriptionNotFound)?;

    if !sub.active {
        return Err(SubstrataError::SubscriptionInactive);
    }
    if sub.paused {
        return Err(SubstrataError::SubscriptionPaused);
    }

    let now = e.ledger().timestamp();

    if now < sub.next_billing {
        return Err(SubstrataError::BillingNotDue);
    }

    let plan = storage::load_plan(e, plan_id).ok_or(SubstrataError::PlanNotFound)?;
    let token_client = token::Client::new(e, &plan.token);

    let balance = token_client.balance(&subscriber);
    if balance < plan.price {
        sub.failed_attempts += 1;
        if sub.failed_attempts >= MAX_FAILED_ATTEMPTS {
            sub.active = false;
        }
        storage::save_sub(e, &sub);

        e.events().publish(
            (Events::payment_fail(e), subscriber.clone(), plan_id),
            sub.failed_attempts,
        );
        return Ok(BillingOutcome::Failed);
    }

    token_client.transfer_from(
        &e.current_contract_address(),
        &subscriber,
        &plan.merchant,
        &plan.price,
    );

    sub.last_charged = now;
    sub.next_billing = now + plan.interval.as_secs();
    sub.failed_attempts = 0;
    storage::save_sub(e, &sub);

    e.events().publish(
        (Events::payment_ok(e), subscriber, plan_id),
        plan.price,
    );

    Ok(BillingOutcome::Paid)
}
