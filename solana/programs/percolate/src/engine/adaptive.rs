//! Adaptive k controller.
//!
//! Continuously tunes the vAMM `k` based on real-time volume and
//! volatility. High volume or high volatility means more liquidity is
//! needed (higher k). Low volume means liquidity can shrink to be more
//! capital efficient.

use crate::constants::*;

/// Compute the target k for a market based on current conditions.
///
/// k_target = k_base * (1 + volume_factor + volatility_factor) / 3
///
/// Then clamped to [k_min, k_max].
pub fn compute_k_target(
    k_base: u128,
    k_min: u128,
    k_max: u128,
    volume_24h: u128,
    volume_avg_7d: u128,
    volatility_score: u32,
) -> u128 {
    let volume_factor_bps = if volume_avg_7d == 0 {
        10_000
    } else {
        ((volume_24h.saturating_mul(10_000)) / volume_avg_7d).min(50_000) as u32
    };

    let volatility_factor_bps = (volatility_score as u128 * 2).min(20_000) as u32;

    let combined_bps = (10_000 + volume_factor_bps + volatility_factor_bps) / 3;
    let raw_target = k_base.saturating_mul(combined_bps as u128) / 10_000;

    raw_target.clamp(k_min, k_max)
}

/// Smooth current k toward target over the smoothing window.
///
/// delta = (target - current) * elapsed / window
/// new_k = current + delta
pub fn smooth_toward_target(
    current_k: u128,
    target_k: u128,
    elapsed_seconds: u32,
    window_seconds: u32,
) -> u128 {
    if window_seconds == 0 || elapsed_seconds == 0 {
        return current_k;
    }

    let elapsed_capped = elapsed_seconds.min(window_seconds) as u128;
    let window = window_seconds as u128;

    if target_k > current_k {
        let diff = target_k - current_k;
        let delta = diff.saturating_mul(elapsed_capped) / window;
        current_k.saturating_add(delta)
    } else {
        let diff = current_k - target_k;
        let delta = diff.saturating_mul(elapsed_capped) / window;
        current_k.saturating_sub(delta)
    }
}

/// Update the rolling 7-day volume average with the latest 24h volume.
///
/// Uses exponential moving average with the configured decay rate.
pub fn update_volume_avg(current_avg: u128, new_volume_24h: u128) -> u128 {
    let decay = VOLUME_AVG_DECAY_BPS as u128;
    let keep = 10_000 - decay;
    (current_avg.saturating_mul(keep) + new_volume_24h.saturating_mul(decay)) / 10_000
}
