//! Typed storage helpers over instance and persistent maps.
//!
//! No business logic lives here — see `docs/STORAGE.md` for key layout.

use soroban_sdk::{Address, Env};

use crate::types::{DataKey, Plan, Subscription};

// ── Admin ────────────────────────────────────────────────────────────────────

pub fn set_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&DataKey::Admin, admin);
}

pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Admin)
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

pub fn plan_count(e: &Env) -> u64 {
    e.storage().instance().get(&DataKey::PlanCount).unwrap_or(0)
}

// ── Plans ────────────────────────────────────────────────────────────────────

pub fn next_plan_id(e: &Env) -> u64 {
    let id: u64 = e.storage().instance().get(&DataKey::PlanCount).unwrap_or(0);
    e.storage().instance().set(&DataKey::PlanCount, &(id + 1));
    id
}

pub fn save_plan(e: &Env, id: u64, plan: &Plan) {
    e.storage().persistent().set(&DataKey::Plan(id), plan);
}

pub fn load_plan(e: &Env, id: u64) -> Option<Plan> {
    e.storage().persistent().get(&DataKey::Plan(id))
}

// ── Subscriptions ─────────────────────────────────────────────────────────────

pub fn save_sub(e: &Env, sub: &Subscription) {
    let key = DataKey::Sub(sub.subscriber.clone(), sub.plan_id);
    e.storage().persistent().set(&key, sub);
}

pub fn load_sub(e: &Env, subscriber: &Address, plan_id: u64) -> Option<Subscription> {
    e.storage()
        .persistent()
        .get(&DataKey::Sub(subscriber.clone(), plan_id))
}

// ── Grace period ──────────────────────────────────────────────────────────────

pub fn set_grace_secs(e: &Env, secs: u64) {
    e.storage().instance().set(&DataKey::GraceSecs, &secs);
}

pub fn get_grace_secs(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get(&DataKey::GraceSecs)
        .unwrap_or(crate::constants::DEFAULT_GRACE_SECS)
}
