//! Per-market insurance fund.
//!
//! Funded by 50% of liquidation fees. Used to absorb bad debt before
//! the deficit gets socialized through the A/K mechanism.

use anchor_lang::prelude::*;

#[account]
pub struct InsuranceFund {
    pub market: Pubkey,
    pub balance: u64,
    pub total_inflows: u64,
    pub total_outflows: u64,
    pub bump: u8,
}

impl InsuranceFund {
    pub const LEN: usize = 8 +
        32 + 8 + 8 + 8 + 1;
}
