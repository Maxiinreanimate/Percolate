//! withdraw — withdraw collateral. Must maintain initial margin across all positions.

use anchor_lang::prelude::*;
use crate::state::{UserAccount, CollateralEntry, Protocol};
use crate::errors::PercolateError;
use crate::constants::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
    )]
    pub protocol: Account<'info, Protocol>,

    #[account(
        mut,
        seeds = [USER_ACCOUNT_SEED, authority.key().as_ref()],
        bump = user_account.bump,
        has_one = authority @ PercolateError::Unauthorized,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub collateral: Account<'info, CollateralEntry>,
}

pub fn handler(ctx: Context<Withdraw>, collateral_index: u8, amount: u64) -> Result<()> {
    let acc = &mut ctx.accounts.user_account;
    let slot = acc
        .find_collateral(collateral_index)
        .ok_or(PercolateError::CollateralNotRegistered)?;

    require!(
        acc.collateral_balances[slot].amount >= amount,
        PercolateError::InsufficientCollateral
    );

    acc.collateral_balances[slot].amount -= amount;

    // TODO: enforce that account remains above initial margin after withdrawal
    // by walking all positions and computing aggregate equity vs notional.

    let collateral = &mut ctx.accounts.collateral;
    collateral.total_deposited = collateral.total_deposited.saturating_sub(amount);

    Ok(())
}
