#![cfg(test)]

use crate::types::BillingInterval;

#[test]
fn daily_interval_seconds() {
    assert_eq!(BillingInterval::Daily.as_secs(), 86_400);
}

#[test]
fn weekly_interval_seconds() {
    assert_eq!(BillingInterval::Weekly.as_secs(), 604_800);
}

#[test]
fn monthly_interval_seconds() {
    assert_eq!(BillingInterval::Monthly.as_secs(), 2_592_000);
}

#[test]
fn yearly_interval_seconds() {
    assert_eq!(BillingInterval::Yearly.as_secs(), 31_536_000);
}

#[test]
fn custom_interval_seconds() {
    assert_eq!(BillingInterval::Custom(1_209_600).as_secs(), 1_209_600);
}
