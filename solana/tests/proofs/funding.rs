//! Kani proofs for funding rate calculations.

#![cfg(kani)]

use percolate::engine::funding::*;

#[kani::proof]
fn proof_funding_rate_clamped() {
    let mark: u128 = kani::any();
    let oracle: u128 = kani::any();
    let cap: u16 = kani::any();
    kani::assume(oracle > 0 && oracle < 2u128.pow(80));
    kani::assume(mark < 2u128.pow(80));
    kani::assume(cap > 0 && cap <= 1000);

    if let Ok(rate) = compute_funding_rate(mark, oracle, cap) {
        assert!(rate.abs() <= cap as i128);
    }
}

#[kani::proof]
fn proof_apply_funding_zero_sum() {
    let long: i128 = kani::any();
    let short: i128 = kani::any();
    let rate: i128 = kani::any();
    kani::assume(long.abs() < 2i128.pow(80));
    kani::assume(short.abs() < 2i128.pow(80));
    kani::assume(rate.abs() < 1000);

    let (new_long, new_short) = apply_funding(long, short, rate);
    let total_before = long + short;
    let total_after = new_long + new_short;
    assert_eq!(total_before, total_after);
}
