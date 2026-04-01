//! open_user_account — creates the cross-margin user account.

use anchor_lang::prelude::*;
use crate::state::UserAccount;
use crate::constants::USER_ACCOUNT_SEED;

#[derive(Accounts)]
pub struct OpenUserAccount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = UserAccount::LEN,
        seeds = [USER_ACCOUNT_SEED, authority.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<OpenUserAccount>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let acc = &mut ctx.accounts.user_account;
    acc.authority = ctx.accounts.authority.key();
    acc.collateral_balances = Default::default();
    acc.positions = [Default::default(); crate::constants::MAX_POSITIONS_PER_ACCOUNT];
    acc.position_count = 0;
    acc.total_margin_used = 0;
    acc.total_unrealized_pnl = 0;
    acc.total_realized_pnl = 0;
    acc.fee_credits = 0;
    acc.last_settled_at = now;
    acc.bump = ctx.bumps.user_account;
    acc.created_at = now;
    Ok(())
}
