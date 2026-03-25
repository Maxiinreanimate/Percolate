//! Margin calculations for cross-margin accounts.

use crate::state::user_account::{PositionRef, UserAccount};
use crate::engine::position::*;

/// Compute the initial margin required for a single position.
pub fn initial_margin(notional: u128, leverage: u32) -> u128 {
    if leverage == 0 {
        return notional;
    }
    notional / (leverage as u128 / 100)
}

/// Compute the maintenance margin for a position.
pub fn maintenance_margin(notional: u128, mm_bps: u16) -> u128 {
    notional.saturating_mul(mm_bps as u128) / 10_000
}

/// Aggregate equity across the user's collateral balances and positions.
///
/// total_equity = sum(collateral_i * (1 - haircut_i)) + sum(unrealized_pnl)
pub fn account_equity(
    collateral_value_after_haircut: u128,
    total_unrealized_pnl: i128,
) -> i128 {
    collateral_value_after_haircut as i128 + total_unrealized_pnl
}

/// Aggregate maintenance margin across all positions in an account.
pub fn account_maintenance_margin(
    positions: &[PositionRef],
    mark_prices: &[u128],
    mm_bps: u16,
) -> u128 {
    let mut total = 0u128;
    for (pos, mark) in positions.iter().zip(mark_prices.iter()) {
        if pos.is_empty() {
            continue;
        }
        let notional = position_notional(pos, *mark);
        total = total.saturating_add(maintenance_margin(notional, mm_bps));
    }
    total
}

/// Check whether the account is healthy (above maintenance margin).
pub fn is_healthy(equity: i128, mm: u128) -> bool {
    equity >= 0 && (equity as u128) >= mm
}

/// Compute total notional across all positions.
pub fn account_total_notional(positions: &[PositionRef], mark_prices: &[u128]) -> u128 {
    let mut total = 0u128;
    for (pos, mark) in positions.iter().zip(mark_prices.iter()) {
        if pos.is_empty() {
            continue;
        }
        total = total.saturating_add(position_notional(pos, *mark));
    }
    total
}

mod position {
    pub use crate::state::position::*;
}
