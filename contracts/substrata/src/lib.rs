#![no_std]

mod errors;
mod payments;
mod plans;
mod storage;
mod subscriptions;
mod types;

#[cfg(test)]
mod tests;

use soroban_sdk::{contract, contractimpl, Address, Env};

use errors::SubstrataError;
use types::{BillingInterval, Plan, Subscription};

#[contract]
pub struct SubstrataContract;

#[contractimpl]
impl SubstrataContract {
    // ── Initialisation ────────────────────────────────────────────────────────

    /// Set the admin (billing backend). Must be called once after deployment.
    pub fn initialize(e: Env, admin: Address) {
        admin.require_auth();
        storage::set_admin(&e, &admin);
    }

    // ── Plan Management ───────────────────────────────────────────────────────

    pub fn create_plan(
        e: Env,
        merchant: Address,
        price: i128,
        interval: BillingInterval,
        token: Address,
    ) -> Result<u64, SubstrataError> {
        plans::create_plan(&e, merchant, price, interval, token)
    }

    pub fn update_plan_price(
        e: Env,
        merchant: Address,
        plan_id: u64,
        new_price: i128,
    ) -> Result<(), SubstrataError> {
        plans::update_plan_price(&e, merchant, plan_id, new_price)
    }

    pub fn deactivate_plan(
        e: Env,
        merchant: Address,
        plan_id: u64,
    ) -> Result<(), SubstrataError> {
        plans::deactivate_plan(&e, merchant, plan_id)
    }

    pub fn get_plan(e: Env, plan_id: u64) -> Result<Plan, SubstrataError> {
        plans::get_plan(&e, plan_id)
    }

    // ── Subscription Management ───────────────────────────────────────────────

    pub fn subscribe(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<(), SubstrataError> {
        subscriptions::subscribe(&e, subscriber, plan_id)
    }

    pub fn cancel(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<(), SubstrataError> {
        subscriptions::cancel(&e, subscriber, plan_id)
    }

    pub fn pause(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<(), SubstrataError> {
        subscriptions::pause(&e, subscriber, plan_id)
    }

    pub fn resume(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<(), SubstrataError> {
        subscriptions::resume(&e, subscriber, plan_id)
    }

    pub fn get_subscription(
        e: Env,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<Subscription, SubstrataError> {
        subscriptions::get_subscription(&e, &subscriber, plan_id)
    }

    // ── Payment Execution ─────────────────────────────────────────────────────

    pub fn execute_billing(
        e: Env,
        caller: Address,
        subscriber: Address,
        plan_id: u64,
    ) -> Result<(), SubstrataError> {
        payments::execute_billing(&e, caller, subscriber, plan_id)
    }
}
