//! settle_pnl — anyone calls to settle a user's funding and warmup state.

use anchor_lang::prelude::*;
use crate::state::{Market, UserAccount};

#[derive(Accounts)]
pub struct SettlePnl<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

pub fn handler(_ctx: Context<SettlePnl>) -> Result<()> {
    // Walk all positions in user_account that match this market
    // Apply funding deltas
    // Mature reserved_pnl based on warmup window
    // Apply lazy A/K updates from engine::risk
    Ok(())
}
