import "dotenv/config";
import pino from "pino";
import { LiquidatorLoop } from "./liquidator";
import { FundingLoop } from "./funding";
import { TriggerExecutorLoop } from "./trigger-executor";
import { PegUpdaterLoop } from "./peg-updater";
import { AdaptiveKLoop } from "./adaptive-k";

const logger = pino({
  level: process.env.LOG_LEVEL || "info",
  transport: { target: "pino-pretty" },
});

interface ChainConfig {
  name: string;
  type: "solana" | "evm";
  rpc: string;
  signer: any;
  contractAddress?: string;
}

async function main() {
  logger.info("Percolate Cranker starting");

  const chains: ChainConfig[] = [];

  if (process.env.SOLANA_RPC) {
    chains.push({
      name: "solana",
      type: "solana",
      rpc: process.env.SOLANA_RPC,
      signer: null,
    });
  }
  if (process.env.ETH_RPC) {
    chains.push({
      name: "ethereum",
      type: "evm",
      rpc: process.env.ETH_RPC,
      signer: null,
    });
  }
  if (process.env.BASE_RPC) {
    chains.push({
      name: "base",
      type: "evm",
      rpc: process.env.BASE_RPC,
      signer: null,
    });
  }
  if (process.env.ARB_RPC) {
    chains.push({
      name: "arbitrum",
      type: "evm",
      rpc: process.env.ARB_RPC,
      signer: null,
    });
  }

  logger.info({ chains: chains.map((c) => c.name) }, "Configured chains");

  for (const chain of chains) {
    const child = logger.child({ chain: chain.name });
    new LiquidatorLoop(chain, child).start();
    new FundingLoop(chain, child).start();
    new TriggerExecutorLoop(chain, child).start();
    new PegUpdaterLoop(chain, child).start();
    new AdaptiveKLoop(chain, child).start();
  }

  process.on("SIGINT", () => {
    logger.info("Shutting down");
    process.exit(0);
  });
}

main().catch((err) => {
  logger.error({ err }, "Cranker crashed");
  process.exit(1);
});
