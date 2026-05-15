use soroban_sdk::{Address, Env};

use crate::{
    errors::SubstrataError,
    storage,
    types::{BillingInterval, Events, Plan},
};

/// Create a new subscription plan. Returns the plan ID.
pub fn create_plan(
    e: &Env,
    merchant: Address,
    price: i128,
    interval: BillingInterval,
    token: Address,
) -> Result<u64, SubstrataError> {
    merchant.require_auth();

    if price <= 0 {
        return Err(SubstrataError::InvalidPrice);
    }
    if let BillingInterval::Custom(s) = &interval {
        if *s == 0 {
            return Err(SubstrataError::InvalidInterval);
        }
    }

    let id = storage::next_plan_id(e);
    let plan = Plan {
        merchant: merchant.clone(),
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
) -> Result<(), SubstrataError> {
    merchant.require_auth();

    if new_price <= 0 {
        return Err(SubstrataError::InvalidPrice);
    }

    let mut plan = storage::load_plan(e, plan_id).ok_or(SubstrataError::PlanNotFound)?;
    if plan.merchant != merchant {
        return Err(SubstrataError::Unauthorized);
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
) -> Result<(), SubstrataError> {
    merchant.require_auth();

    let mut plan = storage::load_plan(e, plan_id).ok_or(SubstrataError::PlanNotFound)?;
    if plan.merchant != merchant {
        return Err(SubstrataError::Unauthorized);
    }

    plan.active = false;
    storage::save_plan(e, plan_id, &plan);
    Ok(())
}

pub fn get_plan(e: &Env, plan_id: u64) -> Result<Plan, SubstrataError> {
    storage::load_plan(e, plan_id).ok_or(SubstrataError::PlanNotFound)
}
