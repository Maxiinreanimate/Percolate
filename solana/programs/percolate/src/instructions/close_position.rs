//! close_position — close all or part of a position.

use anchor_lang::prelude::*;
use crate::state::{Market, UserAccount};
use crate::engine::vamm::{simulate_buy, simulate_sell};
use crate::errors::PercolateError;
use crate::constants::*;

#[derive(Accounts)]
pub struct ClosePosition<'info> {
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

pub fn handler(ctx: Context<ClosePosition>, base_size: Option<u64>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let acc = &mut ctx.accounts.user_account;

    let market_key = market.key();
    let slot = acc
        .find_position(&market_key)
        .ok_or(PercolateError::PositionNotFound)?;

    let pos = &mut acc.positions[slot];
    let close_size = base_size.unwrap_or(pos.base_size.unsigned_abs()) as u128;

    require!(close_size > 0, PercolateError::PositionNotFound);

    let is_long = pos.base_size > 0;

    // Reverse trade against vAMM
    let result = if is_long {
        simulate_sell(
            market.base_reserve,
            market.quote_reserve,
            market.k,
            market.peg_multiplier,
            close_size,
        )?
    } else {
        simulate_buy(
            market.base_reserve,
            market.quote_reserve,
            market.k,
            market.peg_multiplier,
            close_size,
        )?
    };

    market.base_reserve = result.new_base_reserve;
    market.quote_reserve = result.new_quote_reserve;

    if is_long {
        market.total_long_position = market.total_long_position.saturating_sub(close_size);
    } else {
        market.total_short_position = market.total_short_position.saturating_sub(close_size);
    }

    market.total_volume = market.total_volume.saturating_add(result.quote_delta);

    // Update position
    let close_size_i64 = close_size as i64;
    if is_long {
        pos.base_size -= close_size_i64;
    } else {
        pos.base_size += close_size_i64;
    }

    if pos.base_size == 0 {
        pos.quote_entry = 0;
    } else {
        // Reduce quote_entry proportionally
        let original = pos.quote_entry;
        pos.quote_entry = original.saturating_sub(result.quote_delta);
    }

    Ok(())
}
