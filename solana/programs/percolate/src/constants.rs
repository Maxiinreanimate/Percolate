//! Protocol constants
//! All limits, defaults, and scaling factors live here.

// ─────────────────────────────────────────────────────────────────────
// Protocol Fee Configuration
// ─────────────────────────────────────────────────────────────────────

/// Creator earns 8% of all trading fees on their market forever.
pub const CREATOR_FEE_SHARE_BPS: u16 = 800;

/// Minimum trading fee a creator can set (0.03%).
pub const MIN_TRADING_FEE_BPS: u16 = 3;

/// Maximum trading fee a creator can set (1%).
pub const MAX_TRADING_FEE_BPS: u16 = 100;

/// Liquidation fee charged on the closed notional (1%).
pub const LIQUIDATION_FEE_BPS: u16 = 100;

/// Liquidator's share of the liquidation fee (50%).
pub const LIQUIDATOR_SHARE_BPS: u16 = 5_000;

/// Trigger order execution fee (0.01%).
pub const TRIGGER_EXECUTION_FEE_BPS: u16 = 1;

/// Reward paid to crankers for adapting k.
pub const ADAPTIVE_K_REWARD: u64 = 1_000_000; // 1 USDC

// ─────────────────────────────────────────────────────────────────────
// Cross-Margin Configuration
// ─────────────────────────────────────────────────────────────────────

/// Maximum number of positions a single user account can hold.
pub const MAX_POSITIONS_PER_ACCOUNT: usize = 16;

/// Maximum number of registered collateral types in the protocol.
pub const MAX_COLLATERAL_TYPES: usize = 8;

/// Maximum collateral balances tracked per user account.
pub const MAX_COLLATERAL_PER_ACCOUNT: usize = 8;

/// Minimum equity required to keep a user account open (10 USDC).
pub const MIN_ACCOUNT_EQUITY: u64 = 10_000_000;

// ─────────────────────────────────────────────────────────────────────
// Market Defaults
// ─────────────────────────────────────────────────────────────────────

pub const DEFAULT_MAX_LEVERAGE: u32 = 2_000;          // 20x
pub const MIN_LEVERAGE: u32 = 100;                    // 1x
pub const MAX_LEVERAGE_CAP: u32 = 2_000;              // 20x absolute ceiling

pub const MAINTENANCE_MARGIN_BPS: u16 = 500;          // 5%
pub const DEFAULT_FUNDING_PERIOD_SECONDS: u32 = 3_600; // 1 hour
pub const FUNDING_RATE_CAP_BPS: u16 = 10;             // 0.1% per period
pub const WARMUP_PERIOD_SLOTS: u64 = 1_000;           // ~400 seconds

pub const MAX_TRIGGER_ORDERS_PER_USER: u8 = 16;

// ─────────────────────────────────────────────────────────────────────
// Adaptive K Controller
// ─────────────────────────────────────────────────────────────────────

/// Smoothing window for the adaptive k controller (1 hour).
pub const ADAPTIVE_K_WINDOW_SECONDS: u32 = 3_600;

/// Minimum interval between adapt_k calls (60 seconds).
pub const ADAPTIVE_K_MIN_INTERVAL: u32 = 60;

/// Decay rate for the 7-day rolling volume average (1% per cycle).
pub const VOLUME_AVG_DECAY_BPS: u16 = 100;

/// Maximum k multiplier (k can grow up to 10x base).
pub const ADAPTIVE_K_MAX_MULTIPLIER_BPS: u16 = 100_000;

/// Minimum k multiplier (k can shrink to 25% of base).
pub const ADAPTIVE_K_MIN_MULTIPLIER_BPS: u16 = 2_500;

// ─────────────────────────────────────────────────────────────────────
// Oracle
// ─────────────────────────────────────────────────────────────────────

pub const ORACLE_STALENESS_SECONDS: u32 = 30;
pub const ORACLE_CONFIDENCE_BPS: u16 = 200;            // 2%
pub const AMM_PEG_THRESHOLD_BPS: u16 = 50;             // 0.5% drift triggers re-peg
pub const MAX_ORACLE_DIVERGENCE_BPS: u16 = 500;        // 5% between sources

// ─────────────────────────────────────────────────────────────────────
// Multi-Collateral Haircuts
// ─────────────────────────────────────────────────────────────────────

/// Default haircut for stablecoins (0%).
pub const STABLE_HAIRCUT_BPS: u16 = 0;

/// Default static haircut for SOL collateral (15%).
pub const SOL_STATIC_HAIRCUT_BPS: u16 = 1_500;

/// Default static haircut for ETH collateral (15%).
pub const ETH_STATIC_HAIRCUT_BPS: u16 = 1_500;

/// Default static haircut for wBTC collateral (10%).
pub const WBTC_STATIC_HAIRCUT_BPS: u16 = 1_000;

/// Maximum dynamic haircut adjustment (10%).
pub const MAX_DYNAMIC_HAIRCUT_BPS: u16 = 1_000;

/// Volatility window for dynamic haircut adjustment (30 days).
pub const VOLATILITY_WINDOW_DAYS: u32 = 30;

// ─────────────────────────────────────────────────────────────────────
// Precision / Scaling
// ─────────────────────────────────────────────────────────────────────

pub const PRICE_SCALE: u64 = 1_000_000;                // 6 decimals
pub const K_SCALE: u128 = 1_000_000_000_000;           // 12 decimals
pub const PEG_SCALE: u128 = 1_000_000;                 // 6 decimals
pub const POS_SCALE: u128 = 1_000_000;                 // 6 decimals
pub const HAIRCUT_SCALE: u16 = 10_000;                 // bps

// ─────────────────────────────────────────────────────────────────────
// PDA Seeds
// ─────────────────────────────────────────────────────────────────────

pub const PROTOCOL_SEED: &[u8] = b"protocol";
pub const MARKET_SEED: &[u8] = b"market";
pub const USER_ACCOUNT_SEED: &[u8] = b"user_account";
pub const COLLATERAL_SEED: &[u8] = b"collateral";
pub const TRIGGER_SEED: &[u8] = b"trigger";
pub const VAULT_SEED: &[u8] = b"vault";
pub const INSURANCE_SEED: &[u8] = b"insurance";
