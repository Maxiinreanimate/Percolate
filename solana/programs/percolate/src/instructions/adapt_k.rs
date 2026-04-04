//! adapt_k — anyone calls to tune the vAMM k based on volume and volatility.

use anchor_lang::prelude::*;
use crate::state::Market;
use crate::engine::adaptive::{compute_k_target, smooth_toward_target};
use crate::errors::PercolateError;
use crate::constants::ADAPTIVE_K_MIN_INTERVAL;

#[derive(Accounts)]
pub struct AdaptK<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
}

pub fn handler(ctx: Context<AdaptK>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let now = Clock::get()?.unix_timestamp;

    let elapsed = (now - market.k_last_adjusted) as u32;
    require!(
        elapsed >= ADAPTIVE_K_MIN_INTERVAL,
        PercolateError::AdaptKIntervalNotElapsed
    );

    market.k_target = compute_k_target(
        market.k_base,
        market.k_min,
        market.k_max,
        market.volume_24h,
        market.volume_avg_7d,
        market.volatility_score,
    );

    market.k = smooth_toward_target(
        market.k,
        market.k_target,
        elapsed,
        crate::constants::ADAPTIVE_K_WINDOW_SECONDS,
    );

    market.k_last_adjusted = now;
    Ok(())
}
