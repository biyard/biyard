//! Test-only stub for on-chain reads.
//!
//! When the `disable-chain` feature is enabled, chain-dependent handlers
//! read from this module instead of calling `ethers` providers. Tests
//! inject values via the `set_*` helpers; production builds never
//! enable this feature.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

pub struct StubState {
    pub current_month: u64,
    pub monthly_ceilings: HashMap<u64, u128>,
}

impl Default for StubState {
    fn default() -> Self {
        Self {
            current_month: u64::MAX,
            monthly_ceilings: HashMap::new(),
        }
    }
}

fn state() -> &'static RwLock<StubState> {
    static STATE: OnceLock<RwLock<StubState>> = OnceLock::new();
    STATE.get_or_init(|| RwLock::new(StubState::default()))
}

pub fn set_current_month(m: u64) {
    state().write().unwrap().current_month = m;
}

pub fn set_monthly_ceiling(month: u64, ceiling: u128) {
    state()
        .write()
        .unwrap()
        .monthly_ceilings
        .insert(month, ceiling);
}

pub fn current_month() -> u64 {
    state().read().unwrap().current_month
}

pub fn monthly_ceiling(month: u64) -> u128 {
    state()
        .read()
        .unwrap()
        .monthly_ceilings
        .get(&month)
        .copied()
        .unwrap_or(0)
}

pub fn reset() {
    *state().write().unwrap() = StubState::default();
}
