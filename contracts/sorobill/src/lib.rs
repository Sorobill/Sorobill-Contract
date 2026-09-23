#![no_std]

mod constants;
mod errors;
mod payments;
mod plans;
mod storage;
mod subscriptions;
mod types;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod interval_tests;

#[cfg(test)]
mod error_tests;

#[cfg(test)]
mod constant_tests;

#[cfg(test)]
mod outcome_tests;

#[cfg(test)]
mod interval_eq_tests;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

use constants::CONTRACT_VERSION;
use errors::SorobillError;
use types::{BillingInterval, BillingOutcome, Plan, Subscription};

#[contract]
pub struct SorobillContract;

#[contractimpl]
impl SorobillContract {
    // ── Initialisation ────────────────────────────────────────────────────────

    /// Set the admin (billing backend). Must be called once after deployment.
    pub fn initialize(e: Env, admin: Address) -> Result<(), SorobillError> {
        if storage::has_admin(&e) {
            return Err(SorobillError::AlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&e, &admin);
        Ok(())
    }

    /// Return the billing admin address.
    pub fn get_admin(e: Env) -> Result<Address, SorobillError> {
        if !storage::has_admin(&e) {
            return Err(SorobillError::Unauthorized);
        }
        Ok(storage::get_admin(&e))
    }

    /// Semver of this contract build.
    pub fn version(_e: Env) -> String {
        String::from_str(&_e, CONTRACT_VERSION)
    }

    /// Configure grace seconds after max failed attempts (admin only).
    pub fn set_grace_period(e: Env, caller: Address, secs: u64) -> Result<(), SorobillError> {
        let admin = storage::get_admin(&e);
        if caller != admin {
            return Err(SorobillError::Unauthorized);
        }
        caller.require_auth();
        storage::set_grace_secs(&e, secs);
        Ok(())
    }

    pub fn get_grace_period(e: Env) -> u64 {
        storage::get_grace_secs(&e)
    }

    /// Total number of plans ever created.
    pub fn plan_count(e: Env) -> u64 {
        storage::plan_count(&e)
    }

    /// True if subscriber has an active (possibly paused) subscription record.
    pub fn is_subscribed(e: Env, subscriber: Address, plan_id: u64) -> bool {
        storage::load_sub(&e, &subscriber, plan_id)
            .map(|s| s.active)
            .unwrap_or(false)
    }

    // ── Plan Management ───────────────────────────────────────────────────────

    /// Create a merchant plan. `name` required; `description` may be empty.
    /// Rejects non-positive price and `Custom(0)` intervals.
    pub fn create_plan(
        e: Env,
        merchant: Address,
        name: String,
        description: String,
        price: i128,
        interval: BillingInterval,
        token: Address,
    ) -> Result<u64, SorobillError> {
        plans::create_plan(&e, merchant, name, description, price, interval, token)
    }

    /// Merchant-only price update; applies on the next successful charge.
    pub fn update_plan_price(
        e: Env,
        merchant: Address,
        plan_id: u64,
        new_price: i128,
    ) -> Result<(), SorobillError> {
        plans::update_plan_price(&e, merchant, plan_id, new_price)
    }

    /// Deactivate a plan so new subscriptions are rejected (`PlanInactive`).
    pub fn deactivate_plan(
        e: Env,
        merchant: Address,
        plan_id: u64,
    ) -> Result<(), SorobillError> {
        plans::deactivate_plan(&e, merchant, plan_id)
    }

    pub fn reactivate_plan(
        e: Env,
        merchant: Address,
        plan_id: u64,
    ) -> Result<(), SorobillError> {
        plans::reactivate_plan(&e, merchant, plan_id)
    }

    pub fn get_plan(e: Env, plan_id: u64) -> Result<Plan, SorobillError> {
        plans::get_plan(&e, plan_id)
    }

    // ── Subscription Management ───────────────────────────────────────────────

    pub fn subscribe(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<(), SorobillError> {
        subscriptions::subscribe(&e, subscriber, plan_id)
    }

    pub fn cancel(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<(), SorobillError> {
        subscriptions::cancel(&e, subscriber, plan_id)
    }

    pub fn pause(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<(), SorobillError> {
        subscriptions::pause(&e, subscriber, plan_id)
    }

    pub fn resume(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<(), SorobillError> {
        subscriptions::resume(&e, subscriber, plan_id)
    }

    pub fn get_subscription(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<Subscription, SorobillError> {
        subscriptions::get_subscription(&e, &subscriber, plan_id)
    }

    // ── Payment Execution ─────────────────────────────────────────────────────

    pub fn execute_billing(
        e: Env,
        caller: Address,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<BillingOutcome, SorobillError> {
        payments::execute_billing(&e, caller, subscriber, plan_id)
    }
}
