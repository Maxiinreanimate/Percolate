//! execute_trigger_order — anyone calls when the trigger condition is met.

use anchor_lang::prelude::*;
use crate::state::{TriggerOrder, Market};
use crate::errors::PercolateError;

#[derive(Accounts)]
pub struct ExecuteTriggerOrder<'info> {
    pub executor: Signer<'info>,

    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(mut, close = executor)]
    pub trigger_order: Account<'info, TriggerOrder>,
}

pub fn handler(_ctx: Context<ExecuteTriggerOrder>) -> Result<()> {
    // Read current oracle / mark price
    // Verify trigger condition is met based on order_type and side
    // Execute the trade via vAMM (open or close depending on reduce_only)
    // Pay executor 0.01% of notional
    // Close the trigger_order account
    Ok(())
}
