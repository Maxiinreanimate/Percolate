//! liquidate — anyone can call to close an undercollateralized position.

use anchor_lang::prelude::*;
use crate::state::{Market, UserAccount};
use crate::errors::PercolateError;
use crate::constants::*;

#[derive(Accounts)]
pub struct Liquidate<'info> {
    pub liquidator: Signer<'info>,

    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

pub fn handler(_ctx: Context<Liquidate>) -> Result<()> {
    // Settle target user funding + lazy A/K updates
    // Compute aggregate equity, check below maintenance margin
    // Find worst position via engine::liquidation::find_worst_position
    // Close that position via vAMM
    // Charge liquidation fee, split 50/50 between liquidator and insurance fund
    // If account still below maintenance, repeat
    // If account ends in deficit, socialize via A/K
    Ok(())
}
