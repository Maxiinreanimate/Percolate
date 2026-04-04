//! update_amm — anyone calls to re-anchor the vAMM peg toward the oracle.

use anchor_lang::prelude::*;
use crate::state::Market;
use crate::engine::vamm::{mark_price, compute_new_peg};

#[derive(Accounts)]
pub struct UpdateAmm<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
}

pub fn handler(ctx: Context<UpdateAmm>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let mark = mark_price(market.base_reserve, market.quote_reserve, market.peg_multiplier);
    // Placeholder: read oracle price externally
    let oracle_price = mark;
    market.peg_multiplier = compute_new_peg(market.peg_multiplier, oracle_price, mark);
    Ok(())
}
