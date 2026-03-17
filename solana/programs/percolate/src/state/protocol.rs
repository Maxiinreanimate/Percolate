//! Protocol singleton account.
//!
//! Holds global configuration. Created once via `initialize_protocol`.
//! Only the admin can update most fields.

use anchor_lang::prelude::*;

#[account]
pub struct Protocol {
    /// Global admin (emergency pause + config updates only)
    pub admin: Pubkey,

    /// Global kill switch. When paused: no new positions, no deposits.
    /// Withdrawals and closes still work.
    pub paused: bool,

    /// Total markets created across all creators.
    pub market_count: u64,

    /// Protocol's fee collection account (USDC).
    pub protocol_fee_vault: Pubkey,

    // ─── Fee config ───
    /// Creator's share of trading fees (800 = 8%).
    pub creator_fee_share_bps: u16,

    /// Minimum trading fee a creator can set.
    pub min_trading_fee_bps: u16,

    /// Maximum trading fee a creator can set.
    pub max_trading_fee_bps: u16,

    /// Minimum k required to create a market.
    pub min_initial_k: u128,

    // ─── Cross-margin config ───
    /// Minimum equity required to keep a user account open.
    pub min_account_equity: u64,

    // ─── Multi-collateral config ───
    /// Number of registered collateral types.
    pub collateral_count: u8,

    // ─── Adaptive k config ───
    /// Whether the adaptive k controller is enabled globally.
    pub adaptive_k_enabled: bool,

    /// Smoothing window for the adaptive controller.
    pub adaptive_k_window_seconds: u32,

    // ─── Stats ───
    pub total_volume: u128,
    pub total_fees_collected: u128,

    pub bump: u8,
}

impl Protocol {
    pub const LEN: usize = 8 + // discriminator
        32 +  // admin
        1  +  // paused
        8  +  // market_count
        32 +  // protocol_fee_vault
        2  +  // creator_fee_share_bps
        2  +  // min_trading_fee_bps
        2  +  // max_trading_fee_bps
        16 +  // min_initial_k
        8  +  // min_account_equity
        1  +  // collateral_count
        1  +  // adaptive_k_enabled
        4  +  // adaptive_k_window_seconds
        16 +  // total_volume
        16 +  // total_fees_collected
        1;    // bump
}
