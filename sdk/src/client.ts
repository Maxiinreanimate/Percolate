import {
  Chain,
  PercolateConfig,
  CreateMarketParams,
  OpenPositionParams,
  DepositParams,
  WithdrawParams,
  TriggerOrderParams,
  MarketState,
  Position,
  UserAccountState,
  FundingInfo,
  TxSig,
} from "./types";
import { SolanaAdapter } from "./chains/solana";
import { EvmAdapter } from "./chains/evm";

/**
 * PercolateClient — multichain client for Percolate.
 *
 * One unified API across Solana and EVM. Internal adapters handle the
 * chain-specific RPC calls and transaction format.
 *
 * @example
 * ```ts
 * const client = new PercolateClient({
 *   chain: "solana",
 *   rpc: "https://api.mainnet-beta.solana.com",
 *   wallet: keypair,
 * });
 *
 * await client.openUserAccount();
 * await client.deposit({ collateral: "USDC", amount: 1000 });
 * await client.openLong({ market: "SOL-PERP", size: 10, leverage: 5 });
 * ```
 */
export class PercolateClient {
  private adapter: SolanaAdapter | EvmAdapter;

  constructor(config: PercolateConfig) {
    if (config.chain === "solana") {
      this.adapter = new SolanaAdapter(config);
    } else {
      this.adapter = new EvmAdapter(config);
    }
  }

  // ─── User Account ───

  async openUserAccount(): Promise<TxSig> {
    return this.adapter.openUserAccount();
  }

  async deposit(params: DepositParams): Promise<TxSig> {
    return this.adapter.deposit(params);
  }

  async withdraw(params: WithdrawParams): Promise<TxSig> {
    return this.adapter.withdraw(params);
  }

  async getUserAccount(): Promise<UserAccountState> {
    return this.adapter.getUserAccount();
  }

  // ─── Markets ───

  async createMarket(params: CreateMarketParams): Promise<{ tx: TxSig; market: string }> {
    return this.adapter.createMarket(params);
  }

  async getMarkets(): Promise<MarketState[]> {
    return this.adapter.getMarkets();
  }

  async getMarket(marketId: string): Promise<MarketState> {
    return this.adapter.getMarket(marketId);
  }

  async getMarkPrice(params: { market: string }): Promise<number> {
    const market = await this.adapter.getMarket(params.market);
    return market.markPrice;
  }

  async getFundingRate(params: { market: string }): Promise<FundingInfo> {
    return this.adapter.getFundingRate(params.market);
  }

  // ─── Trading ───

  async openLong(params: Omit<OpenPositionParams, "side">): Promise<TxSig> {
    return this.adapter.openPosition({ ...params, side: "long" });
  }

  async openShort(params: Omit<OpenPositionParams, "side">): Promise<TxSig> {
    return this.adapter.openPosition({ ...params, side: "short" });
  }

  async openPosition(params: OpenPositionParams): Promise<TxSig> {
    return this.adapter.openPosition(params);
  }

  async closePosition(params: { market: string }): Promise<TxSig> {
    return this.adapter.closePosition(params.market);
  }

  async closePositionPartial(params: { market: string; size: number }): Promise<TxSig> {
    return this.adapter.closePositionPartial(params.market, params.size);
  }

  async getPosition(params: { market: string }): Promise<Position | null> {
    return this.adapter.getPosition(params.market);
  }

  // ─── Trigger Orders ───

  async placeLimitOrder(params: TriggerOrderParams): Promise<TxSig> {
    return this.adapter.placeTriggerOrder({ ...params, type: "limit" });
  }

  async placeStopLoss(params: { market: string; triggerPrice: number }): Promise<TxSig> {
    return this.adapter.placeTriggerOrder({
      market: params.market,
      side: "long",
      type: "stopLoss",
      size: 0,
      triggerPrice: params.triggerPrice,
      reduceOnly: true,
    });
  }

  async placeTakeProfit(params: { market: string; triggerPrice: number }): Promise<TxSig> {
    return this.adapter.placeTriggerOrder({
      market: params.market,
      side: "long",
      type: "takeProfit",
      size: 0,
      triggerPrice: params.triggerPrice,
      reduceOnly: true,
    });
  }

  async cancelTriggerOrder(params: { orderId: string }): Promise<TxSig> {
    return this.adapter.cancelTriggerOrder(params.orderId);
  }

  // ─── Cranker (permissionless) ───

  async liquidate(params: { user: string; market: string }): Promise<TxSig> {
    return this.adapter.liquidate(params.user, params.market);
  }

  async crankFunding(params: { market: string }): Promise<TxSig> {
    return this.adapter.crankFunding(params.market);
  }

  async executeTriggerOrder(params: { orderId: string }): Promise<TxSig> {
    return this.adapter.executeTriggerOrder(params.orderId);
  }

  async updateAmm(params: { market: string }): Promise<TxSig> {
    return this.adapter.updateAmm(params.market);
  }

  async adaptK(params: { market: string }): Promise<TxSig> {
    return this.adapter.adaptK(params.market);
  }

  // ─── Chain info ───

  get chain(): Chain {
    return this.adapter.chain;
  }
}
