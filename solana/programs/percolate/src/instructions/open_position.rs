//! open_position — open or increase a position via the vAMM.

use anchor_lang::prelude::*;
use crate::state::{Market, UserAccount, Side, SideState};
use crate::engine::vamm::{simulate_buy, simulate_sell};
use crate::errors::PercolateError;
use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OpenPositionParams {
    pub side: Side,
    pub base_size: u64,
    pub leverage: u32,
    pub max_slippage_bps: u16,
}

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub market: Account<'info, Market>,

    #[account(
        mut,
        seeds = [USER_ACCOUNT_SEED, authority.key().as_ref()],
        bump = user_account.bump,
        has_one = authority @ PercolateError::Unauthorized,
    )]
    pub user_account: Account<'info, UserAccount>,
}

pub fn handler(ctx: Context<OpenPosition>, params: OpenPositionParams) -> Result<()> {
    let market = &mut ctx.accounts.market;
    require!(market.active, PercolateError::MarketInactive);
    require!(
        params.leverage >= MIN_LEVERAGE && params.leverage <= market.max_leverage,
        PercolateError::LeverageOutOfBounds
    );

    // Check side state — refuse new OI on a side that is in DrainOnly or ResetPending
    match params.side {
        Side::Long => require!(market.long_state == SideState::Normal, PercolateError::SideDrainOnly),
        Side::Short => require!(market.short_state == SideState::Normal, PercolateError::SideDrainOnly),
    }

    let base_size = params.base_size as u128;
    let result = match params.side {
        Side::Long => simulate_buy(
            market.base_reserve,
            market.quote_reserve,
            market.k,
            market.peg_multiplier,
            base_size,
        )?,
        Side::Short => simulate_sell(
            market.base_reserve,
            market.quote_reserve,
            market.k,
            market.peg_multiplier,
            base_size,
        )?,
    };

    require!(
        result.slippage_bps <= params.max_slippage_bps,
        PercolateError::SlippageExceeded
    );

    // Update vAMM reserves
    market.base_reserve = result.new_base_reserve;
    market.quote_reserve = result.new_quote_reserve;

    // Update market OI
    match params.side {
        Side::Long => market.total_long_position = market.total_long_position.saturating_add(base_size),
        Side::Short => market.total_short_position = market.total_short_position.saturating_add(base_size),
    }

    // Update market volume + adaptive k state
    market.total_volume = market.total_volume.saturating_add(result.quote_delta);
    market.volume_24h = market.volume_24h.saturating_add(result.quote_delta);

    // Find or allocate a position slot for this user
    let acc = &mut ctx.accounts.user_account;
    let market_key = market.key();
    let slot = match acc.find_position(&market_key) {
        Some(s) => s,
        None => acc
            .allocate_position_slot()
            .ok_or(PercolateError::PositionLimitReached)?,
    };

    let pos = &mut acc.positions[slot];
    pos.market = market_key;
    let signed_size = match params.side {
        Side::Long => params.base_size as i64,
        Side::Short => -(params.base_size as i64),
    };
    pos.base_size = pos.base_size.saturating_add(signed_size);
    pos.quote_entry = pos.quote_entry.saturating_add(result.quote_delta);
    pos.leverage = params.leverage;
    pos.a_snapshot = match params.side {
        Side::Long => market.long_a,
        Side::Short => market.short_a,
    };
    pos.k_snapshot = match params.side {
        Side::Long => market.long_k_index,
        Side::Short => market.short_k_index,
    };
    pos.epoch_snapshot = match params.side {
        Side::Long => market.long_epoch,
        Side::Short => market.short_epoch,
    };

    // TODO: charge trading fee, split 8% creator / 92% protocol
    // TODO: enforce cross-margin equity check

    Ok(())
}
