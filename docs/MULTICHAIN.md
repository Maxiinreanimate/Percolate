# Multichain

Percolate is a multichain protocol, not a cross-chain protocol. The distinction matters.

## Multichain vs Cross-Chain

**Cross-chain protocols** route assets or messages between chains. Bridge contracts hold collateral on one chain and mint receipts on another. Settlement requires waiting for finality on both chains. Bridges are the largest source of exploits in DeFi history.

**Multichain protocols** deploy independently on each chain. Each deployment is fully self-contained with its own state, its own risk engine, and its own liquidations. There is no shared state, no bridge, and no cross-chain settlement.

## Architecture

```
Solana                 Ethereum               Base
─────────              ────────               ────
Anchor program         Solidity contract      Solidity contract
PDA accounts           Mappings               Mappings
Solana cranker         EVM cranker            EVM cranker
PercOracle (Solana)    PercOracle (EVM)       PercOracle (EVM)
```

Each chain runs the full protocol. State is independent. A user trading on Solana has no relationship to a user trading on Ethereum, even if they are the same person.

## SDK Abstraction

The user-facing abstraction lives in the SDK, not the protocol. `PercolateClient` accepts a `chain` parameter and routes calls to the appropriate adapter:

```typescript
const solana = new PercolateClient({ chain: "solana", ... });
const ethereum = new PercolateClient({ chain: "ethereum", ... });

// Same API, different runtime
await solana.openLong({ market: "SOL-PERP", size: 10, leverage: 5 });
await ethereum.openLong({ market: "ETH-PERP", size: 1, leverage: 5 });
```

The SDK serializes the call into the appropriate transaction format for each chain. Solana gets an Anchor instruction, EVM gets a contract call. Same input, different output.

## Cranker Multichain Execution

The cranker runs all five loops against every configured chain in parallel. A single Node.js process can crank Solana, Ethereum, Base, and Arbitrum simultaneously:

```typescript
const chains = [
  { name: "solana", rpc: process.env.SOLANA_RPC },
  { name: "ethereum", rpc: process.env.ETH_RPC },
  { name: "base", rpc: process.env.BASE_RPC },
  { name: "arbitrum", rpc: process.env.ARB_RPC },
];

for (const chain of chains) {
  new LiquidatorLoop(chain).start();
  new FundingLoop(chain).start();
  new TriggerExecutorLoop(chain).start();
  new PegUpdaterLoop(chain).start();
  new AdaptiveKLoop(chain).start();
}
```

Each chain has its own RPC connection, its own signer, its own rate limiting. A failure on one chain does not affect the others.

## Per-Chain Tuning

Different chains have different characteristics. Percolate parameters are tuned per chain:

| Chain | Block Time | Funding Period | Max Leverage |
|-------|------------|----------------|--------------|
| Solana | 400ms | 1 hour | 20x |
| Ethereum | 12s | 8 hours | 10x |
| Base | 2s | 1 hour | 20x |
| Arbitrum | 250ms | 1 hour | 20x |

Ethereum has longer funding periods and lower max leverage because the slower block time means liquidations take longer to execute, increasing risk.

## Why No Bridges

Bridges concentrate risk. A single bridge exploit can drain all the value flowing through it. Multichain protocols avoid this by never holding cross-chain positions.

If a user wants to move from Solana to Ethereum, they close their Solana position, withdraw their collateral, bridge the collateral with a third-party bridge of their choice (Wormhole, LayerZero, deBridge), and open a new position on Ethereum. Percolate is not in the bridge business.

This means traders pay the bridge cost themselves, but they get to choose which bridge to trust. It also means Percolate has no cross-chain attack surface.
