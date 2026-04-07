# Cross-Margin

Most permissionless perp protocols launch with isolated margin. Percolate launches with cross-margin from day one.

## Why Cross-Margin Matters

A trader who is long SOL and short ETH has a partially hedged portfolio. Their net risk is lower than the sum of their individual positions. Isolated margin treats them as fully independent and requires margin for both. Cross-margin recognizes the hedge and requires less capital.

This is more capital efficient. It is also more dangerous if implemented incorrectly because the risk engine has to track aggregate state across multiple positions.

## Implementation

### UserAccount Structure

```rust
pub struct UserAccount {
    pub authority: Pubkey,
    pub collateral_balances: [CollateralBalance; 8],  // multi-asset
    pub positions: [PositionRef; 16],                  // up to 16 positions
    pub position_count: u8,
    pub total_margin_used: u64,
    pub total_unrealized_pnl: i64,
    pub total_realized_pnl: i64,
    pub fee_credits: i64,
    pub last_settled_at: i64,
    pub bump: u8,
    pub created_at: i64,
}
```

A single PDA per user holds:
- All collateral across registered asset types
- References to every position the user has open

### Margin Calculation

Account equity:

```
equity = sum(collateral_i * (1 - haircut_i))
       + sum(unrealized_pnl across all positions)
```

Account total notional:

```
notional = sum(|base_size_j| * mark_price_j across all positions)
```

Account margin ratio:

```
margin_ratio = equity / notional
```

The account is healthy if `margin_ratio >= maintenance_margin / leverage_weighted_average`.

### Liquidation Routing

When an account drops below maintenance, the liquidator does not close everything. The cross-margin router picks the worst position first.

```
fn find_worst_position(positions, mark_prices) -> Option<usize> {
    let mut worst_idx = None;
    let mut worst_drawdown = 0;

    for (i, (pos, mark)) in positions.iter().zip(marks.iter()).enumerate() {
        let pnl = position_unrealized_pnl(pos, mark);
        if pnl < worst_drawdown {
            worst_drawdown = pnl;
            worst_idx = Some(i);
        }
    }
    worst_idx
}
```

The worst position is closed via vAMM. If the account is back above maintenance after that, the rest of the portfolio is preserved. Otherwise, the next worst position gets liquidated.

This is the major win of cross-margin over isolated. Hedged portfolios lose only the worst leg, not everything.

### Conservation

Cross-margin does not change the per-market H + A/K math. Each position has its own basis, a_snapshot, k_snapshot, and epoch tracked in the user account. The risk engine operates on these per-position values just like isolated margin.

The aggregate fields in the user account (`total_margin_used`, `total_unrealized_pnl`) are caches. They are recomputed during settlement and never used as the source of truth for individual position math.

## Limits

- Max 16 positions per user account
- Max 8 collateral types per user account
- Min account equity (to keep account open): 10 USDC equivalent

## Migration Path

The cross-margin design is forward-compatible. A future protocol upgrade could increase the position limit or add multi-account support without breaking existing user accounts.
