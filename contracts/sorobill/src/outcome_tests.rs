#![cfg(test)]

use crate::types::BillingOutcome;

#[test]
fn billing_outcome_paid_ne_failed() {
    assert_ne!(BillingOutcome::Paid, BillingOutcome::Failed);
}

#[test]
fn billing_outcome_equality() {
    assert_eq!(BillingOutcome::Paid, BillingOutcome::Paid);
    assert_eq!(BillingOutcome::Failed, BillingOutcome::Failed);
}
