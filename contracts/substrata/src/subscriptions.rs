use soroban_sdk::{Address, Env};

use crate::{
    errors::SubstrataError,
    storage,
    types::{Events, Subscription},
};

/// Subscribe a user to a plan. Billing starts immediately.
pub fn subscribe(
    e: &Env,
    subscriber: Address,
    plan_id: u64,
) -> Result<(), SubstrataError> {
    subscriber.require_auth();

    let plan = storage::load_plan(e, plan_id).ok_or(SubstrataError::PlanNotFound)?;
    if !plan.active {
        return Err(SubstrataError::PlanInactive);
    }

    if storage::load_sub(e, &subscriber, plan_id).is_some() {
        return Err(SubstrataError::AlreadySubscribed);
    }

    let now = e.ledger().timestamp();
    let sub = Subscription {
        subscriber: subscriber.clone(),
        plan_id,
        next_billing: now + plan.interval.as_secs(),
        active: true,
        failed_attempts: 0,
        last_charged: now,
        paused: false,
    };
    storage::save_sub(e, &sub);

    e.events()
        .publish((Events::subscribed(e), subscriber, plan_id), now);

    Ok(())
}

/// Cancel an active subscription.
pub fn cancel(
    e: &Env,
    subscriber: Address,
    plan_id: u64,
) -> Result<(), SubstrataError> {
    subscriber.require_auth();

    let mut sub = storage::load_sub(e, &subscriber, plan_id)
        .ok_or(SubstrataError::SubscriptionNotFound)?;
    if !sub.active {
        return Err(SubstrataError::SubscriptionInactive);
    }

    sub.active = false;
    storage::save_sub(e, &sub);

    e.events()
        .publish((Events::cancelled(e), subscriber, plan_id), e.ledger().timestamp());

    Ok(())
}

/// Pause a subscription (no billing while paused).
pub fn pause(
    e: &Env,
    subscriber: Address,
    plan_id: u64,
) -> Result<(), SubstrataError> {
    subscriber.require_auth();

    let mut sub = storage::load_sub(e, &subscriber, plan_id)
        .ok_or(SubstrataError::SubscriptionNotFound)?;
    if !sub.active {
        return Err(SubstrataError::SubscriptionInactive);
    }
    if sub.paused {
        return Err(SubstrataError::AlreadyPaused);
    }

    sub.paused = true;
    storage::save_sub(e, &sub);

    e.events()
        .publish((Events::paused(e), subscriber, plan_id), e.ledger().timestamp());

    Ok(())
}

/// Resume a paused subscription. Resets next_billing from now.
pub fn resume(
    e: &Env,
    subscriber: Address,
    plan_id: u64,
) -> Result<(), SubstrataError> {
    subscriber.require_auth();

    let mut sub = storage::load_sub(e, &subscriber, plan_id)
        .ok_or(SubstrataError::SubscriptionNotFound)?;
    if !sub.active {
        return Err(SubstrataError::SubscriptionInactive);
    }
    if !sub.paused {
        return Err(SubstrataError::NotPaused);
    }

    let plan = storage::load_plan(e, plan_id).ok_or(SubstrataError::PlanNotFound)?;
    let now = e.ledger().timestamp();

    sub.paused = false;
    sub.next_billing = now + plan.interval.as_secs();
    storage::save_sub(e, &sub);

    e.events()
        .publish((Events::resumed(e), subscriber, plan_id), now);

    Ok(())
}

pub fn get_subscription(
    e: &Env,
    subscriber: &Address,
    plan_id: u64,
) -> Result<Subscription, SubstrataError> {
    storage::load_sub(e, subscriber, plan_id).ok_or(SubstrataError::SubscriptionNotFound)
}
