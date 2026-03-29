//! Error codes for the Percolate program.

use anchor_lang::prelude::*;

#[error_code]
pub enum PercolateError {
    #[msg("Protocol is paused")]
    Paused,

    #[msg("Market is not active")]
    MarketInactive,

    #[msg("Leverage out of bounds")]
    LeverageOutOfBounds,

    #[msg("Trading fee out of bounds")]
    TradingFeeOutOfBounds,

    #[msg("Initial k below minimum")]
    KBelowMinimum,

    #[msg("Insufficient margin")]
    InsufficientMargin,

    #[msg("Insufficient collateral")]
    InsufficientCollateral,

    #[msg("Slippage exceeded")]
    SlippageExceeded,

    #[msg("Position not found")]
    PositionNotFound,

    #[msg("Position too large")]
    PositionTooLarge,

    #[msg("Account already at position limit")]
    PositionLimitReached,

    #[msg("Account at collateral limit")]
    CollateralLimitReached,

    #[msg("Collateral not registered")]
    CollateralNotRegistered,

    #[msg("Collateral disabled")]
    CollateralDisabled,

    #[msg("Account below maintenance margin")]
    BelowMaintenanceMargin,

    #[msg("Account healthy, cannot liquidate")]
    NotLiquidatable,

    #[msg("Funding period not yet elapsed")]
    FundingPeriodNotElapsed,

    #[msg("Oracle stale")]
    OracleStale,

    #[msg("Oracle divergence too high")]
    OracleDivergence,

    #[msg("Trigger condition not met")]
    TriggerConditionNotMet,

    #[msg("Trigger order expired")]
    TriggerOrderExpired,

    #[msg("Trigger order limit reached")]
    TriggerOrderLimitReached,

    #[msg("Math overflow")]
    MathOverflow,

    #[msg("Math underflow")]
    MathUnderflow,

    #[msg("Division by zero")]
    DivisionByZero,

    #[msg("Adaptive k interval not elapsed")]
    AdaptKIntervalNotElapsed,

    #[msg("Side in drain-only mode")]
    SideDrainOnly,

    #[msg("Side in reset pending")]
    SideResetPending,

    #[msg("Account equity below minimum")]
    EquityBelowMinimum,

    #[msg("Unauthorized")]
    Unauthorized,
}
