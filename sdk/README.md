# @percolate/sdk

Multichain TypeScript SDK for the Percolate perpetual futures DEX.

The same client API works across Solana and every supported EVM chain. Internal adapters translate to the appropriate RPC calls and transaction format per chain.

## Install

```bash
npm install @percolate/sdk
```

## Usage

```typescript
import { PercolateClient } from "@percolate/sdk";

const client = new PercolateClient({
  chain: "solana",
  rpc: "https://api.mainnet-beta.solana.com",
  wallet: keypair,
});

await client.openUserAccount();
await client.deposit({ collateral: "USDC", amount: 1000 });
await client.openLong({ market: "SOL-PERP", size: 10, leverage: 5 });

const position = await client.getPosition({ market: "SOL-PERP" });
console.log(position);
```

Switch to EVM by changing the chain:

```typescript
const client = new PercolateClient({
  chain: "ethereum",
  rpc: "https://eth.llamarpc.com",
  signer: wallet,
});

// Same API
await client.openLong({ market: "SOL-PERP", size: 10, leverage: 5 });
```

See [../docs](../docs) for the protocol specification.
