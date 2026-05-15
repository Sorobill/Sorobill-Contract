use soroban_sdk::{contracttype, Address, Symbol};

/// Supported billing intervals (in seconds).
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum BillingInterval {
    Daily,    // 86_400
    Weekly,   // 604_800
    Monthly,  // 2_592_000
    Yearly,   // 31_536_000
    Custom(u64),
}

impl BillingInterval {
    pub fn as_secs(&self) -> u64 {
        match self {
            BillingInterval::Daily => 86_400,
            BillingInterval::Weekly => 604_800,
            BillingInterval::Monthly => 2_592_000,
            BillingInterval::Yearly => 31_536_000,
            BillingInterval::Custom(s) => *s,
        }
    }
}

/// A subscription plan created by a merchant.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Plan {
    pub merchant: Address,
    pub price: i128,
    pub interval: BillingInterval,
    /// Token contract address (multi-asset support).
    pub token: Address,
    pub active: bool,
}

/// A user's subscription to a plan.
#[contracttype]
#[derive(Clone, Debug)]
pub struct Subscription {
    pub subscriber: Address,
    pub plan_id: u64,
    pub next_billing: u64,
    pub active: bool,
    pub failed_attempts: u32,
    pub last_charged: u64,
    pub paused: bool,
}

/// Storage key namespace.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    PlanCount,
    Plan(u64),
    Sub(Address, u64), // (subscriber, plan_id)
}

/// Event topics — kept as Symbol constants for gas efficiency.
pub struct Events;
impl Events {
    pub fn plan_created(e: &soroban_sdk::Env) -> Symbol {
        Symbol::new(e, "plan_created")
    }
    pub fn subscribed(e: &soroban_sdk::Env) -> Symbol {
        Symbol::new(e, "subscribed")
    }
    pub fn payment_ok(e: &soroban_sdk::Env) -> Symbol {
        Symbol::new(e, "payment_executed")
    }
    pub fn payment_fail(e: &soroban_sdk::Env) -> Symbol {
        Symbol::new(e, "payment_failed")
    }
    pub fn cancelled(e: &soroban_sdk::Env) -> Symbol {
        Symbol::new(e, "sub_cancelled")
    }
    pub fn paused(e: &soroban_sdk::Env) -> Symbol {
        Symbol::new(e, "sub_paused")
    }
    pub fn resumed(e: &soroban_sdk::Env) -> Symbol {
        Symbol::new(e, "sub_resumed")
    }
    pub fn plan_updated(e: &soroban_sdk::Env) -> Symbol {
        Symbol::new(e, "plan_updated")
    }
}
