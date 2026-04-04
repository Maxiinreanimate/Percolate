//! crank_funding — anyone calls every funding period to update funding indices.

use anchor_lang::prelude::*;
use crate::state::Market;
use crate::engine::funding::{compute_funding_rate, apply_funding};
use crate::errors::PercolateError;

#[derive(Accounts)]
pub struct CrankFunding<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
}

pub fn handler(ctx: Context<CrankFunding>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let now = Clock::get()?.unix_timestamp;

    let elapsed = now - market.last_funding_time;
    require!(
        elapsed >= market.funding_period_seconds as i64,
        PercolateError::FundingPeriodNotElapsed
    );

    let mark = market.quote_reserve.saturating_mul(market.peg_multiplier)
        / market.base_reserve.max(1);
    // Placeholder: read oracle price externally
    let oracle_price = mark;

    let rate = compute_funding_rate(mark, oracle_price, market.funding_rate_cap_bps)?;
    let (new_long, new_short) = apply_funding(
        market.cumulative_long_funding,
        market.cumulative_short_funding,
        rate,
    );

    market.cumulative_long_funding = new_long;
    market.cumulative_short_funding = new_short;
    market.last_funding_time = now;

    Ok(())
}
