#![cfg(test)]

use crate::types::BillingInterval;

#[test]
fn named_intervals_are_distinct() {
    assert_ne!(BillingInterval::Daily, BillingInterval::Weekly);
    assert_ne!(BillingInterval::Monthly, BillingInterval::Yearly);
}

#[test]
fn custom_intervals_compare_by_seconds() {
    assert_eq!(BillingInterval::Custom(60), BillingInterval::Custom(60));
    assert_ne!(BillingInterval::Custom(60), BillingInterval::Custom(120));
}
