export type Chain = "solana" | "ethereum" | "base" | "arbitrum";

export type Side = "long" | "short";

export type TriggerOrderType = "limit" | "stopLoss" | "takeProfit";

export type CollateralAsset = "USDC" | "USDT" | "PYUSD" | "SOL" | "ETH" | "wBTC";

export interface PercolateConfig {
  chain: Chain;
  rpc: string;
  wallet?: any;       // Solana keypair
  signer?: any;       // EVM signer (ethers/viem)
  programId?: string;
  contractAddress?: string;
}

export interface CreateMarketParams {
  token: string;
  oracle: string;
  maxLeverage: number;
  tradingFeeBps: number;
  initialK: bigint;
  kMin: bigint;
  kMax: bigint;
  maintenanceMarginBps?: number;
}

export interface OpenPositionParams {
  market: string;
  side: Side;
  size: number;
  leverage: number;
  maxSlippageBps?: number;
}

export interface DepositParams {
  collateral: CollateralAsset;
  amount: number;
}

export interface WithdrawParams {
  collateral: CollateralAsset;
  amount: number;
}

export interface TriggerOrderParams {
  market: string;
  side: Side;
  type: TriggerOrderType;
  size: number;
  triggerPrice: number;
  leverage?: number;
  reduceOnly?: boolean;
  expiry?: number;
}

export interface MarketState {
  marketId: string;
  tokenMint: string;
  creator: string;
  baseReserve: bigint;
  quoteReserve: bigint;
  k: bigint;
  kTarget: bigint;
  pegMultiplier: bigint;
  markPrice: number;
  totalLongPosition: bigint;
  totalShortPosition: bigint;
  maxLeverage: number;
  tradingFeeBps: number;
  fundingRate: number;
  volume24h: bigint;
  active: boolean;
}

export interface Position {
  market: string;
  baseSize: bigint;
  quoteEntry: bigint;
  side: Side;
  leverage: number;
  unrealizedPnl: number;
  liquidationPrice: number;
  margin: number;
}

export interface UserAccountState {
  authority: string;
  collateralBalances: Record<CollateralAsset, bigint>;
  positions: Position[];
  totalEquity: number;
  totalNotional: number;
  marginRatio: number;
}

export interface FundingInfo {
  rate: number;
  nextFundingTime: number;
  longCumulative: bigint;
  shortCumulative: bigint;
}

export type TxSig = string;
