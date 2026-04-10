# Percolate

**The first and only multichain perpetual futures DEX. Any chain. Any token. Fully permissionless.**

Percolate is a permissionless perpetual futures protocol built on [Toly's Percolator risk engine](https://github.com/aeyakovenko/percolator). It runs natively on **Solana** and **EVM** chains, with cross-margin, multi-collateral, and adaptive liquidity from day one.

## Why Percolate

[Perk](https://github.com/kai-builds-ai/perk-protocol-public) was the first production implementation of Percolator. It launched on Solana with isolated margin, single-asset collateral, and static liquidity pools. We took the same risk engine and pushed it further:

- **Cross-margin from day one** — one collateral pool, multiple positions, shared risk
- **Multi-collateral** — USDC, USDT, SOL, ETH, wBTC accepted with dynamic haircuts
- **Adaptive liquidity** — vAMM `k` auto-scales with real-time volume and volatility
- **Multichain native** — same protocol, same risk engine, deployed natively on Solana and EVM

## Architecture

```
percolate/
├── solana/          Anchor program (Rust)
│   ├── programs/    Main on-chain program
│   └── tests/       Kani formal verification proofs
│
├── evm/             Solidity contracts (Foundry)
│   ├── src/         Core contracts and libraries
│   └── test/        Unit and invariant tests
│
├── sdk/             TypeScript SDK (multichain client)
│   └── src/chains/  Solana and EVM adapters
│
├── cranker/         Permissionless keeper bots
│   └── src/         Liquidator, funding, trigger, peg, adaptive-k loops
│
├── docs/            Architecture, risk engine, oracle, proofs
├── audits/          Security audit reports
├── examples/        SDK usage examples
└── scripts/         Deployment and management
```

See [ARCHITECTURE.md](./ARCHITECTURE.md) for the full system design.

## Risk Engine

Percolate uses the H + A/K risk engine from Anatoly Yakovenko's Percolator research.

**H — The Haircut**

```
Residual = max(0, Vault - Capital - Insurance)
H = min(Residual, ProfitTotal) / ProfitTotal
```

If the vault is fully backed, H = 1. If stressed, H < 1. Every profitable account sees the same proportional reduction. Deposited capital is never haircut. Self-healing as the vault recovers.

**A/K — Lazy Side Indices**

```
effective_pos(i) = floor(basis_i * A / a_snapshot_i)
pnl_delta(i)     = floor(|basis_i| * (K - k_snapshot_i) / (a_snapshot_i * POS_SCALE))
```

When a position goes bankrupt, A scales every account on that side equally and K accumulates the deficit. No queues. No forced closures. Settlement is O(1) per account. Three-phase recovery (DrainOnly → ResetPending → Normal) runs autonomously.

See [docs/RISK-ENGINE.md](./docs/RISK-ENGINE.md) for the full specification.

## Cross-Margin

Most permissionless perp protocols launch with isolated margin only. Percolate launches with cross-margin native:

- Single user account holds collateral and references multiple positions across markets
- Shared margin reduces liquidation risk for hedged portfolios
- Per-position risk still tracked individually for the H + A/K engine
- Liquidations target the worst position first, not the whole account

See [docs/CROSS-MARGIN.md](./docs/CROSS-MARGIN.md).

## Multi-Collateral

Stablecoins are not the only collateral that matters. Percolate accepts:

| Asset | Haircut | Notes |
|-------|---------|-------|
| USDC | 0% | Reference asset |
| USDT | 0% | Reference asset |
| PYUSD | 0% | Reference asset |
| SOL | 15% | Volatile, dynamic haircut |
| ETH | 15% | Volatile, dynamic haircut |
| wBTC | 10% | Volatile, lower haircut |

Haircuts adjust dynamically based on 30-day price stability. See [docs/COLLATERAL.md](./docs/COLLATERAL.md).

## Adaptive Liquidity

Perk's `k` is set at market creation and grows with collateral. Percolate's `k` adapts in real time:

- Volume above 24h average → k increases (deeper liquidity)
- Volatility above threshold → k increases (more slippage protection)
- Sustained low volume → k decreases (capital efficiency)

This means markets self-tune. High-traffic markets get deeper, quiet markets stay efficient. See [docs/ADAPTIVE-LIQUIDITY.md](./docs/ADAPTIVE-LIQUIDITY.md).

## Multichain

The same protocol is deployed natively on each chain. No bridges. No wrapped positions. No cross-chain settlement risk.

| Chain | Status | Program/Contract |
|-------|--------|------------------|
| Solana | Live | TBD |
| Ethereum | Live | TBD |
| Base | Live | TBD |
| Arbitrum | Live | TBD |

See [docs/MULTICHAIN.md](./docs/MULTICHAIN.md).

## Stack

- **Solana:** Rust + Anchor
- **EVM:** Solidity + Foundry
- **SDK:** TypeScript with adapters per chain
- **Cranker:** Node.js, runs against all chains in parallel
- **Oracle:** PercOracle (Pyth + Jupiter + DEX aggregation, cranker-maintained)
- **Frontend:** Next.js + TradingView (separate repo)

## Formal Verification

The Solana program ships with Kani formal verification proofs covering:

- Arithmetic correctness (U256/I256, mul_div, rounding)
- Conservation laws (no minting, no value leak)
- Margin invariants (IM/MM, equity, liquidation triggers)
- Funding (k-index deltas, rate clamping, zero-sum)
- Liveness (reset finalization, terminal drain, bankruptcy)
- Instruction composition (deposit/withdraw/open/close lifecycles)

Run with:

```bash
cd solana/programs/percolate
cargo install --locked kani-verifier
cargo kani setup
cargo kani
```

See [docs/PROOF-SPEC.md](./docs/PROOF-SPEC.md).

## SDK Usage

```typescript
import { PercolateClient } from "@percolate/sdk";

// Solana
const solana = new PercolateClient({
  chain: "solana",
  rpc: "https://api.mainnet-beta.solana.com",
  wallet: keypair,
});

// Or EVM
const evm = new PercolateClient({
  chain: "ethereum",
  rpc: "https://eth.llamarpc.com",
  signer: signer,
});

// Same API on every chain
await client.openLong({ market: "SOL-PERP", size: 10, leverage: 5 });
await client.closePosition({ market: "SOL-PERP" });
```

See [sdk/README.md](./sdk/README.md).

## Cranker

```bash
cd cranker
npm install
cp .env.example .env
npm start
```

Runs liquidation, funding, trigger order, AMM peg update, and adaptive-k loops across every configured chain in parallel. Permissionless — anyone can run a cranker for incentives.

## License

Apache 2.0. See [LICENSE](./LICENSE).

Engine inspired by [aeyakovenko/percolator](https://github.com/aeyakovenko/percolator).
