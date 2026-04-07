# Risk Engine

> Direct port of [Anatoly Yakovenko's Percolator research](https://github.com/aeyakovenko/percolator), extended for cross-margin and multi-collateral.

The Percolate risk engine solves two independent problems with two clean mechanisms.

## Problem 1: Exit Fairness

When the vault is stressed, who gets paid and how much? Most exchanges rely on insurance funds and ad-hoc decisions. Percolator uses a single global ratio.

### H — The Haircut

```
Residual = max(0, Vault - Capital - Insurance)
H = min(Residual, MaturedProfitTotal) / MaturedProfitTotal
```

- If H = 1: vault is fully backed, all profit is real
- If H < 1: vault is stressed, profit is proportionally reduced
- Capital is **never** haircut. Flat accounts always withdraw their deposits in full.
- Self-healing: as losses settle, H rises back to 1.

### Application

When a user releases profit (closes a position or withdraws), the released amount is multiplied by H:

```
effective_pnl = floor(released_pnl * H_num / H_denom)
```

The floor rounding is conservative — the sum of all effective PnL across the protocol can never exceed what is actually in the vault.

### Warmup Window

Fresh profit does not enter the haircut denominator immediately. It sits in `reserved_pnl` and matures linearly over `warmup_period_slots`. This is the core defense against oracle manipulation:

1. Attacker pumps oracle price
2. Opens a leveraged position
3. Tries to claim profit

The profit is locked in `reserved_pnl` until the warmup window passes. By that time, the oracle has reverted and the attacker has nothing to claim.

## Problem 2: Overhang Clearing

When a leveraged account goes bankrupt, the position quantity needs to leave open interest and any uncovered deficit needs to be distributed across the surviving accounts on the same side.

Traditional auto-deleveraging picks specific accounts and force-closes them. This creates queue priority gaming, unfair outcomes, and complex matching logic.

### A/K — The Lazy Indices

Two global coefficients per side replace the ADL queue:

- **A** scales every account's effective position equally
- **K** accumulates all PnL events into one index

```
effective_pos(i) = floor(basis_i * A / a_snapshot_i)
pnl_delta(i)     = floor(|basis_i| * (K - k_snapshot_i) / (a_snapshot_i * POS_SCALE))
```

When a liquidation removes OI:

```
new_A = A * (oi - removed) / oi
```

When a deficit gets socialized:

```
new_K = K - (deficit / oi)
```

Every account on that side absorbs the same per-unit impact. No queue. No selection. Settlement is O(1) per account and order-independent.

## Three-Phase Recovery

A/K guarantees forward progress through a deterministic state machine.

**State: Normal**
- Side accepts new OI
- Liquidations adjust A and K
- Settlement happens lazily on each account interaction

**State: DrainOnly** (entered when A drops below precision threshold)
- Side does NOT accept new OI
- Existing positions can only close
- Liquidations continue until OI hits zero

**State: ResetPending** (entered when OI hits zero)
- Snapshot K
- Increment epoch
- Reset A back to 1
- Wait for stale accounts to settle one final time

**Back to Normal**
- Once all stale accounts have settled, side reopens

This entire cycle runs without admin intervention. No governance vote, no manual reset, no human in the loop.

## Cross-Margin Considerations

In Percolate's cross-margin model, a single user account holds positions across many markets. The H + A/K math operates per-market, not per-account, so cross-margin does not change the underlying invariants. Each position has its own basis, snapshot, and epoch tracked in the user account.

When liquidation happens, the cross-margin liquidation router finds the worst position (highest absolute drawdown) and closes that one first. This minimizes the impact on the rest of the portfolio. The H + A/K updates that follow apply only to the market being liquidated.

## Conservation

The whole system is built around one invariant:

> No user can ever withdraw more value than actually exists on the exchange balance sheet.

H ensures this for profit extraction. A/K ensures this for position scaling. The combination of floor rounding and conservative arithmetic guarantees that the sum of all withdrawable claims is always less than or equal to the vault balance.
