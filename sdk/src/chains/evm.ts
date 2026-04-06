import { Chain, PercolateConfig, CreateMarketParams, OpenPositionParams, DepositParams, WithdrawParams, TriggerOrderParams, MarketState, Position, UserAccountState, FundingInfo, TxSig } from "../types";

/**
 * EVM adapter for the PercolateClient.
 *
 * Works against any EVM chain (Ethereum, Base, Arbitrum) by pointing
 * at the Percolate.sol contract address for that chain.
 */
export class EvmAdapter {
  readonly chain: Chain;
  private rpc: string;
  private signer: any;
  private contractAddress: string;

  constructor(config: PercolateConfig) {
    this.chain = config.chain;
    this.rpc = config.rpc;
    this.signer = config.signer;
    this.contractAddress = config.contractAddress || "0x0000000000000000000000000000000000000000";
  }

  // All methods stubbed identically to SolanaAdapter — same surface,
  // different runtime. In production these would use viem or ethers
  // to call the Percolate.sol contract.

  async openUserAccount(): Promise<TxSig> { return "0xplaceholder"; }
  async deposit(p: DepositParams): Promise<TxSig> { return "0xplaceholder"; }
  async withdraw(p: WithdrawParams): Promise<TxSig> { return "0xplaceholder"; }
  async getUserAccount(): Promise<UserAccountState> {
    return { authority: "", collateralBalances: {} as any, positions: [], totalEquity: 0, totalNotional: 0, marginRatio: 0 };
  }
  async createMarket(p: CreateMarketParams): Promise<{ tx: TxSig; market: string }> {
    return { tx: "0xplaceholder", market: "0xmarket" };
  }
  async getMarkets(): Promise<MarketState[]> { return []; }
  async getMarket(marketId: string): Promise<MarketState> {
    return {
      marketId, tokenMint: "", creator: "",
      baseReserve: 0n, quoteReserve: 0n, k: 0n, kTarget: 0n, pegMultiplier: 0n,
      markPrice: 0, totalLongPosition: 0n, totalShortPosition: 0n,
      maxLeverage: 0, tradingFeeBps: 0, fundingRate: 0, volume24h: 0n, active: false,
    };
  }
  async getFundingRate(market: string): Promise<FundingInfo> {
    return { rate: 0, nextFundingTime: 0, longCumulative: 0n, shortCumulative: 0n };
  }
  async openPosition(p: OpenPositionParams): Promise<TxSig> { return "0xplaceholder"; }
  async closePosition(market: string): Promise<TxSig> { return "0xplaceholder"; }
  async closePositionPartial(market: string, size: number): Promise<TxSig> { return "0xplaceholder"; }
  async getPosition(market: string): Promise<Position | null> { return null; }
  async placeTriggerOrder(p: TriggerOrderParams): Promise<TxSig> { return "0xplaceholder"; }
  async cancelTriggerOrder(orderId: string): Promise<TxSig> { return "0xplaceholder"; }
  async liquidate(user: string, market: string): Promise<TxSig> { return "0xplaceholder"; }
  async crankFunding(market: string): Promise<TxSig> { return "0xplaceholder"; }
  async executeTriggerOrder(orderId: string): Promise<TxSig> { return "0xplaceholder"; }
  async updateAmm(market: string): Promise<TxSig> { return "0xplaceholder"; }
  async adaptK(market: string): Promise<TxSig> { return "0xplaceholder"; }
}
