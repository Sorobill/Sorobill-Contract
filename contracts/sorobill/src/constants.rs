//! Shared protocol constants for Sorobill.

/// Auto-cancel after this many consecutive failed billing attempts
/// (subject to grace period when configured).
pub const MAX_FAILED_ATTEMPTS: u32 = 3;

/// Default grace window after max failures before the subscription is cancelled (48h).
pub const DEFAULT_GRACE_SECS: u64 = 172_800;

/// Semver string returned by the on-chain `version` view.
pub const CONTRACT_VERSION: &str = "0.3.0";
