import { Logger } from "pino";

const INTERVAL = parseInt(process.env.TRIGGER_INTERVAL || "1000", 10);

/**
 * Trigger order executor loop.
 *
 * Every 1 second:
 * - Get current oracle price for each market
 * - Fetch all TriggerOrders
 * - For each order where the trigger condition is met, call execute_trigger_order
 * - Earn 0.01% of notional as execution fee
 */
export class TriggerExecutorLoop {
  constructor(private chain: any, private logger: Logger) {}

  start() {
    this.logger.info({ interval: INTERVAL }, "Trigger executor loop started");
    setInterval(() => this.tick(), INTERVAL);
  }

  private async tick() {
    try {
      // For each market:
      //   - Get oracle price
      //   - Fetch TriggerOrders
      //   - For each order:
      //     - LimitLong: price <= trigger_price
      //     - LimitShort: price >= trigger_price
      //     - StopLossLong: price <= trigger_price
      //     - StopLossShort: price >= trigger_price
      //     - TakeProfitLong: price >= trigger_price
      //     - TakeProfitShort: price <= trigger_price
      //   - If condition met, call execute_trigger_order
    } catch (err) {
      this.logger.error({ err }, "Trigger executor tick failed");
    }
  }
}
