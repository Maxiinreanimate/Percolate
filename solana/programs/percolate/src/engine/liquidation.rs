//! Liquidation routing for cross-margin accounts.
//!
//! When an account falls below maintenance, we want to close the
//! position with the worst drawdown first. This minimizes the impact
//! on the rest of the portfolio.

use crate::state::user_account::PositionRef;
use crate::engine::position::position_unrealized_pnl;

/// Find the worst position in the account by absolute drawdown.
pub fn find_worst_position(
    positions: &[PositionRef],
    mark_prices: &[u128],
) -> Option<usize> {
    let mut worst_idx = None;
    let mut worst_drawdown: i128 = 0;

    for (i, (pos, mark)) in positions.iter().zip(mark_prices.iter()).enumerate() {
        if pos.is_empty() {
            continue;
        }
        let pnl = position_unrealized_pnl(pos, *mark);
        if pnl < worst_drawdown {
            worst_drawdown = pnl;
            worst_idx = Some(i);
        }
    }

    worst_idx
}

mod position {
    pub use crate::state::position::*;
}
