# Percolate — Architecture

> The first and only multichain perpetual futures DEX. Any token. Any leverage. Fully permissionless.
> Built on Anatoly Yakovenko's Percolator risk engine, extended with cross-margin, multi-collateral, and adaptive liquidity.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Design Principles](#2-design-principles)
3. [Multichain Architecture](#3-multichain-architecture)
4. [Solana Program](#4-solana-program)
5. [EVM Contracts](#5-evm-contracts)
6. [Account Layout](#6-account-layout)
7. [Instructions](#7-instructions)
8. [vAMM Engine](#8-vamm-engine)
9. [Risk Engine (H + A/K)](#9-risk-engine)
10. [Cross-Margin](#10-cross-margin)
11. [Multi-Collateral](#11-multi-collateral)
12. [Adaptive Liquidity](#12-adaptive-liquidity)
13. [Fee System](#13-fee-system)
14. [Oracle Integration](#14-oracle-integration)
15. [Trigger Orders](#15-trigger-orders)
16. [Cranker Bots](#16-cranker-bots)
17. [SDK](#17-sdk)
18. [Safety Rails](#18-safety-rails)
19. [Constants](#19-constants)
20. [Deployment](#20-deployment)

---

## 1. Overview

Percolate is a permissionless perpetual futures protocol that runs natively on multiple chains. Anyone can launch a leveraged trading market for any token. Market creators earn 8% of all trading fees on their market forever.

The protocol is built on the Percolator risk engine — Anatoly Yakovenko's research into autonomous, predictable perpetual futures risk management. We took that engine, ported it to production, and added the features Perk's launch was missing.

### What Percolate Adds Beyond Perk

| Feature | Perk | Percolate |
|---------|------|-----------|
| Margin model | Isolated only | **Cross-margin native** |
| Collateral | Stablecoins (6 dec) | **Multi-asset with dynamic haircuts** |
| Liquidity | Static k, grows with collateral | **Adaptive k, real-time tuning** |
| Chains | Solana | **Solana + EVM** |
| Risk engine | Percolator H/A/K | Percolator H/A/K |
| Permissionless | Yes | Yes |

### Stack Summary

- **Solana:** Rust + Anchor framework
- **EVM:** Solidity 0.8.23 + Foundry
- **Oracle:** PercOracle (Pyth + Jupiter + DEX aggregation)
- **SDK:** TypeScript with chain adapters
- **Cranker:** Node.js multichain runner
- **Frontend:** Next.js + TradingView Advanced Charts

---

## 2. Design Principles

### Permissionless

- No admin approval to create markets on any chain
- No whitelist for tokens
- Anyone can create, anyone can trade, anyone can liquidate
- Markets self-heal via Percolator's H/A/K mechanism

### Decentralized

- All logic on-chain — no off-chain matching, no centralized sequencer
- Crankers are permissionless on every chain
- Protocol admin only controls: global pause (emergency), protocol fee rate, minimum parameters
- Individual market creators have NO admin control after creation

### Sustainable

- Market creators earn 8% of fees → incentivizes market bootstrapping
- Protocol earns 92% of fees → sustainable without a token
- Liquidators earn incentive fees → keeps the system healthy
- Insurance fund per market → absorbs bad debt

### Multichain Native

- Each chain runs its own deployment of the protocol
- No cross-chain bridges, no wrapped assets, no settlement risk
- Same risk engine math on every chain
- Same SDK API across all chains

---

## 3. Multichain Architecture

Percolate is not a cross-chain protocol. It is a multichain protocol. The distinction matters.

**Cross-chain protocols** route messages or assets between chains, introducing bridge risk, settlement latency, and shared state vulnerabilities.

**Multichain protocols** deploy independently on each chain. Each deployment is fully self-contained with its own state, its own risk engine, and its own liquidations.

```
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│     Solana       │  │    Ethereum      │  │      Base        │
│                  │  │                  │  │                  │
│  Percolate       │  │  Percolate       │  │  Percolate       │
│  Program         │  │  Contract        │  │  Contract        │
│                  │  │                  │  │                  │
│  ┌────────────┐  │  │  ┌────────────┐  │  │  ┌────────────┐  │
│  │ Risk H/A/K │  │  │  │ Risk H/A/K │  │  │  │ Risk H/A/K │  │
│  └────────────┘  │  │  └────────────┘  │  │  └────────────┘  │
│                  │  │                  │  │                  │
│  ┌────────────┐  │  │  ┌────────────┐  │  │  ┌────────────┐  │
│  │ vAMM       │  │  │  │ vAMM       │  │  │  │ vAMM       │  │
│  └────────────┘  │  │  └────────────┘  │  │  └────────────┘  │
└──────────────────┘  └──────────────────┘  └──────────────────┘
         ▲                     ▲                     ▲
         │                     │                     │
         └─────────────────────┴─────────────────────┘
                               │
                    ┌──────────┴──────────┐
                    │   Percolate SDK     │
                    │   (TypeScript)      │
                    └─────────────────────┘
                               │
                    ┌──────────┴──────────┐
                    │   Cranker Bots      │
                    │   (multi-chain)     │
                    └─────────────────────┘
```

The SDK abstracts the chain differences. Crankers run loops against every configured chain in parallel. Same API, same logic, different runtime.

### Chain-Specific Considerations

| Chain | Storage Cost | Compute Cost | Block Time | Notes |
|-------|--------------|--------------|------------|-------|
| Solana | Low | Very low | 400ms | Fastest execution, lowest fees |
| Ethereum | Very high | High | 12s | Highest security, slowest |
| Base | Low | Low | 2s | L2, EVM-compatible |
| Arbitrum | Low | Low | ~250ms | L2, EVM-compatible, fastest L2 |

The protocol parameters (funding period, max leverage caps, minimum k) are tuned per chain to account for these differences.

---

## 4. Solana Program

### Program Structure

```
solana/programs/percolate/
├── Cargo.toml
└── src/
    ├── lib.rs                    Anchor entrypoint
    ├── errors.rs
    ├── constants.rs
    ├── state/
    │   ├── mod.rs
    │   ├── protocol.rs           Global config
    │   ├── market.rs             Per-market state
    │   ├── user_account.rs       Cross-margin user account
    │   ├── position.rs           Per-market position (referenced from user_account)
    │   ├── trigger_order.rs      Limit/stop orders
    │   ├── insurance_fund.rs     Per-market insurance
    │   └── collateral.rs         Multi-collateral registry
    ├── instructions/
    │   ├── mod.rs
    │   ├── initialize_protocol.rs
    │   ├── register_collateral.rs   Multi-collateral registration
    │   ├── create_market.rs         Permissionless
    │   ├── open_user_account.rs     Cross-margin account creation
    │   ├── deposit.rs               Multi-collateral deposit
    │   ├── withdraw.rs
    │   ├── open_position.rs
    │   ├── close_position.rs
    │   ├── place_trigger_order.rs
    │   ├── execute_trigger_order.rs
    │   ├── cancel_trigger_order.rs
    │   ├── liquidate.rs             Permissionless
    │   ├── crank_funding.rs         Permissionless
    │   ├── update_amm.rs            Oracle peg
    │   ├── adapt_k.rs               Adaptive liquidity tuning
    │   ├── settle_pnl.rs
    │   ├── admin_pause.rs           Emergency only
    │   └── admin_update_protocol.rs
    ├── engine/
    │   ├── mod.rs
    │   ├── vamm.rs                  Virtual AMM (x*y=k)
    │   ├── risk.rs                  H + A/K from Percolator
    │   ├── funding.rs               Funding rate calculation
    │   ├── margin.rs                Cross-margin calculations
    │   ├── liquidation.rs           Liquidation routing
    │   ├── oracle.rs                Oracle abstraction
    │   ├── warmup.rs                PnL warmup window
    │   ├── adaptive.rs              Adaptive k controller
    │   └── collateral_haircut.rs    Multi-collateral haircuts
    └── math/
        ├── mod.rs
        ├── u256.rs                  256-bit unsigned math
        ├── i256.rs                  256-bit signed math
        └── safe_math.rs             Overflow-checked operations
```

The program is structured around three core systems: state, instructions, and engine. State defines the persisted data. Instructions are the entry points called by users and crankers. The engine contains the pure mathematical logic that computes everything.

This separation matters for formal verification. The engine modules contain only deterministic functions with no Solana runtime dependencies, which means they can be tested with Kani proofs in isolation.

---

## 5. EVM Contracts

### Contract Structure

```
evm/
├── foundry.toml
├── src/
│   ├── Percolate.sol             Main protocol entrypoint
│   ├── Market.sol                Per-market storage and logic
│   ├── Vault.sol                 Multi-collateral vault
│   ├── RiskEngine.sol            H + A/K implementation
│   ├── VAMM.sol                  Virtual AMM
│   ├── Oracle.sol                Oracle aggregator
│   ├── interfaces/
│   │   ├── IPercolate.sol
│   │   ├── IMarket.sol
│   │   └── IOracle.sol
│   └── libraries/
│       ├── MarginMath.sol
│       ├── HaircutMath.sol
│       └── LazyIndices.sol
└── test/
    ├── Percolate.t.sol
    ├── RiskEngine.t.sol
    ├── VAMM.t.sol
    └── Invariants.t.sol           Foundry invariant tests
```

The EVM implementation uses the same risk engine math as the Solana program. The libraries `MarginMath`, `HaircutMath`, and `LazyIndices` are direct ports of the corresponding Rust modules.

Storage is organized differently due to Solidity's account model. Instead of separate PDA accounts per market and per position, the EVM contract uses nested mappings on a single Percolate contract. Each market is identified by `keccak256(token, creator)` and stored in a `mapping(bytes32 => Market)`.

User accounts work the same way: `mapping(address => UserAccount)`. The cross-margin design means a single user account can hold positions across many markets without creating new contract storage per position.

### Why Foundry

Foundry's invariant testing is essential for a protocol like this. We can fuzz millions of instruction sequences against the contract and assert that conservation, OI consistency, and margin invariants hold. This gives the same kind of correctness guarantees that Kani provides for the Solana program.

---

## 6. Account Layout

### Protocol (singleton)

```rust
#[account]
pub struct Protocol {
    pub admin: Pubkey,
    pub paused: bool,
    pub market_count: u64,

    // Fee config
    pub creator_fee_share_bps: u16,    // 800 = 8% to creator
    pub min_trading_fee_bps: u16,      // 3 bps floor
    pub max_trading_fee_bps: u16,      // 100 bps ceiling

    // Cross-margin config
    pub min_account_equity: u64,       // Minimum equity to keep account open

    // Multi-collateral config
    pub collateral_count: u8,          // Number of registered collateral types

    // Adaptive k config
    pub adaptive_k_enabled: bool,
    pub adaptive_k_window_seconds: u32,

    // Stats
    pub total_volume: u128,
    pub total_fees_collected: u128,

    pub bump: u8,
}
```

### Collateral Registry

```rust
#[account]
pub struct CollateralEntry {
    pub mint: Pubkey,
    pub decimals: u8,
    pub haircut_bps: u16,              // Static portion of haircut (e.g. SOL = 1500)
    pub dynamic_haircut_bps: u16,      // Adjusted by 30-day volatility
    pub price_oracle: Pubkey,          // Pyth feed for collateral pricing
    pub enabled: bool,
    pub total_deposited: u64,
    pub bump: u8,
}
```

Each accepted collateral has its own registry entry. The protocol admin can register new collateral types but cannot remove them once accounts hold balances.

### Market

```rust
#[account]
pub struct Market {
    pub market_index: u64,
    pub token_mint: Pubkey,            // Base asset
    pub creator: Pubkey,               // Earns 8% of fees forever
    pub creator_fee_account: Pubkey,

    // vAMM state
    pub base_reserve: u128,
    pub quote_reserve: u128,
    pub k: u128,                       // Adaptive — adjusts in real time
    pub k_target: u128,                // Adaptive controller target
    pub k_last_adjusted: i64,
    pub peg_multiplier: u128,
    pub total_long_position: u128,
    pub total_short_position: u128,

    // Adaptive k state
    pub volume_24h: u128,
    pub volume_avg_7d: u128,
    pub volatility_score: u32,         // 0-10000

    // Market parameters (immutable after creation)
    pub max_leverage: u32,
    pub trading_fee_bps: u16,
    pub liquidation_fee_bps: u16,
    pub maintenance_margin_bps: u16,

    // Oracle
    pub oracle_source: OracleSource,
    pub oracle_address: Pubkey,

    // Risk engine state (Percolator H + A/K)
    pub insurance_fund_balance: u64,
    pub haircut_numerator: u128,
    pub haircut_denominator: u128,
    pub long_a: u128,
    pub long_k_index: i128,
    pub short_a: u128,
    pub short_k_index: i128,
    pub long_epoch: u64,
    pub short_epoch: u64,
    pub long_state: SideState,
    pub short_state: SideState,

    // Funding
    pub last_funding_time: i64,
    pub cumulative_long_funding: i128,
    pub cumulative_short_funding: i128,
    pub funding_period_seconds: u32,
    pub funding_rate_cap_bps: u16,

    // Warmup
    pub warmup_period_slots: u64,

    // Stats
    pub creator_fees_earned: u64,
    pub protocol_fees_earned: u64,
    pub total_volume: u128,

    pub active: bool,
    pub bump: u8,
    pub created_at: i64,
}
```

### UserAccount (Cross-Margin)

```rust
#[account]
pub struct UserAccount {
    pub authority: Pubkey,

    // Multi-collateral balances
    pub collateral_balances: [CollateralBalance; 8],

    // Position references (up to 16 positions across markets)
    pub positions: [PositionRef; 16],
    pub position_count: u8,

    // Cross-margin aggregate state
    pub total_margin_used: u64,
    pub total_unrealized_pnl: i64,
    pub total_realized_pnl: i64,

    // Risk engine per-account state
    pub fee_credits: i64,
    pub last_settled_at: i64,

    pub bump: u8,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub struct CollateralBalance {
    pub collateral_index: u8,
    pub amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub struct PositionRef {
    pub market: Pubkey,
    pub base_size: i64,
    pub quote_entry: u128,
    pub last_funding_index: i128,
    pub a_snapshot: u128,
    pub k_snapshot: i128,
    pub epoch_snapshot: u64,
    pub reserved_pnl: u64,
    pub warmup_started_at: u64,
}
```

This is the key cross-margin structure. A single user account references positions across many markets. Margin is computed by aggregating unrealized PnL across all positions and comparing to the sum of collateral balances (after haircuts).

### TriggerOrder

```rust
#[account]
pub struct TriggerOrder {
    pub authority: Pubkey,
    pub market: Pubkey,
    pub order_id: u64,

    pub order_type: TriggerOrderType,
    pub side: Side,
    pub size: u64,
    pub trigger_price: u64,
    pub leverage: u32,
    pub reduce_only: bool,

    pub created_at: i64,
    pub expiry: i64,

    pub bump: u8,
}
```

---

## 7. Instructions

### Protocol Admin

`initialize_protocol` — singleton, set fee config and minimums
`admin_pause(paused)` — emergency only
`admin_update_protocol(params)` — update fee bounds, adaptive k window
`register_collateral(mint, oracle, haircut_bps)` — add accepted collateral

### Market Creation (Permissionless)

`create_market(params)` — anyone calls. Validates leverage bounds, fee bounds, k minimum. Verifies oracle exists. Creates Market PDA. Sets `creator = signer`. Earns 8% of fees forever.

### User — Account Management

`open_user_account()` — creates the cross-margin user account. Required before any deposits or trades.

`deposit(collateral_index, amount)` — deposit any registered collateral. The account tracks each collateral type separately. Haircuts are applied when computing available margin.

`withdraw(collateral_index, amount)` — checks initial margin held across all positions before allowing withdrawal.

### User — Trading

`open_position(market, side, base_size, leverage, max_slippage_bps)` — settles user (funding, lazy liquidation check). Validates leverage. Computes required margin against the cross-margin account equity. Executes against vAMM. Splits fees 8% to creator, 92% to protocol. Updates position and market state.

`close_position(market)` / `close_position_partial(market, base_size)` — settle user. Reverse trade against vAMM. Apply warmup rules and haircut to profit. Apply trading fee. Update or zero the position. Return freed margin to the account.

### Trigger Orders

`place_trigger_order(params)`, `cancel_trigger_order(order_id)`, `execute_trigger_order(order_id)` — same model as Perk. Limit, stop loss, take profit. Permissionless execution with 0.01% incentive fee.

### Maintenance (Permissionless)

`liquidate(user, market)` — settle target user. If undercollateralized across the cross-margin account, liquidate the worst position first (highest absolute PnL drawdown). 50% of liquidation fee to liquidator, 50% to insurance fund.

`crank_funding(market)` — anyone calls every funding period. Updates cumulative funding indices.

`update_amm(market)` — anyone calls. Adjusts peg_multiplier toward oracle. Small incentive.

`adapt_k(market)` — anyone calls. Reads volume_24h and volatility_score, adjusts k_target. Slowly converges actual k to target over a smoothing window. Small incentive.

`settle_pnl(user, market)` — anyone calls. Settles funding, applies haircut to released profit, advances warmup. Mostly used by other instructions internally but exposed for crankers.

---

## 8. vAMM Engine

The vAMM is a virtual constant-product AMM:

```
base_reserve * quote_reserve = k
mark_price = (quote_reserve * peg_multiplier) / base_reserve
```

Same model as Perk. The difference is that `k` is not static. It is continuously adjusted by the adaptive controller.

### Opening a Long Position

```
Before: base = 1000, quote = 1000, k = 1,000,000, peg = 150

User goes long 10 base:
new_base = 990
new_quote = k / new_base = 1010.10
quote_cost = 10.10
effective_price = 10.10 * 150 / 10 = $151.51 (slippage)
```

### Adaptive k Adjustment

After every trade, the adaptive controller checks:

```
volume_factor = volume_24h / volume_avg_7d
volatility_factor = volatility_score / 5000

k_target = k_base * (1 + volume_factor + volatility_factor) / 3
```

The controller smooths k toward k_target over a configurable window (default 1 hour) to prevent oscillation. This means a market that suddenly sees 10x volume gets deeper liquidity within an hour, and a market that goes quiet returns to baseline depth slowly.

See [docs/ADAPTIVE-LIQUIDITY.md](./docs/ADAPTIVE-LIQUIDITY.md) for the full controller specification.

---

## 9. Risk Engine

The risk engine is the heart of the protocol. It is a direct port of the math published in Anatoly Yakovenko's [Percolator](https://github.com/aeyakovenko/percolator) research repository.

### H — The Haircut

Capital is senior. Profit is junior. A single global ratio determines how much profit is real.

```
Residual = max(0, Vault - Capital - Insurance)
H = min(Residual, ProfitTotal) / ProfitTotal
```

When the vault is fully backed by capital + insurance + actual profit, H equals 1 and all matured profit is withdrawable. When the vault is stressed (loss exceeded insurance), H falls below 1 and every profitable account sees the same proportional reduction.

Critically:
- Deposited capital is **never** haircut. Flat accounts always withdraw 100% of their deposits.
- Only **matured** profit enters the haircut. Fresh profit sits in `reserved_pnl` and matures over the warmup window.
- The math is conservative — `floor` rounding means the sum of all effective PnL never exceeds what exists in the vault.
- Self-healing — as losses settle and the vault recovers, H rises back toward 1.

### A/K — Lazy Side Indices

When a leveraged account goes bankrupt, two things need to happen: the position quantity must be removed from open interest, and any uncovered deficit must be distributed across the opposing side.

Traditional auto-deleveraging (ADL) picks specific accounts and force-closes them. Percolator replaces the ADL queue with two global coefficients per side:

- **A** scales every account's effective position equally on that side
- **K** accumulates all PnL events into one index

```
effective_pos(i) = floor(basis_i * A / a_snapshot_i)
pnl_delta(i)     = floor(|basis_i| * (K - k_snapshot_i) / (a_snapshot_i * POS_SCALE))
```

When a liquidation reduces OI, A decreases — every account on that side shrinks proportionally. When a deficit is socialized, K shifts — every account absorbs the same per-unit loss.

No account is singled out. Settlement is O(1) per account and order-independent.

### Three-Phase Recovery

A/K guarantees forward progress through a deterministic state machine:

1. **DrainOnly** — A drops below a precision threshold. No new OI can be added on that side. Existing positions can only close.
2. **ResetPending** — OI reaches zero. The engine snapshots K, increments the epoch, and resets A back to 1.
3. **Normal** — once all stale accounts settle and OI is confirmed zero, the side reopens.

No admin intervention. No governance vote. The state machine always makes progress.

### Warmup Window

New profit enters `reserved_pnl` (locked). It converts to matured profit linearly over `warmup_period_slots`. This prevents oracle manipulation: an attacker who pumps a price, opens a position, and tries to claim profit will see that profit locked until the warmup window passes.

### Margin Calculations

Standard model:

```
Initial margin    = notional / leverage
Maintenance       = notional * maintenance_margin_bps / 10000
Margin ratio      = (collateral + unrealized_pnl) / notional
Liquidation when  = margin_ratio < maintenance_margin_bps / 10000
```

Cross-margin extension:

```
Total equity      = sum(collateral_i * (1 - haircut_i)) + sum(unrealized_pnl across positions)
Total notional    = sum(|base_size_j| * mark_price_j across positions)
Account margin    = total_equity / total_notional
Account liquidates when account_margin < weighted_maintenance_margin
```

When an account is liquidated, the worst position (highest absolute drawdown) is closed first. If that brings the account back above maintenance, the rest of the positions stay open. Otherwise the next worst position gets liquidated.

---

## 10. Cross-Margin

### Why Cross-Margin

Isolated margin is simple. Each position has its own collateral and its own liquidation price. If one position blows up, only that position's collateral is at risk.

Cross-margin is more capital efficient. A trader who is long SOL and short ETH has a partially hedged portfolio. Their net risk is lower than the sum of their individual positions. Isolated margin treats them as fully independent and requires margin for both. Cross-margin recognizes the hedge and requires less.

Most permissionless perp protocols launch with isolated only because cross-margin is harder to implement correctly. The risk engine has to track aggregate state across multiple positions and route liquidations intelligently.

### Implementation

The `UserAccount` PDA holds:
- All collateral balances (multi-asset)
- References to all positions across all markets
- Aggregate margin state

Every instruction that touches a position also updates the aggregate state. Margin checks happen against the aggregate, not per-position.

Liquidation routing picks the worst position first. The "worst" position is the one with the highest absolute drawdown relative to its margin contribution. This minimizes the impact on the rest of the portfolio.

### Conservation

Cross-margin still respects all the same conservation laws. The risk engine tracks per-position state (basis, snapshots, epoch) and per-account state (collateral, aggregate PnL). The H and A/K mechanisms operate at the per-market level, not per-account, so cross-margin does not change the underlying risk math.

See [docs/CROSS-MARGIN.md](./docs/CROSS-MARGIN.md) for the full specification.

---

## 11. Multi-Collateral

### Accepted Collateral

| Asset | Static Haircut | Dynamic Range | Notes |
|-------|---------------|---------------|-------|
| USDC | 0% | 0% | Reference asset |
| USDT | 0% | 0% | Reference asset |
| PYUSD | 0% | 0% | Reference asset |
| SOL | 15% | 10-25% | Adjusted by 30d volatility |
| ETH | 15% | 10-25% | Adjusted by 30d volatility |
| wBTC | 10% | 5-20% | Adjusted by 30d volatility |

The static haircut is fixed at registration time. The dynamic haircut adjusts based on the 30-day price volatility of the collateral asset. Higher volatility means higher haircut means less effective margin.

### How Haircuts Work

When computing user equity, each collateral balance is multiplied by `(1 - haircut)`:

```
effective_balance = raw_balance * (1 - static_haircut - dynamic_haircut)
```

For USDC at 0% haircut, the effective balance equals the raw balance. For SOL at 25% combined haircut, $100 of SOL collateral counts as $75 of margin.

This protects the protocol from collateral price drops. If SOL drops 15% in an hour, an account that posted SOL as collateral does not become instantly insolvent because there is built-in safety margin.

### Pricing

Each registered collateral has its own oracle for pricing. SOL collateral uses the SOL/USD Pyth feed. ETH uses ETH/USD. The protocol does not assume any collateral is pegged unless it is explicitly registered as a stablecoin.

### Vault Architecture

Each market still has its own vault but the vault now holds multiple token accounts — one per accepted collateral type. Deposits route to the correct token account based on the collateral mint. Withdrawals do the same in reverse.

See [docs/COLLATERAL.md](./docs/COLLATERAL.md) for the full registry and haircut logic.

---

## 12. Adaptive Liquidity

### The Problem

Static k pools work fine for predictable markets. They fail in two scenarios:

**Scenario 1: A new memecoin pumps 10x in an hour.** Volume spikes. Slippage destroys traders because k was set for normal conditions. The market becomes unusable until someone manually updates parameters.

**Scenario 2: A market that was active for a week goes dead.** Liquidity sits idle. Capital is locked in a pool that nobody uses.

### The Solution

Percolate's k adapts continuously. After every trade, the adaptive controller recomputes:

```
volume_factor    = volume_24h / volume_avg_7d
volatility_factor = volatility_score / 5000  // 0-2.0 range
k_target = k_base * (1 + volume_factor + volatility_factor) / 3
```

If volume is 5x its 7-day average and volatility is high, `k_target` becomes much larger. The actual `k` then smooths toward `k_target` over a 1-hour window:

```
delta = (k_target - k) / (3600 / time_since_last_adjust)
k_new = k + delta
```

This prevents oscillation. A single big trade does not immediately blow up k. Sustained activity does.

### Bounds

`k` has hard bounds: `k_min` (minimum liquidity, prevents manipulation) and `k_max` (maximum liquidity, prevents controller runaway). Both are set per market at creation time.

### Cranker Integration

The `adapt_k` instruction can be called by anyone. Crankers run a loop that calls it whenever the time since last adjustment exceeds the configured interval (default 60 seconds). Cranker earns a small fee per call.

See [docs/ADAPTIVE-LIQUIDITY.md](./docs/ADAPTIVE-LIQUIDITY.md) for the controller math and stability proofs.

---

## 13. Fee System

### Fee Flow

```
Trade executes → fee = notional * trading_fee_bps / 10000

Split:
├── 8% → Market Creator (creator_fee_account)
└── 92% → Protocol (protocol_fee_vault)

Liquidation → fee = notional * liquidation_fee_bps / 10000
├── 50% → Liquidator
└── 50% → Market Insurance Fund

Trigger order execution → 0.01% of notional
└── 100% → Executor

Adaptive k cranker → fixed reward per call
└── 100% → Caller
```

### Fee Bounds

- Minimum trading fee: 3 bps (0.03%)
- Maximum trading fee: 100 bps (1%)
- Liquidation fee: 100 bps (1%) — fixed
- Creator picks their fee within the bounds at market creation time

### Revenue at Scale

| Total Volume | Protocol Revenue/Day | Protocol Revenue/Month |
|--------------|---------------------|-----------------------|
| $1M | $552 | $16,560 |
| $10M | $5,520 | $165,600 |
| $100M | $55,200 | $1,656,000 |
| $1B | $552,000 | $16,560,000 |

Assuming average 0.06% blended fee, 92% protocol share.

---

## 14. Oracle Integration

### PercOracle System

PercOracle is a custom oracle built for permissionless multichain markets. The cranker aggregates prices from multiple sources, validates consensus via divergence checks, and writes to on-chain accounts.

**Key properties:**

- Fail-closed: if sources disagree beyond MAX_DIVERGENCE_PCT (5%), oracle freezes
- Minimum 2 price sources required per update
- Anyone can initialize oracles for any token by paying rent
- One oracle per token mint per chain
- Same oracle math on Solana and EVM, different storage layouts

### Sources

- **Pyth** (where available) — primary price feed
- **Jupiter** (Solana) — DEX aggregator quote
- **Birdeye** (Solana) — DEX TWAP
- **Uniswap V3 TWAP** (EVM) — on-chain price
- **Chainlink** (EVM) — when available

The cranker reads from at least 2 sources, computes the median, rejects outliers, and writes to the oracle PDA/contract. If fewer than 2 sources are available, the oracle freezes and trading halts.

### Price Validation

- Cranker computes the median of available sources
- Rejects updates where divergence exceeds 5%
- On-chain validation: staleness check, confidence interval, fallback oracle support

### Price Scaling

- All prices: u64 with 6 decimal places
- Position sizes: u128 with POS_SCALE = 10^6
- Collateral: each registered collateral keeps its own decimal count

See [docs/PERC-ORACLE-SPEC.md](./docs/PERC-ORACLE-SPEC.md) for the full specification.

---

## 15. Trigger Orders

Same model as Perk. Limit, stop loss, and take profit orders that execute when the oracle price crosses a threshold.

### Order Types

- **Limit (Open)** — open a position when price reaches the trigger
- **Stop Loss (Close)** — close an existing position when price moves against you
- **Take Profit (Close)** — close an existing position when price hits target

### Execution

The `execute_trigger_order` instruction is permissionless. Crankers watch the chain for orders where the trigger condition is met and call this instruction to execute them. The cranker earns 0.01% of notional as an execution fee.

### Limits

- Max 16 trigger orders per user account (cross-margin total, not per market)
- GTC or expiry timestamp
- Cancel anytime

---

## 16. Cranker Bots

### Architecture

A single Node.js process runs 5 async loops per chain. Configured chains run in parallel.

### Loop 1: Funding Rate Cranker

Every 60 seconds, for each market on each chain, check if funding period elapsed and call `crank_funding`. Earns nothing directly but keeps markets healthy.

### Loop 2: Liquidation Bot

Every 2 seconds, for each market on each chain, fetch all UserAccounts with open positions. For each account below maintenance margin, call `liquidate`. Earns 50% of liquidation fee.

### Loop 3: Trigger Order Executor

Every 1 second, for each market on each chain, get current oracle price and fetch all TriggerOrders. For each order where the trigger condition is met, call `execute_trigger_order`. Earns 0.01% of notional.

### Loop 4: AMM Peg Updater

Every 10 seconds, for each market on each chain, check `|mark_price - oracle_price| / oracle_price > 0.5%`. If true, call `update_amm` to re-peg. Earns small incentive.

### Loop 5: Adaptive K Controller

Every 60 seconds, for each market on each chain, call `adapt_k`. Reads volume and volatility and updates the k controller. Earns small incentive.

### Multichain Execution

The cranker runs all 5 loops against every configured chain in parallel. A single cranker process can monitor Solana, Ethereum, Base, and Arbitrum simultaneously. Each chain has its own RPC connection, signer, and rate limiting.

### Why Anyone Can Run These

All cranker instructions are permissionless on every chain. We run the first set but anyone can compete for the incentives. More crankers means faster execution and healthier markets.

See [cranker/README.md](./cranker/README.md).

---

## 17. SDK

### Package: `@percolate/sdk`

```typescript
import { PercolateClient } from "@percolate/sdk";

const client = new PercolateClient({
  chain: "solana",  // or "ethereum", "base", "arbitrum"
  rpc: rpcUrl,
  wallet: wallet,
});

// User Account
await client.openUserAccount();
await client.deposit({ collateral: "USDC", amount: 1000 });
await client.deposit({ collateral: "SOL", amount: 5 });
await client.withdraw({ collateral: "USDC", amount: 200 });

// Markets
const markets = await client.getMarkets();
await client.createMarket({
  token: "BONK",
  oracle: "BONK_USD",
  maxLeverage: 10,
  tradingFeeBps: 10,
});

// Trading
await client.openLong({ market: "SOL-PERP", size: 10, leverage: 5 });
await client.openShort({ market: "ETH-PERP", size: 0.5, leverage: 3 });
await client.closePosition({ market: "SOL-PERP" });
await client.closePositionPartial({ market: "ETH-PERP", size: 0.2 });

// Trigger Orders
await client.placeLimitOrder({ market: "SOL-PERP", side: "long", triggerPrice: 140, size: 10, leverage: 5 });
await client.placeStopLoss({ market: "SOL-PERP", triggerPrice: 130 });
await client.placeTakeProfit({ market: "SOL-PERP", triggerPrice: 170 });
await client.cancelTriggerOrder({ orderId });

// Read
const account = await client.getUserAccount();
const position = await client.getPosition({ market: "SOL-PERP" });
const markPrice = await client.getMarkPrice({ market: "SOL-PERP" });
const fundingRate = await client.getFundingRate({ market: "SOL-PERP" });

// Cranker (permissionless)
await client.liquidate({ user: address, market: "SOL-PERP" });
await client.crankFunding({ market: "SOL-PERP" });
await client.executeTriggerOrder({ orderId });
await client.updateAmm({ market: "SOL-PERP" });
await client.adaptK({ market: "SOL-PERP" });
```

The same API works on every chain. The SDK has internal adapters that translate to the appropriate RPC calls and transaction format for each chain.

See [sdk/README.md](./sdk/README.md).

---

## 18. Safety Rails

### Global

| Safeguard | Detail |
|-----------|--------|
| Global pause | Admin can freeze all markets (emergency only) |
| Withdrawals always work | Even when paused, users can close + withdraw |
| Fee bounds | Creators cannot set predatory fees (3-100 bps) |
| Min liquidity | Markets must have minimum k to prevent manipulation |
| Min account equity | Cross-margin accounts must hold minimum equity |

### Per-Market (Immutable after creation)

| Safeguard | Detail |
|-----------|--------|
| Max leverage | Creator sets 1x-20x, locked after creation |
| Maintenance margin | 5% default |
| Warmup window | PnL matures over time, blocks oracle manipulation |
| Insurance fund | Funded by 50% of liquidation fees |
| Three-phase recovery | Markets self-heal from cascading liquidations |
| Adaptive k bounds | k_min and k_max prevent controller runaway |

### Economic

| Safeguard | Detail |
|-----------|--------|
| Haircut (H) | Profit scaled down when vault is stressed |
| A/K socialization | Deficit spread equally across surviving positions |
| Funding rate cap | ±0.1% per period |
| Oracle staleness | Rejects stale prices, halts trading if oracle dies |
| Multi-collateral haircuts | Volatile collateral has built-in safety margin |
| Cross-margin liquidation routing | Worst position closed first, minimizes portfolio impact |

---

## 19. Constants

```rust
// Protocol
pub const CREATOR_FEE_SHARE_BPS: u16 = 800;       // 8% to creator
pub const MIN_TRADING_FEE_BPS: u16 = 3;
pub const MAX_TRADING_FEE_BPS: u16 = 100;
pub const LIQUIDATION_FEE_BPS: u16 = 100;
pub const LIQUIDATOR_SHARE_BPS: u16 = 5000;
pub const TRIGGER_EXECUTION_FEE_BPS: u16 = 1;
pub const ADAPTIVE_K_REWARD: u64 = 1_000_000;     // 1 USDC equivalent

// Cross-margin
pub const MAX_POSITIONS_PER_ACCOUNT: usize = 16;
pub const MAX_COLLATERAL_TYPES: usize = 8;
pub const MIN_ACCOUNT_EQUITY: u64 = 10_000_000;   // 10 USDC

// Market defaults
pub const DEFAULT_MAX_LEVERAGE: u32 = 2000;        // 20x
pub const MAINTENANCE_MARGIN_BPS: u16 = 500;       // 5%
pub const DEFAULT_FUNDING_PERIOD: u32 = 3600;      // 1 hour
pub const FUNDING_RATE_CAP_BPS: u16 = 10;
pub const WARMUP_PERIOD_SLOTS: u64 = 1000;
pub const MAX_TRIGGER_ORDERS_PER_USER: u8 = 16;

// Adaptive k
pub const ADAPTIVE_K_WINDOW_SECONDS: u32 = 3600;
pub const ADAPTIVE_K_MIN_INTERVAL: u32 = 60;
pub const VOLUME_AVG_DECAY_BPS: u16 = 100;         // 1% per cycle

// Oracle
pub const ORACLE_STALENESS_SECONDS: u32 = 30;
pub const ORACLE_CONFIDENCE_BPS: u16 = 200;
pub const AMM_PEG_THRESHOLD_BPS: u16 = 50;
pub const MAX_ORACLE_DIVERGENCE_BPS: u16 = 500;

// Precision
pub const PRICE_SCALE: u64 = 1_000_000;
pub const K_SCALE: u128 = 1_000_000_000_000;
pub const PEG_SCALE: u128 = 1_000_000;
pub const HAIRCUT_SCALE: u16 = 10_000;
```

---

## 20. Deployment

### Solana

```bash
cd solana
anchor build
anchor deploy --provider.cluster mainnet-beta
```

### EVM

```bash
cd evm
forge build
forge script script/Deploy.s.sol --rpc-url $RPC_URL --broadcast --verify
```

### Cranker

```bash
cd cranker
npm install
cp .env.example .env
# Configure RPC URLs and signer for each chain
npm start
```

### Frontend

The frontend is a separate repository that consumes `@percolate/sdk`.

---

## Summary

Percolate takes the cleanest perp risk math (Anatoly's Percolator) and extends it with the features that production protocols actually need: cross-margin, multi-collateral, adaptive liquidity, and multichain native deployment. Same engine. More features. More chains.

We are not cross-chain. We are multichain. Every chain runs its own self-contained deployment with its own state. The SDK and crankers tie everything together at the application layer, not the protocol layer.

Everything is open source. Everything is verifiable. The Percolator engine was published as open research and we are extending that research into production.
