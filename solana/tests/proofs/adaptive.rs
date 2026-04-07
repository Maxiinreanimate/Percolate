//! Kani proofs for the adaptive k controller.

#![cfg(kani)]

use percolate::engine::adaptive::*;

#[kani::proof]
fn proof_k_target_within_bounds() {
    let k_base: u128 = kani::any();
    let k_min: u128 = kani::any();
    let k_max: u128 = kani::any();
    let v24: u128 = kani::any();
    let v7d: u128 = kani::any();
    let vol: u32 = kani::any();

    kani::assume(k_min > 0);
    kani::assume(k_max > k_min);
    kani::assume(k_base >= k_min && k_base <= k_max);
    kani::assume(k_base < 2u128.pow(80));
    kani::assume(v24 < 2u128.pow(80));
    kani::assume(v7d < 2u128.pow(80));

    let target = compute_k_target(k_base, k_min, k_max, v24, v7d, vol);
    assert!(target >= k_min);
    assert!(target <= k_max);
}

#[kani::proof]
fn proof_smoothing_does_not_overshoot() {
    let current: u128 = kani::any();
    let target: u128 = kani::any();
    let elapsed: u32 = kani::any();
    kani::assume(current < 2u128.pow(80));
    kani::assume(target < 2u128.pow(80));
    kani::assume(elapsed < 3600);

    let new_k = smooth_toward_target(current, target, elapsed, 3600);

    if target > current {
        assert!(new_k <= target);
        assert!(new_k >= current);
    } else {
        assert!(new_k >= target);
        assert!(new_k <= current);
    }
}
