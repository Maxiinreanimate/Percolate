//! register_collateral — admin adds a new accepted collateral type.

use anchor_lang::prelude::*;
use crate::state::{Protocol, CollateralEntry};
use crate::errors::PercolateError;
use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RegisterCollateralParams {
    pub haircut_bps: u16,
    pub decimals: u8,
}

#[derive(Accounts)]
pub struct RegisterCollateral<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
        has_one = admin @ PercolateError::Unauthorized,
    )]
    pub protocol: Account<'info, Protocol>,

    /// CHECK: Mint of the collateral being registered.
    pub mint: UncheckedAccount<'info>,

    /// CHECK: Pyth oracle for pricing this collateral against USD.
    pub price_oracle: UncheckedAccount<'info>,

    #[account(
        init,
        payer = admin,
        space = CollateralEntry::LEN,
        seeds = [COLLATERAL_SEED, mint.key().as_ref()],
        bump
    )]
    pub collateral: Account<'info, CollateralEntry>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterCollateral>,
    params: RegisterCollateralParams,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let protocol = &mut ctx.accounts.protocol;

    let collateral = &mut ctx.accounts.collateral;
    collateral.mint = ctx.accounts.mint.key();
    collateral.decimals = params.decimals;
    collateral.haircut_bps = params.haircut_bps;
    collateral.dynamic_haircut_bps = 0;
    collateral.price_oracle = ctx.accounts.price_oracle.key();
    collateral.enabled = true;
    collateral.index = protocol.collateral_count;
    collateral.total_deposited = 0;
    collateral.last_haircut_update = now;
    collateral.bump = ctx.bumps.collateral;

    protocol.collateral_count = protocol.collateral_count.saturating_add(1);
    Ok(())
}
