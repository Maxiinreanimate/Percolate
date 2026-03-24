//! Funding rate calculation.
//!
//! Funding is the periodic payment between longs and shorts that keeps
//! the perp price anchored to the spot price. When the mark price is
//! above the oracle, longs pay shorts. When below, shorts pay longs.

use crate::errors::PercolateError;
use anchor_lang::prelude::*;

/// Compute the funding rate for the current period.
///
/// premium = (mark_price - oracle_price) / oracle_price
/// funding_rate = clamp(premium, -cap, +cap)
pub fn compute_funding_rate(
    mark_price: u128,
    oracle_price: u128,
    cap_bps: u16,
) -> Result<i128> {
    require!(oracle_price > 0, PercolateError::OracleStale);

    let premium_bps = if mark_price > oracle_price {
        let diff = mark_price - oracle_price;
        (diff.saturating_mul(10_000) / oracle_price) as i128
    } else {
        let diff = oracle_price - mark_price;
        -((diff.saturating_mul(10_000) / oracle_price) as i128)
    };

    let cap = cap_bps as i128;
    Ok(premium_bps.clamp(-cap, cap))
}

/// Apply funding to the cumulative indices.
///
/// Positive rate: longs pay shorts.
/// long_cumulative -= rate
/// short_cumulative += rate
pub fn apply_funding(
    long_cumulative: i128,
    short_cumulative: i128,
    rate_bps: i128,
) -> (i128, i128) {
    (long_cumulative - rate_bps, short_cumulative + rate_bps)
}

/// Compute the funding payment for an individual position.
pub fn compute_position_funding(
    base_size: i64,
    last_funding_index: i128,
    current_funding_index: i128,
) -> i128 {
    let delta = current_funding_index - last_funding_index;
    let base = base_size as i128;
    base * delta / 10_000
}
