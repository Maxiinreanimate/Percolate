//! 256-bit unsigned integer math.
//!
//! Used for cases where u128 is not enough, like (k * peg) computations
//! where both inputs are already u128.
//!
//! This module is intentionally minimal and only implements the operations
//! used by the protocol.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct U256 {
    pub hi: u128,
    pub lo: u128,
}

impl U256 {
    pub const ZERO: Self = Self { hi: 0, lo: 0 };

    pub fn from_u128(x: u128) -> Self {
        Self { hi: 0, lo: x }
    }

    pub fn checked_mul_u128(a: u128, b: u128) -> Option<Self> {
        let result = (a as u128).checked_mul(b)?;
        Some(Self::from_u128(result))
    }

    pub fn try_into_u128(self) -> Option<u128> {
        if self.hi == 0 {
            Some(self.lo)
        } else {
            None
        }
    }
}
