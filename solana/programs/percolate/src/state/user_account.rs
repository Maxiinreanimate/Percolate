//! Cross-margin user account.
//!
//! A single user account holds multi-asset collateral and references
//! positions across many markets. This is the core cross-margin
//! abstraction that distinguishes Percolate from Perk.

use anchor_lang::prelude::*;
use crate::constants::{MAX_POSITIONS_PER_ACCOUNT, MAX_COLLATERAL_PER_ACCOUNT};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug)]
pub struct CollateralBalance {
    pub collateral_index: u8,
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default, Debug)]
pub struct PositionRef {
    pub market: Pubkey,
    pub base_size: i64,
    pub quote_entry: u128,
    pub last_funding_index: i128,
    pub a_snapshot: u128,
    pub k_snapshot: i128,
    pub epoch_snapshot: u64,
    pub reserved_pnl: u64,
    pub warmup_started_at: u64,
    pub leverage: u32,
}

impl PositionRef {
    pub fn is_empty(&self) -> bool {
        self.base_size == 0 && self.quote_entry == 0
    }
}

#[account]
pub struct UserAccount {
    pub authority: Pubkey,

    /// Multi-collateral balances. Each entry tracks a different collateral type.
    pub collateral_balances: [CollateralBalance; MAX_COLLATERAL_PER_ACCOUNT],

    /// Position references across all markets the user has touched.
    pub positions: [PositionRef; MAX_POSITIONS_PER_ACCOUNT],
    pub position_count: u8,

    /// Aggregate cross-margin state.
    pub total_margin_used: u64,
    pub total_unrealized_pnl: i64,
    pub total_realized_pnl: i64,

    /// Risk engine per-account state.
    pub fee_credits: i64,
    pub last_settled_at: i64,

    pub bump: u8,
    pub created_at: i64,
}

impl UserAccount {
    pub const LEN: usize = 8 +
        32 +
        (1 + 8) * MAX_COLLATERAL_PER_ACCOUNT +
        (32 + 8 + 16 + 16 + 16 + 16 + 8 + 8 + 8 + 4) * MAX_POSITIONS_PER_ACCOUNT +
        1 +
        8 + 8 + 8 +
        8 + 8 +
        1 + 8;

    /// Find a position reference for the given market, if it exists.
    pub fn find_position(&self, market: &Pubkey) -> Option<usize> {
        self.positions
            .iter()
            .take(self.position_count as usize)
            .position(|p| &p.market == market && !p.is_empty())
    }

    /// Allocate a new slot for a position. Returns None if at limit.
    pub fn allocate_position_slot(&mut self) -> Option<usize> {
        if (self.position_count as usize) >= MAX_POSITIONS_PER_ACCOUNT {
            return None;
        }
        let slot = self.position_count as usize;
        self.position_count += 1;
        Some(slot)
    }

    /// Find a collateral balance entry for the given collateral index.
    pub fn find_collateral(&self, index: u8) -> Option<usize> {
        self.collateral_balances
            .iter()
            .position(|b| b.collateral_index == index && b.amount > 0)
    }

    /// Find or allocate a collateral balance entry.
    pub fn ensure_collateral_slot(&mut self, index: u8) -> Option<usize> {
        if let Some(slot) = self.find_collateral(index) {
            return Some(slot);
        }
        // Allocate first empty slot
        for (i, b) in self.collateral_balances.iter().enumerate() {
            if b.amount == 0 {
                self.collateral_balances[i] = CollateralBalance {
                    collateral_index: index,
                    amount: 0,
                };
                return Some(i);
            }
        }
        None
    }
}
