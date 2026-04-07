//! Kani proofs for arithmetic correctness.
//!
//! Run with: `cargo kani`

#![cfg(kani)]

use percolate::math::safe_math::{mul_div, safe_add, safe_mul};

#[kani::proof]
fn proof_safe_add_no_overflow() {
    let a: u128 = kani::any();
    let b: u128 = kani::any();
    kani::assume(a < u128::MAX / 2);
    kani::assume(b < u128::MAX / 2);
    let result = safe_add(a, b);
    assert!(result.is_some());
}

#[kani::proof]
fn proof_safe_mul_no_overflow() {
    let a: u128 = kani::any();
    let b: u128 = kani::any();
    kani::assume(a < 2u128.pow(64));
    kani::assume(b < 2u128.pow(64));
    let result = safe_mul(a, b);
    assert!(result.is_some());
}

#[kani::proof]
fn proof_mul_div_no_overflow() {
    let a: u128 = kani::any();
    let b: u128 = kani::any();
    let c: u128 = kani::any();
    kani::assume(c > 0);
    kani::assume(a < 2u128.pow(96));
    kani::assume(b < 2u128.pow(96));
    let _ = mul_div(a, b, c);
}
