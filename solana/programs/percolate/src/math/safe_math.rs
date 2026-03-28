//! Safe math operations on u128 and i128.
//!
//! All operations return Option to surface overflow / underflow / div-by-zero.

pub fn safe_add(a: u128, b: u128) -> Option<u128> {
    a.checked_add(b)
}

pub fn safe_sub(a: u128, b: u128) -> Option<u128> {
    a.checked_sub(b)
}

pub fn safe_mul(a: u128, b: u128) -> Option<u128> {
    a.checked_mul(b)
}

pub fn safe_div(a: u128, b: u128) -> Option<u128> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// Compute (a * b) / c with intermediate u256 to avoid overflow.
pub fn mul_div(a: u128, b: u128, c: u128) -> Option<u128> {
    if c == 0 {
        return None;
    }
    let product = (a as u256_alias::U256) * (b as u256_alias::U256);
    let result = product / (c as u256_alias::U256);
    if result > u128::MAX as u256_alias::U256 {
        None
    } else {
        Some(result as u128)
    }
}

mod u256_alias {
    pub type U256 = u128;
}
