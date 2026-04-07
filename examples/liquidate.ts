/**
 * Example: Run the liquidation cranker manually.
 *
 * Permissionless — anyone can call liquidate(). Liquidator earns 50%
 * of the liquidation fee.
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

  // Find an undercollateralized account on a market
  const targetUser = "FakeUserPubkey1111111111111111111111111111";
  const market = "SOL-PERP";

  const tx = await client.liquidate({ user: targetUser, market });
  console.log("Liquidation tx:", tx);
}

main().catch(console.error);
