#![cfg(test)]

use crate::errors::SorobillError;

#[test]
fn unauthorized_code_is_one() {
    assert_eq!(SorobillError::Unauthorized as u32, 1);
}

#[test]
fn plan_not_found_code() {
    assert_eq!(SorobillError::PlanNotFound as u32, 2);
}

#[test]
fn already_initialized_is_fifteen() {
    assert_eq!(SorobillError::AlreadyInitialized as u32, 15);
}

#[test]
fn invalid_price_and_interval_codes() {
    assert_eq!(SorobillError::InvalidInterval as u32, 9);
    assert_eq!(SorobillError::InvalidPrice as u32, 10);
}

#[test]
fn pause_related_error_codes() {
    assert_eq!(SorobillError::SubscriptionPaused as u32, 11);
    assert_eq!(SorobillError::AlreadyPaused as u32, 12);
    assert_eq!(SorobillError::NotPaused as u32, 13);
}
