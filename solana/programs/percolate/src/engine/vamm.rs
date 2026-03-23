//! Virtual AMM (constant product).
//!
//! Implements the x*y=k pricing curve. Same model as Perk but with
//! adaptive k that changes over time via the adaptive controller.

use crate::constants::PEG_SCALE;
use crate::errors::PercolateError;
use anchor_lang::prelude::*;

/// Result of a vAMM swap simulation.
#[derive(Debug, Clone, Copy)]
pub struct SwapResult {
    pub new_base_reserve: u128,
    pub new_quote_reserve: u128,
    pub quote_delta: u128,
    pub effective_price: u128,
    pub slippage_bps: u16,
}

/// Compute the result of buying `base_size` from the vAMM (going long).
///
/// This decreases the base reserve and increases the quote reserve to
/// keep `base * quote = k` invariant.
pub fn simulate_buy(
    base_reserve: u128,
    quote_reserve: u128,
    k: u128,
    peg: u128,
    base_size: u128,
) -> Result<SwapResult> {
    require!(base_size > 0, PercolateError::PositionTooLarge);
    require!(base_size < base_reserve, PercolateError::PositionTooLarge);

    let new_base = base_reserve - base_size;
    let new_quote = k.checked_div(new_base).ok_or(PercolateError::DivisionByZero)?;
    let quote_delta = new_quote
        .checked_sub(quote_reserve)
        .ok_or(PercolateError::MathUnderflow)?;

    let effective_price = quote_delta
        .checked_mul(peg)
        .ok_or(PercolateError::MathOverflow)?
        / base_size;

    let oracle_price = quote_reserve.saturating_mul(peg) / base_reserve.max(1);
    let slippage_bps = compute_slippage_bps(oracle_price, effective_price);

    Ok(SwapResult {
        new_base_reserve: new_base,
        new_quote_reserve: new_quote,
        quote_delta,
        effective_price,
        slippage_bps,
    })
}

/// Compute the result of selling `base_size` into the vAMM (going short).
///
/// This increases the base reserve and decreases the quote reserve.
pub fn simulate_sell(
    base_reserve: u128,
    quote_reserve: u128,
    k: u128,
    peg: u128,
    base_size: u128,
) -> Result<SwapResult> {
    require!(base_size > 0, PercolateError::PositionTooLarge);

    let new_base = base_reserve
        .checked_add(base_size)
        .ok_or(PercolateError::MathOverflow)?;
    let new_quote = k.checked_div(new_base).ok_or(PercolateError::DivisionByZero)?;
    let quote_delta = quote_reserve
        .checked_sub(new_quote)
        .ok_or(PercolateError::MathUnderflow)?;

    let effective_price = quote_delta
        .checked_mul(peg)
        .ok_or(PercolateError::MathOverflow)?
        / base_size;

    let oracle_price = quote_reserve.saturating_mul(peg) / base_reserve.max(1);
    let slippage_bps = compute_slippage_bps(oracle_price, effective_price);

    Ok(SwapResult {
        new_base_reserve: new_base,
        new_quote_reserve: new_quote,
        quote_delta,
        effective_price,
        slippage_bps,
    })
}

/// Mark price = (quote_reserve * peg) / base_reserve.
pub fn mark_price(base_reserve: u128, quote_reserve: u128, peg: u128) -> u128 {
    if base_reserve == 0 {
        return 0;
    }
    quote_reserve.saturating_mul(peg) / base_reserve
}

/// Re-anchor the vAMM peg toward the oracle price.
pub fn compute_new_peg(current_peg: u128, oracle_price: u128, mark: u128) -> u128 {
    if mark == 0 {
        return current_peg;
    }
    // new_peg = current_peg * oracle_price / mark
    current_peg.saturating_mul(oracle_price) / mark
}

fn compute_slippage_bps(oracle: u128, effective: u128) -> u16 {
    if oracle == 0 {
        return 0;
    }
    let diff = if effective > oracle {
        effective - oracle
    } else {
        oracle - effective
    };
    let bps = diff.saturating_mul(10_000) / oracle;
    bps.min(u16::MAX as u128) as u16
}
