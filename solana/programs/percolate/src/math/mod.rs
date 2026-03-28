//! Safety-critical math primitives.
//!
//! These modules provide overflow-checked operations and 256-bit
//! arithmetic for places where u128 is not enough.

pub mod safe_math;
pub mod u256;
pub mod i256;

pub use safe_math::*;
