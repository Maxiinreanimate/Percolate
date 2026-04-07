# Multi-Collateral

Percolate accepts multiple collateral assets, not just stablecoins. Volatile assets are allowed with built-in safety margin via dynamic haircuts.

## Accepted Assets

| Asset | Decimals | Static Haircut | Dynamic Range | Notes |
|-------|----------|---------------|---------------|-------|
| USDC | 6 | 0% | 0% | Reference asset |
| USDT | 6 | 0% | 0% | Reference asset |
| PYUSD | 6 | 0% | 0% | Reference asset |
| SOL | 9 | 15% | ±10% | Volatile, dynamic adjusted |
| ETH | 18 | 15% | ±10% | Volatile, dynamic adjusted |
| wBTC | 8 | 10% | ±10% | Volatile, lower base haircut |

## Static Haircut

The static haircut is set at registration time and never changes. It accounts for the baseline volatility risk of holding the asset as collateral.

USDC has 0% because $1 of USDC is always close to $1. SOL has 15% because the price can swing 15% in a normal day. wBTC has a slightly lower static haircut because BTC is historically less volatile than SOL or ETH.

## Dynamic Haircut

The dynamic haircut adjusts based on the trailing 30-day price volatility:

```
volatility_score = std_dev_30d / mean_30d * 10000  // 0-10000
dynamic_haircut = (volatility_score * MAX_DYNAMIC_HAIRCUT_BPS) / 10000
```

A SOL collateral entry with `volatility_score = 5000` (moderate volatility) gets:
- Static: 1500 bps (15%)
- Dynamic: (5000 * 1000) / 10000 = 500 bps (5%)
- Total: 2000 bps (20%)

If SOL enters a high volatility regime, the cranker observes the new volatility and updates the dynamic haircut on the next call.

## Effective Value

When computing user equity, each collateral balance is multiplied by `(1 - total_haircut)`:

```rust
fn effective_collateral_value(
    raw_value: u128,
    static_haircut_bps: u16,
    dynamic_haircut_bps: u16,
) -> u128 {
    let total = (static_haircut_bps + dynamic_haircut_bps) as u128;
    let multiplier = HAIRCUT_SCALE as u128 - total;
    raw_value * multiplier / HAIRCUT_SCALE as u128
}
```

For $100 of SOL with 20% total haircut, the effective value is $80. The trader can use $80 as margin. The other $20 is the protocol's safety buffer.

## Pricing

Each registered collateral has its own oracle. The protocol does not assume any collateral is pegged unless it is explicitly registered as a stablecoin (which still uses an oracle, just with a tight confidence band).

For SOL collateral the oracle is the SOL/USD Pyth feed. For ETH it is ETH/USD. For wBTC it is BTC/USD with the wBTC/BTC peg trusted at 1:1.

## Vault Architecture

Each market has its own vault but the vault holds multiple token accounts, one per accepted collateral. Deposits route to the correct token account based on the collateral mint. Withdrawals do the same in reverse.

On the EVM side, the `Vault` contract maps `(user, token) => balance` for every registered collateral. The same logical model in a different storage layout.

## Stability

If a user deposits SOL as collateral and SOL drops 30% in an hour, the user's effective margin drops by 30% as well. With a 20% haircut, they have a 20% buffer before becoming insolvent. If the drop exceeds the buffer, the cross-margin liquidation router kicks in and closes positions.

The dynamic haircut updates do not happen instantly with every price tick. They update during `adapt_collateral_haircut` cranker calls (similar cadence to `adapt_k`). This means there is a small lag between volatility changes and haircut updates, which is part of the safety margin design.
