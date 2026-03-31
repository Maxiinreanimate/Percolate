//! admin_update_protocol — update fee config and adaptive k window.

use anchor_lang::prelude::*;
use crate::state::Protocol;
use crate::errors::PercolateError;
use crate::constants::PROTOCOL_SEED;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct AdminUpdateProtocolParams {
    pub min_trading_fee_bps: Option<u16>,
    pub max_trading_fee_bps: Option<u16>,
    pub min_account_equity: Option<u64>,
    pub adaptive_k_enabled: Option<bool>,
    pub adaptive_k_window_seconds: Option<u32>,
}

#[derive(Accounts)]
pub struct AdminUpdateProtocol<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
        has_one = admin @ PercolateError::Unauthorized,
    )]
    pub protocol: Account<'info, Protocol>,
}

pub fn handler(
    ctx: Context<AdminUpdateProtocol>,
    params: AdminUpdateProtocolParams,
) -> Result<()> {
    let p = &mut ctx.accounts.protocol;
    if let Some(v) = params.min_trading_fee_bps {
        p.min_trading_fee_bps = v;
    }
    if let Some(v) = params.max_trading_fee_bps {
        p.max_trading_fee_bps = v;
    }
    if let Some(v) = params.min_account_equity {
        p.min_account_equity = v;
    }
    if let Some(v) = params.adaptive_k_enabled {
        p.adaptive_k_enabled = v;
    }
    if let Some(v) = params.adaptive_k_window_seconds {
        p.adaptive_k_window_seconds = v;
    }
    Ok(())
}
