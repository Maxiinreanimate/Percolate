//! Kani proofs for conservation and safety properties.

#![cfg(kani)]

use percolate::engine::risk::*;

#[kani::proof]
fn proof_haircut_bounded() {
    let vault: u128 = kani::any();
    let capital: u128 = kani::any();
    let insurance: u128 = kani::any();
    let profit: u128 = kani::any();
    kani::assume(profit > 0 && profit < 2u128.pow(80));

    let (num, denom) = compute_haircut(vault, capital, insurance, profit);
    assert!(num <= denom);
}

#[kani::proof]
fn proof_haircut_never_exceeds_profit() {
    let vault: u128 = kani::any();
    let capital: u128 = kani::any();
    let insurance: u128 = kani::any();
    let profit: u128 = kani::any();
    kani::assume(profit > 0 && profit < 2u128.pow(80));

    let (num, _) = compute_haircut(vault, capital, insurance, profit);
    assert!(num <= profit);
}

#[kani::proof]
fn proof_apply_haircut_monotonic() {
    let pnl: u128 = kani::any();
    let h_num: u128 = kani::any();
    let h_denom: u128 = kani::any();
    kani::assume(pnl < 2u128.pow(80));
    kani::assume(h_denom > 0 && h_denom < 2u128.pow(80));
    kani::assume(h_num <= h_denom);

    let result = apply_haircut(pnl, h_num, h_denom);
    assert!(result <= pnl);
}
