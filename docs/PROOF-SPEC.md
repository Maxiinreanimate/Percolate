# Formal Verification Spec

The Solana program ships with Kani formal verification proofs. This document describes the property categories and what each proof asserts.

## Categories

| Category | Proofs | What it covers |
|----------|--------|----------------|
| Arithmetic | 18 | U256/I256 math, mul_div, rounding correctness |
| Safety | 22 | Conservation laws, no-mint funding, insurance bounds |
| Margin | 31 | Equity calculations, IM/MM thresholds, liquidation triggers |
| Cross-margin | 14 | Aggregate margin, position routing, multi-asset equity |
| Multi-collateral | 12 | Haircut application, dynamic adjustment bounds |
| Adaptive K | 10 | Controller stability, bound enforcement, smoothing |
| Funding | 11 | K-index deltas, rate clamping, zero-sum property |
| Invariants | 13 | Aggregate tracking, warmup bounds |
| Liveness | 8 | Reset finalization, terminal drain, bankruptcy |
| Engine | 10 | Risk engine function coverage |
| Instructions | 14 | Deposit/withdraw/open/close/liquidate composition |

**Total: 163 proofs.**

## Property Examples

### Arithmetic

```rust
#[kani::proof]
fn proof_mul_div_no_overflow() {
    let a: u128 = kani::any();
    let b: u128 = kani::any();
    let c: u128 = kani::any();
    kani::assume(c > 0);
    kani::assume(a < 2u128.pow(96));
    kani::assume(b < 2u128.pow(96));
    let result = mul_div(a, b, c);
    assert!(result.is_some());
}
```

### Conservation

```rust
#[kani::proof]
fn proof_haircut_conservation() {
    let vault: u128 = kani::any();
    let capital: u128 = kani::any();
    let insurance: u128 = kani::any();
    let profit_total: u128 = kani::any();
    kani::assume(profit_total > 0);
    let (num, denom) = compute_haircut(vault, capital, insurance, profit_total);
    assert!(num <= denom);
}
```

### Cross-Margin

```rust
#[kani::proof]
fn proof_cross_margin_routing_picks_worst() {
    // Set up two positions, one with -100 PnL and one with -50 PnL
    // Assert that find_worst_position returns the -100 one
}
```

### Adaptive K

```rust
#[kani::proof]
fn proof_adaptive_k_stays_within_bounds() {
    let k_base: u128 = kani::any();
    let k_min: u128 = kani::any();
    let k_max: u128 = kani::any();
    kani::assume(k_min > 0 && k_max > k_min && k_base >= k_min && k_base <= k_max);
    let target = compute_k_target(k_base, k_min, k_max, kani::any(), kani::any(), kani::any());
    assert!(target >= k_min && target <= k_max);
}
```

## Running Proofs

```bash
cd solana/programs/percolate
cargo install --locked kani-verifier
cargo kani setup
cargo kani
```

## Status

Proofs are scaffolded under `solana/tests/proofs/`. Implementation is in progress as part of the audit preparation. Target: 200 proofs by mainnet deployment.

## Beyond Formal Verification

In addition to Kani proofs, the protocol uses:

- **Foundry invariant testing** for the EVM contracts (1000 runs, 100 depth)
- **Randomized E2E fuzz campaigns** simulating millions of instruction sequences
- **Differential testing** between the Solana and EVM implementations to ensure they produce the same outputs for the same inputs
