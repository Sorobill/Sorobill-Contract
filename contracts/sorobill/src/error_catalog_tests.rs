#![cfg(test)]

use crate::errors::SorobillError;

#[test]
fn subscription_lifecycle_error_codes() {
    assert_eq!(SorobillError::AlreadySubscribed as u32, 4);
    assert_eq!(SorobillError::SubscriptionNotFound as u32, 5);
    assert_eq!(SorobillError::SubscriptionInactive as u32, 6);
}

#[test]
fn billing_guard_error_codes() {
    assert_eq!(SorobillError::BillingNotDue as u32, 7);
    assert_eq!(SorobillError::InsufficientBalance as u32, 8);
}

#[test]
fn plan_gate_error_codes() {
    assert_eq!(SorobillError::PlanInactive as u32, 3);
    assert_eq!(SorobillError::InvalidPlanName as u32, 14);
}
