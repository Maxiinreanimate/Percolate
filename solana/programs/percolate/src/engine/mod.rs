//! Percolate engine modules.
//!
//! Pure mathematical logic for the protocol. These modules contain no
//! Solana runtime dependencies which makes them suitable for formal
//! verification with Kani and direct unit testing.

pub mod vamm;
pub mod risk;
pub mod funding;
pub mod margin;
pub mod liquidation;
pub mod oracle;
pub mod warmup;
pub mod adaptive;
pub mod collateral_haircut;

pub use vamm::*;
pub use risk::*;
pub use funding::*;
pub use margin::*;
pub use liquidation::*;
pub use oracle::*;
pub use warmup::*;
pub use adaptive::*;
pub use collateral_haircut::*;
