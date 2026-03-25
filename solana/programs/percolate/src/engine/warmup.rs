//! PnL warmup window.
//!
//! Fresh profit sits in `reserved_pnl` and converts to released profit
//! linearly over `warmup_period_slots`. This prevents oracle manipulation
//! attacks where someone pumps a price, opens a position, and tries to
//! claim profit immediately.

pub fn matured_amount(
    reserved: u64,
    warmup_started_at: u64,
    current_slot: u64,
    warmup_period: u64,
) -> u64 {
    if warmup_period == 0 || current_slot <= warmup_started_at {
        return 0;
    }
    let elapsed = current_slot - warmup_started_at;
    if elapsed >= warmup_period {
        return reserved;
    }
    ((reserved as u128 * elapsed as u128) / warmup_period as u128) as u64
}

pub fn pending_amount(
    reserved: u64,
    warmup_started_at: u64,
    current_slot: u64,
    warmup_period: u64,
) -> u64 {
    let matured = matured_amount(reserved, warmup_started_at, current_slot, warmup_period);
    reserved.saturating_sub(matured)
}
