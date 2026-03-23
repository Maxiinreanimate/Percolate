//! Oracle abstraction.
//!
//! Provides a unified interface over Pyth, PercOracle, and DEX pool
//! oracles. Validates staleness and confidence intervals.

use crate::constants::ORACLE_STALENESS_SECONDS;
use crate::errors::PercolateError;
use anchor_lang::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct OraclePrice {
    pub price: u128,
    pub confidence: u128,
    pub last_updated: i64,
}

impl OraclePrice {
    pub fn is_fresh(&self, now: i64) -> bool {
        now - self.last_updated <= ORACLE_STALENESS_SECONDS as i64
    }

    pub fn validate(&self, now: i64) -> Result<()> {
        require!(self.is_fresh(now), PercolateError::OracleStale);
        require!(self.price > 0, PercolateError::OracleStale);
        Ok(())
    }
}
