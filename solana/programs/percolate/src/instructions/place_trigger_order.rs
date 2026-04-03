//! place_trigger_order — create a limit, stop loss, or take profit order.

use anchor_lang::prelude::*;
use crate::state::{TriggerOrder, TriggerOrderType, Market, Side};
use crate::constants::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct PlaceTriggerOrderParams {
    pub order_id: u64,
    pub order_type: TriggerOrderType,
    pub side: Side,
    pub size: u64,
    pub trigger_price: u64,
    pub leverage: u32,
    pub reduce_only: bool,
    pub expiry: i64,
}

#[derive(Accounts)]
#[instruction(params: PlaceTriggerOrderParams)]
pub struct PlaceTriggerOrder<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    pub market: Account<'info, Market>,

    #[account(
        init,
        payer = authority,
        space = TriggerOrder::LEN,
        seeds = [
            TRIGGER_SEED,
            market.key().as_ref(),
            authority.key().as_ref(),
            &params.order_id.to_le_bytes()
        ],
        bump
    )]
    pub trigger_order: Account<'info, TriggerOrder>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PlaceTriggerOrder>, params: PlaceTriggerOrderParams) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let order = &mut ctx.accounts.trigger_order;
    order.authority = ctx.accounts.authority.key();
    order.market = ctx.accounts.market.key();
    order.order_id = params.order_id;
    order.order_type = params.order_type;
    order.side = params.side;
    order.size = params.size;
    order.trigger_price = params.trigger_price;
    order.leverage = params.leverage;
    order.reduce_only = params.reduce_only;
    order.created_at = now;
    order.expiry = params.expiry;
    order.bump = ctx.bumps.trigger_order;
    Ok(())
}
