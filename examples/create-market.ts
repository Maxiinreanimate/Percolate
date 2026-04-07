/**
 * Example: Create a new permissionless perp market.
 *
 * Anyone can run this to launch a leveraged trading market for any
 * SPL token. The creator earns 8% of all trading fees forever.
 */
import { PercolateClient } from "@percolate/sdk";
import { Keypair } from "@solana/web3.js";

async function main() {
  const wallet = Keypair.fromSecretKey(/* your keypair */ new Uint8Array(64));

  const client = new PercolateClient({
    chain: "solana",
    rpc: "https://api.mainnet-beta.solana.com",
    wallet,
  });

  const { tx, market } = await client.createMarket({
    token: "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263", // BONK mint
    oracle: "perc_oracle_pda_for_bonk",
    maxLeverage: 1000,        // 10x
    tradingFeeBps: 10,        // 0.1%
    initialK: 1_000_000_000n,
    kMin: 100_000_000n,
    kMax: 10_000_000_000n,
  });

  console.log("Market created:", market);
  console.log("Transaction:", tx);
}

main().catch(console.error);
