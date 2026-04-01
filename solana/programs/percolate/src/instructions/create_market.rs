//! create_market — permissionless market creation.
//!
//! Anyone can call this to launch a new perp market for any token.
//! Creator earns 8% of all trading fees on this market forever.

use anchor_lang::prelude::*;
use crate::state::{Protocol, Market, OracleSource, SideState};
use crate::errors::PercolateError;
use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateMarketParams {
    pub oracle_source: OracleSource,
    pub max_leverage: u32,
    pub trading_fee_bps: u16,
    pub initial_k: u128,
    pub k_min: u128,
    pub k_max: u128,
    pub maintenance_margin_bps: u16,
}

#[derive(Accounts)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [PROTOCOL_SEED],
        bump = protocol.bump,
    )]
    pub protocol: Account<'info, Protocol>,

    /// CHECK: Token mint of the asset being traded.
    pub token_mint: UncheckedAccount<'info>,

    /// CHECK: Oracle for pricing the base asset.
    pub oracle: UncheckedAccount<'info>,

    /// CHECK: Creator fee account.
    pub creator_fee_account: UncheckedAccount<'info>,

    #[account(
        init,
        payer = creator,
        space = Market::LEN,
        seeds = [MARKET_SEED, token_mint.key().as_ref(), creator.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateMarket>, params: CreateMarketParams) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol;
    require!(!protocol.paused, PercolateError::Paused);

    require!(
        params.max_leverage >= MIN_LEVERAGE && params.max_leverage <= MAX_LEVERAGE_CAP,
        PercolateError::LeverageOutOfBounds
    );
    require!(
        params.trading_fee_bps >= protocol.min_trading_fee_bps
            && params.trading_fee_bps <= protocol.max_trading_fee_bps,
        PercolateError::TradingFeeOutOfBounds
    );
    require!(
        params.initial_k >= protocol.min_initial_k,
        PercolateError::KBelowMinimum
    );
    require!(params.k_min <= params.initial_k, PercolateError::KBelowMinimum);
    require!(params.k_max >= params.initial_k, PercolateError::KBelowMinimum);

    let now = Clock::get()?.unix_timestamp;
    let market = &mut ctx.accounts.market;

    market.market_index = protocol.market_count;
    market.token_mint = ctx.accounts.token_mint.key();
    market.creator = ctx.accounts.creator.key();
    market.creator_fee_account = ctx.accounts.creator_fee_account.key();

    // vAMM init
    let initial_base = (params.initial_k as f64).sqrt() as u128;
    let initial_quote = params.initial_k / initial_base.max(1);
    market.base_reserve = initial_base;
    market.quote_reserve = initial_quote;
    market.k = params.initial_k;
    market.k_target = params.initial_k;
    market.k_base = params.initial_k;
    market.k_min = params.k_min;
    market.k_max = params.k_max;
    market.k_last_adjusted = now;
    market.peg_multiplier = PEG_SCALE as u128;
    market.total_long_position = 0;
    market.total_short_position = 0;

    // Adaptive k state
    market.volume_24h = 0;
    market.volume_avg_7d = 0;
    market.volatility_score = 0;

    // Params
    market.max_leverage = params.max_leverage;
    market.trading_fee_bps = params.trading_fee_bps;
    market.liquidation_fee_bps = LIQUIDATION_FEE_BPS;
    market.maintenance_margin_bps = params.maintenance_margin_bps.max(MAINTENANCE_MARGIN_BPS);

    // Oracle
    market.oracle_source = params.oracle_source;
    market.oracle_address = ctx.accounts.oracle.key();

    // Risk engine
    market.insurance_fund_balance = 0;
    market.haircut_numerator = 1;
    market.haircut_denominator = 1;
    market.long_a = 1_000_000;
    market.long_k_index = 0;
    market.short_a = 1_000_000;
    market.short_k_index = 0;
    market.long_epoch = 0;
    market.short_epoch = 0;
    market.long_state = SideState::Normal;
    market.short_state = SideState::Normal;

    // Funding
    market.last_funding_time = now;
    market.cumulative_long_funding = 0;
    market.cumulative_short_funding = 0;
    market.funding_period_seconds = DEFAULT_FUNDING_PERIOD_SECONDS;
    market.funding_rate_cap_bps = FUNDING_RATE_CAP_BPS;

    market.warmup_period_slots = WARMUP_PERIOD_SLOTS;
    market.creator_fees_earned = 0;
    market.protocol_fees_earned = 0;
    market.total_volume = 0;
    market.active = true;
    market.bump = ctx.bumps.market;
    market.created_at = now;

    protocol.market_count = protocol.market_count.saturating_add(1);
    Ok(())
}
