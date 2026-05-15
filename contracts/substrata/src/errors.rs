use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum SubstrataError {
    Unauthorized = 1,
    PlanNotFound = 2,
    PlanInactive = 3,
    AlreadySubscribed = 4,
    SubscriptionNotFound = 5,
    SubscriptionInactive = 6,
    BillingNotDue = 7,
    InsufficientBalance = 8,
    InvalidInterval = 9,
    InvalidPrice = 10,
    SubscriptionPaused = 11,
    AlreadyPaused = 12,
    NotPaused = 13,
}
