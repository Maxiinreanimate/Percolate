/**
 * Example: Deposit collateral and open a leveraged position.
 *
 * Cross-margin enabled by default — collateral can come from any
 * registered asset (USDC, USDT, SOL, ETH, wBTC).
 */
import { PercolateClient } from "@percolate/sdk";
import { Keypair } from "@solana/web3.js";

async function main() {
  const wallet = Keypair.fromSecretKey(new Uint8Array(64));

  const client = new PercolateClient({
    chain: "solana",
    rpc: "https://api.mainnet-beta.solana.com",
    wallet,
  });

  // 1. Open the cross-margin user account (once per user)
  await client.openUserAccount();

  // 2. Deposit multiple collateral types
  await client.deposit({ collateral: "USDC", amount: 1000 });
  await client.deposit({ collateral: "SOL", amount: 5 });

  // 3. Open a leveraged long
  await client.openLong({
    market: "SOL-PERP",
    size: 10,         // 10 SOL
    leverage: 5,      // 5x
    maxSlippageBps: 50, // 0.5%
  });

  // 4. Check the position
  const position = await client.getPosition({ market: "SOL-PERP" });
  console.log("Position:", position);

  // 5. Set a stop loss
  await client.placeStopLoss({
    market: "SOL-PERP",
    triggerPrice: 130,
  });

  // 6. Set a take profit
  await client.placeTakeProfit({
    market: "SOL-PERP",
    triggerPrice: 180,
  });
}

main().catch(console.error);
