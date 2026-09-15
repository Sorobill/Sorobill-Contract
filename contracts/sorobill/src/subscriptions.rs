use soroban_sdk::{Address, Env};

use crate::{
    errors::SorobillError,
    storage,
    types::{Events, Subscription},
};

/// Subscribe a user to a plan. Billing starts immediately.
pub fn subscribe(
    e: &Env,
    subscriber: Address,
    plan_id: u64,
) -> Result<(), SorobillError> {
    subscriber.require_auth();

    let plan = storage::load_plan(e, plan_id).ok_or(SorobillError::PlanNotFound)?;
    if !plan.active {
        return Err(SorobillError::PlanInactive);
    }

    if storage::load_sub(e, &subscriber, plan_id).is_some() {
        return Err(SorobillError::AlreadySubscribed);
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
) -> Result<(), SorobillError> {
    subscriber.require_auth();

    let mut sub = storage::load_sub(e, &subscriber, plan_id)
        .ok_or(SorobillError::SubscriptionNotFound)?;
    if !sub.active {
        return Err(SorobillError::SubscriptionInactive);
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
) -> Result<(), SorobillError> {
    subscriber.require_auth();

    let mut sub = storage::load_sub(e, &subscriber, plan_id)
        .ok_or(SorobillError::SubscriptionNotFound)?;
    if !sub.active {
        return Err(SorobillError::SubscriptionInactive);
    }
    if sub.paused {
        return Err(SorobillError::AlreadyPaused);
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
) -> Result<(), SorobillError> {
    subscriber.require_auth();

    let mut sub = storage::load_sub(e, &subscriber, plan_id)
        .ok_or(SorobillError::SubscriptionNotFound)?;
    if !sub.active {
        return Err(SorobillError::SubscriptionInactive);
    }
    if !sub.paused {
        return Err(SorobillError::NotPaused);
    }

    let plan = storage::load_plan(e, plan_id).ok_or(SorobillError::PlanNotFound)?;
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
) -> Result<Subscription, SorobillError> {
    storage::load_sub(e, subscriber, plan_id).ok_or(SorobillError::SubscriptionNotFound)
}
