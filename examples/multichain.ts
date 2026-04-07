/**
 * Example: Same trades on Solana and Ethereum.
 *
 * The PercolateClient API is identical across chains. Only the
 * config differs.
 */
import { PercolateClient } from "@percolate/sdk";

async function main() {
  const solana = new PercolateClient({
    chain: "solana",
    rpc: "https://api.mainnet-beta.solana.com",
    wallet: /* keypair */ null,
  });

  const ethereum = new PercolateClient({
    chain: "ethereum",
    rpc: "https://eth.llamarpc.com",
    signer: /* viem/ethers signer */ null,
  });

  // Same API call, different chain
  await solana.openLong({ market: "SOL-PERP", size: 10, leverage: 5 });
  await ethereum.openLong({ market: "SOL-PERP", size: 10, leverage: 5 });

  // Aggregate state across chains
  const solState = await solana.getUserAccount();
  const ethState = await ethereum.getUserAccount();

  console.log("Solana equity:", solState.totalEquity);
  console.log("Ethereum equity:", ethState.totalEquity);
  console.log("Total exposure:", solState.totalNotional + ethState.totalNotional);
}

main().catch(console.error);
