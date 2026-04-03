//! cancel_trigger_order — owner cancels their trigger order and reclaims rent.

use anchor_lang::prelude::*;
use crate::state::TriggerOrder;
use crate::errors::PercolateError;

#[derive(Accounts)]
pub struct CancelTriggerOrder<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        close = authority,
        has_one = authority @ PercolateError::Unauthorized,
    )]
    pub trigger_order: Account<'info, TriggerOrder>,
}

pub fn handler(_ctx: Context<CancelTriggerOrder>) -> Result<()> {
    Ok(())
}
