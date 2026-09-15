#![no_std]

mod errors;
mod payments;
mod plans;
mod storage;
mod subscriptions;
mod types;

#[cfg(test)]
mod tests;

use soroban_sdk::{contract, contractimpl, Address, Env, String};

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

    /// Total number of plans ever created.
    pub fn plan_count(e: Env) -> u64 {
        storage::plan_count(&e)
    }

    // ── Plan Management ───────────────────────────────────────────────────────

    pub fn create_plan(
        e: Env,
        merchant: Address,
        name: String,
        price: i128,
        interval: BillingInterval,
        token: Address,
    ) -> Result<u64, SorobillError> {
        plans::create_plan(&e, merchant, name, price, interval, token)
    }

    pub fn update_plan_price(
        e: Env,
        merchant: Address,
        plan_id: u64,
        new_price: i128,
    ) -> Result<(), SorobillError> {
        plans::update_plan_price(&e, merchant, plan_id, new_price)
    }

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
