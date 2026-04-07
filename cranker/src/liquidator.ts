import { Logger } from "pino";

const INTERVAL = parseInt(process.env.LIQUIDATION_INTERVAL || "2000", 10);

/**
 * Liquidation loop.
 *
 * Every 2 seconds:
 * - Fetch all UserAccounts with open positions
 * - For each account below maintenance margin, call liquidate()
 * - Earn 50% of liquidation fee
 */
export class LiquidatorLoop {
  constructor(private chain: any, private logger: Logger) {}

  start() {
    this.logger.info({ interval: INTERVAL }, "Liquidator loop started");
    setInterval(() => this.tick(), INTERVAL);
  }

  private async tick() {
    try {
      // 1. Fetch all markets
      // 2. For each market, fetch all user accounts with positions
      // 3. For each account, compute aggregate equity and check vs maintenance
      // 4. If below, call liquidate(user, market)
    } catch (err) {
      this.logger.error({ err }, "Liquidator tick failed");
    }
  }
}
