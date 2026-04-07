import { Logger } from "pino";

const INTERVAL = parseInt(process.env.ADAPTIVE_K_INTERVAL || "60000", 10);

/**
 * Adaptive k controller loop.
 *
 * Every 60 seconds:
 * - For each market, call adapt_k
 * - The on-chain controller reads volume_24h and volatility_score
 * - Computes a new k_target and smooths actual k toward it
 * - Earn fixed reward per call
 */
export class AdaptiveKLoop {
  constructor(private chain: any, private logger: Logger) {}

  start() {
    this.logger.info({ interval: INTERVAL }, "Adaptive k loop started");
    setInterval(() => this.tick(), INTERVAL);
  }

  private async tick() {
    try {
      // For each market: call adapt_k
      // The on-chain controller handles all the math
    } catch (err) {
      this.logger.error({ err }, "Adaptive k tick failed");
    }
  }
}
