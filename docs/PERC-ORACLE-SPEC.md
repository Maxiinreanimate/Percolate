# PercOracle Specification

PercOracle is the custom oracle system used by Percolate for permissionless markets. It aggregates prices from multiple sources and writes them to on-chain accounts via permissionless cranker calls.

## Goals

- Permissionless: anyone can initialize an oracle for any token
- Multichain: same oracle math on Solana and EVM
- Fail-closed: if sources diverge or stale, trading halts
- Resistant to single-source manipulation

## Architecture

### Sources

- **Pyth** (where available)
- **Jupiter** (Solana)
- **Birdeye** (Solana)
- **Uniswap V3 TWAP** (EVM)
- **Chainlink** (EVM, where available)

The cranker reads from at least 2 sources, computes the median, validates the spread, and writes to the on-chain oracle account.

### On-Chain Storage

```rust
#[account]
pub struct PercOracleAccount {
    pub token_mint: Pubkey,
    pub price: u64,          // 6 decimal precision
    pub confidence: u64,
    pub last_updated: i64,
    pub num_sources: u8,
    pub source_divergence_bps: u16,
    pub authorized_cranker: Pubkey,
    pub bump: u8,
}
```

### Validation

On every read, the consumer must check:

```rust
fn validate_price(oracle: &PercOracleAccount, now: i64) -> Result<()> {
    require!(
        now - oracle.last_updated <= ORACLE_STALENESS_SECONDS as i64,
        OracleStale
    );
    require!(oracle.price > 0, OracleStale);
    require!(
        oracle.source_divergence_bps <= MAX_ORACLE_DIVERGENCE_BPS,
        OracleDivergence
    );
    require!(oracle.num_sources >= 2, OracleStale);
    Ok(())
}
```

If any check fails, the consumer reverts. Trading on that market halts until the oracle recovers.

## Cranker Workflow

```
1. cranker reads Pyth.SOL/USD     → 150.32
2. cranker reads Jupiter.SOL/USDC → 150.28
3. cranker reads Birdeye.SOL/USDC → 150.35
4. divergence = (150.35 - 150.28) / 150.32 = 0.0466% → OK
5. median = 150.32
6. cranker submits update_oracle(token=SOL, price=150.32, sources=3, divergence=46)
```

If a source returns an outlier (>5% divergence from the others), it gets rejected:

```
1. cranker reads Pyth.SOL/USD     → 150.32
2. cranker reads Jupiter.SOL/USDC → 150.28
3. cranker reads Birdeye.SOL/USDC → 165.00  ← outlier
4. discard Birdeye
5. divergence = 0.0266% → OK
6. median(Pyth, Jupiter) = 150.30
7. cranker submits update_oracle(price=150.30, sources=2, divergence=26)
```

If only one source remains, the cranker does not submit an update. The oracle becomes stale and trading halts.

## Initialization

```rust
fn init_perc_oracle(token_mint: Pubkey, authorized_cranker: Pubkey) -> Result<()>
```

Anyone can call this. The PDA is derived from `[b"perc_oracle", token_mint]`. The caller pays the rent. The `authorized_cranker` becomes the address allowed to push price updates.

A future version will support multi-cranker authorization for redundancy.

## EVM Implementation

The EVM version uses the same logic but stores everything in mappings on a single `PercOracle` contract:

```solidity
mapping(address => Price) private cachedPrices;

function pushPrice(address token, uint256 value, uint256 confidence) external onlyCranker {
    cachedPrices[token] = Price({
        value: value,
        confidence: confidence,
        lastUpdated: uint64(block.timestamp)
    });
}
```

Same staleness check, same divergence rejection, same fail-closed behavior.
