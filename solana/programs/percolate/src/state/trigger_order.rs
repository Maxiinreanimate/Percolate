//! Trigger orders (limit, stop loss, take profit).

use anchor_lang::prelude::*;
use crate::state::market::Side;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Debug)]
pub enum TriggerOrderType {
    Limit,
    StopLoss,
    TakeProfit,
}

#[account]
pub struct TriggerOrder {
    pub authority: Pubkey,
    pub market: Pubkey,
    pub order_id: u64,

    pub order_type: TriggerOrderType,
    pub side: Side,
    pub size: u64,
    pub trigger_price: u64,
    pub leverage: u32,
    pub reduce_only: bool,

    pub created_at: i64,
    pub expiry: i64,

    pub bump: u8,
}

impl TriggerOrder {
    pub const LEN: usize = 8 +
        32 + 32 + 8 +
        1 + 1 + 8 + 8 + 4 + 1 +
        8 + 8 +
        1;
}
