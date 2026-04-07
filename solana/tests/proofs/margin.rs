//! Kani proofs for margin calculations.

#![cfg(kani)]

use percolate::engine::margin::*;

#[kani::proof]
fn proof_initial_margin_decreases_with_leverage() {
    let notional: u128 = kani::any();
    kani::assume(notional > 0 && notional < 2u128.pow(64));

    let m1 = initial_margin(notional, 100);   // 1x
    let m10 = initial_margin(notional, 1000); // 10x
    assert!(m10 <= m1);
}

#[kani::proof]
fn proof_maintenance_margin_proportional() {
    let notional: u128 = kani::any();
    kani::assume(notional < 2u128.pow(64));

    let mm = maintenance_margin(notional, 500); // 5%
    assert!(mm <= notional);
}

#[kani::proof]
fn proof_healthy_when_equity_positive() {
    let equity: i128 = kani::any();
    let mm: u128 = kani::any();
    kani::assume(equity >= 0 && (equity as u128) >= mm);
    assert!(is_healthy(equity, mm));
}
