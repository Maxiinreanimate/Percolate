//! Risk engine — H + A/K from Percolator.
//!
//! Direct port of the math published in
//! https://github.com/aeyakovenko/percolator
//!
//! H is the haircut ratio that ensures exit fairness when the vault is
//! stressed. A/K are the lazy side indices that handle overhang clearing
//! without an ADL queue.

use crate::constants::POS_SCALE;

/// Compute the haircut ratio H.
///
/// H = min(Residual, ProfitTotal) / ProfitTotal
/// where Residual = max(0, Vault - Capital - Insurance)
///
/// Returns (numerator, denominator) so callers can apply the ratio
/// without intermediate floating point.
pub fn compute_haircut(
    vault_balance: u128,
    total_capital: u128,
    insurance: u128,
    matured_profit_total: u128,
) -> (u128, u128) {
    if matured_profit_total == 0 {
        return (1, 1);
    }

    let buffer = total_capital.saturating_add(insurance);
    let residual = vault_balance.saturating_sub(buffer);
    let numerator = residual.min(matured_profit_total);
    (numerator, matured_profit_total)
}

/// Apply the haircut to a single account's released profit.
///
/// effective_pnl = floor(released_pnl * H_num / H_denom)
pub fn apply_haircut(released_pnl: u128, h_num: u128, h_denom: u128) -> u128 {
    if h_denom == 0 {
        return 0;
    }
    released_pnl.saturating_mul(h_num) / h_denom
}

/// Compute effective position size from the lazy A index.
///
/// effective_pos(i) = floor(basis_i * A / a_snapshot_i)
pub fn effective_position(basis: i128, current_a: u128, a_snapshot: u128) -> i128 {
    if a_snapshot == 0 {
        return 0;
    }
    let basis_abs = basis.unsigned_abs();
    let scaled = basis_abs.saturating_mul(current_a) / a_snapshot;
    if basis < 0 {
        -(scaled as i128)
    } else {
        scaled as i128
    }
}

/// Compute lazy PnL delta for an account from K index movement.
///
/// pnl_delta(i) = floor(|basis_i| * (K - k_snapshot_i) / (a_snapshot_i * POS_SCALE))
pub fn lazy_pnl_delta(
    basis: i128,
    current_k: i128,
    k_snapshot: i128,
    a_snapshot: u128,
) -> i128 {
    if a_snapshot == 0 {
        return 0;
    }
    let basis_abs = basis.unsigned_abs();
    let k_diff = current_k.saturating_sub(k_snapshot);

    let denom = a_snapshot.saturating_mul(POS_SCALE);
    if denom == 0 {
        return 0;
    }

    let abs_diff = k_diff.unsigned_abs();
    let scaled = basis_abs.saturating_mul(abs_diff) / denom;

    if k_diff < 0 {
        -(scaled as i128)
    } else {
        scaled as i128
    }
}

/// Reduce the side A coefficient when a liquidation removes OI.
///
/// new_A = A * (oi - removed) / oi
pub fn reduce_side_a(current_a: u128, oi: u128, removed: u128) -> u128 {
    if oi == 0 {
        return current_a;
    }
    let remaining = oi.saturating_sub(removed);
    current_a.saturating_mul(remaining) / oi
}

/// Shift the side K index to socialize a deficit.
///
/// new_K = K - (deficit / oi)
pub fn socialize_deficit(current_k: i128, deficit: u128, oi: u128) -> i128 {
    if oi == 0 {
        return current_k;
    }
    let per_unit = (deficit / oi) as i128;
    current_k - per_unit
}
