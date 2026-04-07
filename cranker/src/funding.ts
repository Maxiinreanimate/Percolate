import { Logger } from "pino";

const INTERVAL = parseInt(process.env.FUNDING_INTERVAL || "60000", 10);

/**
 * Funding crank loop.
 *
 * Every 60 seconds:
 * - For each market, check if funding period has elapsed
 * - If yes, call crank_funding(market)
 */
export class FundingLoop {
  constructor(private chain: any, private logger: Logger) {}

  start() {
    this.logger.info({ interval: INTERVAL }, "Funding loop started");
    setInterval(() => this.tick(), INTERVAL);
  }

  private async tick() {
    try {
      // For each market: check elapsed since last_funding_time
      // If >= funding_period_seconds, call crank_funding
    } catch (err) {
      this.logger.error({ err }, "Funding tick failed");
    }
  }
}
