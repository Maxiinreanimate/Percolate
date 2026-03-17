//! Per-market account.
//!
//! Each market is a PDA seeded by the token mint and creator. Holds the
//! vAMM state, risk engine state, funding state, and adaptive k state.

use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum OracleSource {
    Pyth,
    PercOracle,
    DexPool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum SideState {
    Normal,
    DrainOnly,
    ResetPending,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum Side {
    Long,
    Short,
}

#[account]
pub struct Market {
    // ─── Identity ───
    pub market_index: u64,
    pub token_mint: Pubkey,
    pub creator: Pubkey,
    pub creator_fee_account: Pubkey,

    // ─── vAMM state ───
    pub base_reserve: u128,
    pub quote_reserve: u128,
    pub k: u128,
    pub k_target: u128,
    pub k_base: u128,           // Original k at creation, used as reference
    pub k_min: u128,
    pub k_max: u128,
    pub k_last_adjusted: i64,
    pub peg_multiplier: u128,
    pub total_long_position: u128,
    pub total_short_position: u128,

    // ─── Adaptive k state ───
    pub volume_24h: u128,
    pub volume_avg_7d: u128,
    pub volatility_score: u32,    // 0-10000

    // ─── Market parameters (immutable after creation) ───
    pub max_leverage: u32,
    pub trading_fee_bps: u16,
    pub liquidation_fee_bps: u16,
    pub maintenance_margin_bps: u16,

    // ─── Oracle ───
    pub oracle_source: OracleSource,
    pub oracle_address: Pubkey,

    // ─── Risk engine state (Percolator H + A/K) ───
    pub insurance_fund_balance: u64,
    pub haircut_numerator: u128,
    pub haircut_denominator: u128,
    pub long_a: u128,
    pub long_k_index: i128,
    pub short_a: u128,
    pub short_k_index: i128,
    pub long_epoch: u64,
    pub short_epoch: u64,
    pub long_state: SideState,
    pub short_state: SideState,

    // ─── Funding ───
    pub last_funding_time: i64,
    pub cumulative_long_funding: i128,
    pub cumulative_short_funding: i128,
    pub funding_period_seconds: u32,
    pub funding_rate_cap_bps: u16,

    // ─── Warmup ───
    pub warmup_period_slots: u64,

    // ─── Fee tracking ───
    pub creator_fees_earned: u64,
    pub protocol_fees_earned: u64,
    pub total_volume: u128,

    // ─── State ───
    pub active: bool,
    pub bump: u8,
    pub created_at: i64,
}

impl Market {
    pub const LEN: usize = 8 +
        8 + 32 + 32 + 32 +              // identity
        16 * 11 + 8 +                   // vAMM (11 u128 + last_adjusted)
        16 + 16 + 4 +                   // adaptive k state
        4 + 2 + 2 + 2 +                 // params
        1 + 32 +                        // oracle
        8 + 16 + 16 +                   // H state
        16 + 16 + 16 + 16 +             // A/K state
        8 + 8 +                         // epochs
        1 + 1 +                         // states
        8 + 16 + 16 +                   // funding
        4 + 2 +                         //
        8 +                             // warmup
        8 + 8 + 16 +                    // fees
        1 + 1 + 8;                      // state

    pub fn mark_price(&self) -> u128 {
        if self.base_reserve == 0 {
            return 0;
        }
        self.quote_reserve
            .saturating_mul(self.peg_multiplier)
            / self.base_reserve
    }
}
