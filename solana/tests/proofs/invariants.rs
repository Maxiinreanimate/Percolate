//! Kani proofs for protocol invariants.

#![cfg(kani)]

use percolate::engine::risk::*;
use percolate::engine::collateral_haircut::*;

#[kani::proof]
fn proof_effective_position_bounded_by_basis() {
    let basis: i128 = kani::any();
    let current_a: u128 = kani::any();
    let a_snapshot: u128 = kani::any();
    kani::assume(basis.abs() < 2i128.pow(64));
    kani::assume(a_snapshot > 0 && a_snapshot < 2u128.pow(64));
    kani::assume(current_a <= a_snapshot);

    let effective = effective_position(basis, current_a, a_snapshot);
    assert!(effective.unsigned_abs() <= basis.unsigned_abs());
}

#[kani::proof]
fn proof_haircut_collateral_never_exceeds_raw() {
    let raw: u128 = kani::any();
    let static_h: u16 = kani::any();
    let dynamic_h: u16 = kani::any();
    kani::assume(raw < 2u128.pow(80));
    kani::assume(static_h <= 5000);
    kani::assume(dynamic_h <= 5000);

    let effective = effective_collateral_value(raw, static_h, dynamic_h);
    assert!(effective <= raw);
}

#[kani::proof]
fn proof_reduce_a_decreases_or_unchanged() {
    let current_a: u128 = kani::any();
    let oi: u128 = kani::any();
    let removed: u128 = kani::any();
    kani::assume(current_a < 2u128.pow(80));
    kani::assume(oi > 0 && oi < 2u128.pow(64));
    kani::assume(removed <= oi);

    let new_a = reduce_side_a(current_a, oi, removed);
    assert!(new_a <= current_a);
}
