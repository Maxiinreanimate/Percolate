//! 256-bit signed integer math.
//!
//! Used for funding indices and PnL accumulation where the value can
//! be negative and large enough to overflow i128.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct I256 {
    pub negative: bool,
    pub abs_hi: u128,
    pub abs_lo: u128,
}

impl I256 {
    pub const ZERO: Self = Self {
        negative: false,
        abs_hi: 0,
        abs_lo: 0,
    };

    pub fn from_i128(x: i128) -> Self {
        let negative = x < 0;
        let abs = x.unsigned_abs();
        Self {
            negative,
            abs_hi: 0,
            abs_lo: abs,
        }
    }
}
