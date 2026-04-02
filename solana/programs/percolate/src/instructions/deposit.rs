//! deposit — deposit any registered collateral into the user account.

use anchor_lang::prelude::*;
use crate::state::{UserAccount, CollateralEntry, Protocol};
use crate::errors::PercolateError;
use crate::constants::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
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

pub fn handler(ctx: Context<Deposit>, collateral_index: u8, amount: u64) -> Result<()> {
    require!(!ctx.accounts.protocol.paused, PercolateError::Paused);
    require!(ctx.accounts.collateral.enabled, PercolateError::CollateralDisabled);
    require!(
        ctx.accounts.collateral.index == collateral_index,
        PercolateError::CollateralNotRegistered
    );

    let acc = &mut ctx.accounts.user_account;
    let slot = acc
        .ensure_collateral_slot(collateral_index)
        .ok_or(PercolateError::CollateralLimitReached)?;

    acc.collateral_balances[slot].amount = acc.collateral_balances[slot]
        .amount
        .saturating_add(amount);

    let collateral = &mut ctx.accounts.collateral;
    collateral.total_deposited = collateral.total_deposited.saturating_add(amount);

    Ok(())
}
