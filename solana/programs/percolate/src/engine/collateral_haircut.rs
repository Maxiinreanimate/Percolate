//! Multi-collateral haircut logic.
//!
//! Computes the dynamic haircut for volatile collateral based on its
//! 30-day price volatility. Stable assets keep a 0% haircut.

use crate::constants::{HAIRCUT_SCALE, MAX_DYNAMIC_HAIRCUT_BPS};

/// Compute the effective collateral value after applying haircut.
///
/// effective = raw * (1 - total_haircut / HAIRCUT_SCALE)
pub fn effective_collateral_value(
    raw_value: u128,
    static_haircut_bps: u16,
    dynamic_haircut_bps: u16,
) -> u128 {
    let total_haircut = (static_haircut_bps + dynamic_haircut_bps) as u128;
    let multiplier = (HAIRCUT_SCALE as u128).saturating_sub(total_haircut);
    raw_value.saturating_mul(multiplier) / HAIRCUT_SCALE as u128
}

/// Compute the dynamic haircut based on 30-day volatility.
///
/// volatility_score is 0-10000 where 10000 means extremely volatile.
/// dynamic_haircut = (volatility_score * MAX_DYNAMIC_HAIRCUT_BPS) / 10000
pub fn compute_dynamic_haircut(volatility_score: u32) -> u16 {
    let score = volatility_score.min(10_000) as u128;
    let max = MAX_DYNAMIC_HAIRCUT_BPS as u128;
    ((score * max) / 10_000) as u16
}
