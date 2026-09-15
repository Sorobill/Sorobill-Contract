use soroban_sdk::{Address, Env, String};

use crate::{
    errors::SorobillError,
    storage,
    types::{BillingInterval, Events, Plan},
};

/// Create a new subscription plan. Returns the plan ID.
pub fn create_plan(
    e: &Env,
    merchant: Address,
    name: String,
    description: String,
    price: i128,
    interval: BillingInterval,
    token: Address,
) -> Result<u64, SorobillError> {
    merchant.require_auth();

    if price <= 0 {
        return Err(SorobillError::InvalidPrice);
    }
    if name.len() == 0 {
        return Err(SorobillError::InvalidPlanName);
    }
    if let BillingInterval::Custom(s) = &interval {
        if *s == 0 {
            return Err(SorobillError::InvalidInterval);
        }
    }

    let id = storage::next_plan_id(e);
    let plan = Plan {
        merchant: merchant.clone(),
        name,
        description,
        price,
        interval,
        token,
        active: true,
    };
    storage::save_plan(e, id, &plan);

    e.events()
        .publish((Events::plan_created(e), merchant, id), price);

    Ok(id)
}

/// Update price on an existing plan (merchant only).
pub fn update_plan_price(
    e: &Env,
    merchant: Address,
    plan_id: u64,
    new_price: i128,
) -> Result<(), SorobillError> {
    merchant.require_auth();

    if new_price <= 0 {
        return Err(SorobillError::InvalidPrice);
    }

    let mut plan = storage::load_plan(e, plan_id).ok_or(SorobillError::PlanNotFound)?;
    if plan.merchant != merchant {
        return Err(SorobillError::Unauthorized);
    }

    plan.price = new_price;
    storage::save_plan(e, plan_id, &plan);

    e.events()
        .publish((Events::plan_updated(e), plan_id), new_price);

    Ok(())
}

/// Deactivate a plan so no new subscriptions can be created.
pub fn deactivate_plan(
    e: &Env,
    merchant: Address,
    plan_id: u64,
) -> Result<(), SorobillError> {
    merchant.require_auth();

    let mut plan = storage::load_plan(e, plan_id).ok_or(SorobillError::PlanNotFound)?;
    if plan.merchant != merchant {
        return Err(SorobillError::Unauthorized);
    }

    plan.active = false;
    storage::save_plan(e, plan_id, &plan);
    Ok(())
}

/// Reactivate a previously deactivated plan.
pub fn reactivate_plan(
    e: &Env,
    merchant: Address,
    plan_id: u64,
) -> Result<(), SorobillError> {
    merchant.require_auth();

    let mut plan = storage::load_plan(e, plan_id).ok_or(SorobillError::PlanNotFound)?;
    if plan.merchant != merchant {
        return Err(SorobillError::Unauthorized);
    }

    plan.active = true;
    storage::save_plan(e, plan_id, &plan);
    Ok(())
}

pub fn get_plan(e: &Env, plan_id: u64) -> Result<Plan, SorobillError> {
    storage::load_plan(e, plan_id).ok_or(SorobillError::PlanNotFound)
}
