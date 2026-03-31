//! admin_pause — emergency pause of all trading.

use anchor_lang::prelude::*;
use crate::state::Protocol;
use crate::errors::PercolateError;
use crate::constants::PROTOCOL_SEED;

#[derive(Accounts)]
pub struct AdminPause<'info> {
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
        has_one = admin @ PercolateError::Unauthorized,
    )]
    pub protocol: Account<'info, Protocol>,
}

pub fn handler(ctx: Context<AdminPause>, paused: bool) -> Result<()> {
    ctx.accounts.protocol.paused = paused;
    Ok(())
}
