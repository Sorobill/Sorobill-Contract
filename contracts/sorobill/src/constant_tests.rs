#![cfg(test)]

use crate::constants::{CONTRACT_VERSION, DEFAULT_GRACE_SECS, MAX_FAILED_ATTEMPTS};

#[test]
fn max_failed_attempts_is_three() {
    assert_eq!(MAX_FAILED_ATTEMPTS, 3);
}

#[test]
fn default_grace_is_48_hours() {
    assert_eq!(DEFAULT_GRACE_SECS, 172_800);
    assert_eq!(DEFAULT_GRACE_SECS, 48 * 60 * 60);
}

#[test]
fn contract_version_semver() {
    assert_eq!(CONTRACT_VERSION, "0.3.0");
}
