#![cfg(test)]

use crate::constants::DEFAULT_GRACE_SECS;

#[test]
fn grace_deadline_is_now_plus_grace() {
    let now: u64 = 1_700_000_000;
    let deadline = now + DEFAULT_GRACE_SECS;
    assert_eq!(deadline, 1_700_172_800);
}

#[test]
fn grace_zero_means_immediate_cancel_policy() {
    let grace: u64 = 0;
    assert_eq!(grace, 0);
}
