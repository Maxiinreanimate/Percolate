import { Logger } from "pino";

const INTERVAL = parseInt(process.env.PEG_INTERVAL || "10000", 10);

/**
 * AMM peg updater loop.
 *
 * Every 10 seconds:
 * - For each market, compute |mark_price - oracle_price| / oracle_price
 * - If above 0.5%, call update_amm to re-anchor
 */
export class PegUpdaterLoop {
  constructor(private chain: any, private logger: Logger) {}

  start() {
    this.logger.info({ interval: INTERVAL }, "Peg updater loop started");
    setInterval(() => this.tick(), INTERVAL);
  }

  private async tick() {
    try {
      // For each market:
      //   mark = quote_reserve * peg / base_reserve
      //   oracle = read oracle
      //   drift = |mark - oracle| / oracle
      //   if drift > 0.005, call update_amm
    } catch (err) {
      this.logger.error({ err }, "Peg updater tick failed");
    }
  }
}
