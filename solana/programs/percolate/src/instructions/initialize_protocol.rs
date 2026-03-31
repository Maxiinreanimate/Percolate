//! initialize_protocol — one-time setup of the global protocol singleton.

use anchor_lang::prelude::*;
use crate::state::Protocol;
use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeProtocolParams {
    pub creator_fee_share_bps: u16,
    pub min_trading_fee_bps: u16,
    pub max_trading_fee_bps: u16,
    pub min_initial_k: u128,
    pub min_account_equity: u64,
}

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Protocol::LEN,
        seeds = [PROTOCOL_SEED],
        bump
    )]
    pub protocol: Account<'info, Protocol>,

    /// CHECK: Protocol fee vault, set by admin.
    pub protocol_fee_vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeProtocol>,
    params: InitializeProtocolParams,
) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;
    protocol.admin = ctx.accounts.admin.key();
    protocol.paused = false;
    protocol.market_count = 0;
    protocol.protocol_fee_vault = ctx.accounts.protocol_fee_vault.key();
    protocol.creator_fee_share_bps = params.creator_fee_share_bps;
    protocol.min_trading_fee_bps = params.min_trading_fee_bps;
    protocol.max_trading_fee_bps = params.max_trading_fee_bps;
    protocol.min_initial_k = params.min_initial_k;
    protocol.min_account_equity = params.min_account_equity;
    protocol.collateral_count = 0;
    protocol.adaptive_k_enabled = true;
    protocol.adaptive_k_window_seconds = ADAPTIVE_K_WINDOW_SECONDS;
    protocol.total_volume = 0;
    protocol.total_fees_collected = 0;
    protocol.bump = ctx.bumps.protocol;
    Ok(())
}
