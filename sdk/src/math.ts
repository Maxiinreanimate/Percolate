import { Side } from "./types";

const PRICE_SCALE = 1_000_000n;
const POS_SCALE = 1_000_000n;
const BPS = 10_000n;

/**
 * Compute the liquidation price for a position.
 *
 * Long: liq = entry * (1 - 1/leverage + mm/10000)
 * Short: liq = entry * (1 + 1/leverage - mm/10000)
 */
export function calculateLiquidationPrice(
  entry: number,
  leverage: number,
  side: Side,
  maintenanceMarginBps: number = 500
): number {
  const lev = leverage / 100;
  const mm = maintenanceMarginBps / 10_000;
  if (side === "long") {
    return entry * (1 - 1 / lev + mm);
  } else {
    return entry * (1 + 1 / lev - mm);
  }
}

/**
 * Compute the unrealized PnL for a position.
 */
export function calculatePnl(
  side: Side,
  entryQuote: number,
  exitQuote: number
): number {
  if (side === "long") {
    return exitQuote - entryQuote;
  } else {
    return entryQuote - exitQuote;
  }
}

/**
 * Compute the initial margin required to open a position.
 */
export function calculateMarginRequired(
  notional: number,
  leverage: number
): number {
  return notional / (leverage / 100);
}

/**
 * Estimate slippage for a trade against the vAMM.
 */
export function estimateSlippage(
  baseSize: number,
  baseReserve: number,
  quoteReserve: number
): number {
  if (baseReserve === 0) return 0;
  const k = baseReserve * quoteReserve;
  const newBase = baseReserve - baseSize;
  if (newBase <= 0) return 1;
  const newQuote = k / newBase;
  const quoteCost = newQuote - quoteReserve;
  const oraclePrice = quoteReserve / baseReserve;
  const effectivePrice = quoteCost / baseSize;
  return Math.abs(effectivePrice - oraclePrice) / oraclePrice;
}

/**
 * Compute the funding payment for a position over a period.
 */
export function calculateFundingPayment(
  baseSize: number,
  fundingRateBps: number
): number {
  return baseSize * (fundingRateBps / 10_000);
}

/**
 * Apply the haircut H to a profit amount.
 */
export function applyHaircut(profit: number, haircutNum: bigint, haircutDenom: bigint): number {
  if (haircutDenom === 0n) return 0;
  return (profit * Number(haircutNum)) / Number(haircutDenom);
}
