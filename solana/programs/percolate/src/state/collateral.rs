//! Multi-collateral registry.
//!
//! Each accepted collateral asset has its own entry. The protocol admin
//! can register new collateral but cannot remove existing ones.

use anchor_lang::prelude::*;

#[account]
pub struct CollateralEntry {
    /// SPL token mint of this collateral.
    pub mint: Pubkey,

    /// Token decimals (USDC = 6, SOL = 9, etc.).
    pub decimals: u8,

    /// Static portion of the haircut applied to this collateral (in bps).
    /// USDC = 0, SOL = 1500 (15%), wBTC = 1000 (10%), etc.
    pub haircut_bps: u16,

    /// Dynamic portion adjusted by 30-day price volatility.
    pub dynamic_haircut_bps: u16,

    /// Pyth oracle for pricing this collateral against USD.
    pub price_oracle: Pubkey,

    /// Whether this collateral is currently accepted for new deposits.
    pub enabled: bool,

    /// Index in the global collateral registry (0-based).
    pub index: u8,

    /// Total amount of this collateral deposited across all users.
    pub total_deposited: u64,

    /// Last time the dynamic haircut was updated.
    pub last_haircut_update: i64,

    pub bump: u8,
}

impl CollateralEntry {
    pub const LEN: usize = 8 +
        32 +  // mint
        1  +  // decimals
        2  +  // haircut_bps
        2  +  // dynamic_haircut_bps
        32 +  // price_oracle
        1  +  // enabled
        1  +  // index
        8  +  // total_deposited
        8  +  // last_haircut_update
        1;    // bump

    /// Total haircut as a fraction of HAIRCUT_SCALE.
    pub fn total_haircut_bps(&self) -> u16 {
        self.haircut_bps.saturating_add(self.dynamic_haircut_bps)
    }
}
