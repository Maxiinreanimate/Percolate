//! Position helpers.
//!
//! In Percolate, positions are stored inline in the UserAccount as
//! PositionRef structs (see user_account.rs). This module provides
//! helper functions for position math.

use crate::state::user_account::PositionRef;

pub fn position_notional(pos: &PositionRef, mark_price: u128) -> u128 {
    if pos.base_size == 0 {
        return 0;
    }
    let base_abs = pos.base_size.unsigned_abs() as u128;
    base_abs.saturating_mul(mark_price) / 1_000_000
}

pub fn position_unrealized_pnl(pos: &PositionRef, mark_price: u128) -> i128 {
    if pos.base_size == 0 {
        return 0;
    }
    let base_abs = pos.base_size.unsigned_abs() as u128;
    let current_quote = base_abs.saturating_mul(mark_price) / 1_000_000;

    if pos.base_size > 0 {
        // Long: profit when current quote > entry quote
        current_quote as i128 - pos.quote_entry as i128
    } else {
        // Short: profit when entry quote > current quote
        pos.quote_entry as i128 - current_quote as i128
    }
}
