//! Percolate
//!
//! Multichain permissionless perpetual futures DEX.
//! Built on Anatoly Yakovenko's Percolator risk engine.
//!
//! Extended with:
//! - Cross-margin native (single account, multiple positions, shared collateral)
//! - Multi-collateral (USDC, USDT, SOL, ETH, wBTC with dynamic haircuts)
//! - Adaptive liquidity (k auto-tunes based on real-time volume and volatility)

use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod state;
pub mod instructions;
pub mod engine;
pub mod math;

use instructions::*;

declare_id!("PercoLat3MultichainDexProtocol1111111111111");

#[program]
pub mod percolate {
    use super::*;

    // ─────────────────────────────────────────────────────────────────
    // Protocol Admin
    // ─────────────────────────────────────────────────────────────────

    pub fn initialize_protocol(
        ctx: Context<InitializeProtocol>,
        params: InitializeProtocolParams,
    ) -> Result<()> {
        instructions::initialize_protocol::handler(ctx, params)
    }

    pub fn admin_pause(ctx: Context<AdminPause>, paused: bool) -> Result<()> {
        instructions::admin_pause::handler(ctx, paused)
    }

    pub fn admin_update_protocol(
        ctx: Context<AdminUpdateProtocol>,
        params: AdminUpdateProtocolParams,
    ) -> Result<()> {
        instructions::admin_update_protocol::handler(ctx, params)
    }

    pub fn register_collateral(
        ctx: Context<RegisterCollateral>,
        params: RegisterCollateralParams,
    ) -> Result<()> {
        instructions::register_collateral::handler(ctx, params)
    }

    // ─────────────────────────────────────────────────────────────────
    // Market Creation (Permissionless)
    // ─────────────────────────────────────────────────────────────────

    pub fn create_market(
        ctx: Context<CreateMarket>,
        params: CreateMarketParams,
    ) -> Result<()> {
        instructions::create_market::handler(ctx, params)
    }

    // ─────────────────────────────────────────────────────────────────
    // User — Account Management
    // ─────────────────────────────────────────────────────────────────

    pub fn open_user_account(ctx: Context<OpenUserAccount>) -> Result<()> {
        instructions::open_user_account::handler(ctx)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        collateral_index: u8,
        amount: u64,
    ) -> Result<()> {
        instructions::deposit::handler(ctx, collateral_index, amount)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        collateral_index: u8,
        amount: u64,
    ) -> Result<()> {
        instructions::withdraw::handler(ctx, collateral_index, amount)
    }

    // ─────────────────────────────────────────────────────────────────
    // Trading
    // ─────────────────────────────────────────────────────────────────

    pub fn open_position(
        ctx: Context<OpenPosition>,
        params: OpenPositionParams,
    ) -> Result<()> {
        instructions::open_position::handler(ctx, params)
    }

    pub fn close_position(
        ctx: Context<ClosePosition>,
        base_size: Option<u64>,
    ) -> Result<()> {
        instructions::close_position::handler(ctx, base_size)
    }

    // ─────────────────────────────────────────────────────────────────
    // Trigger Orders
    // ─────────────────────────────────────────────────────────────────

    pub fn place_trigger_order(
        ctx: Context<PlaceTriggerOrder>,
        params: PlaceTriggerOrderParams,
    ) -> Result<()> {
        instructions::place_trigger_order::handler(ctx, params)
    }

    pub fn execute_trigger_order(ctx: Context<ExecuteTriggerOrder>) -> Result<()> {
        instructions::execute_trigger_order::handler(ctx)
    }

    pub fn cancel_trigger_order(ctx: Context<CancelTriggerOrder>) -> Result<()> {
        instructions::cancel_trigger_order::handler(ctx)
    }

    // ─────────────────────────────────────────────────────────────────
    // Maintenance (Permissionless)
    // ─────────────────────────────────────────────────────────────────

    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        instructions::liquidate::handler(ctx)
    }

    pub fn crank_funding(ctx: Context<CrankFunding>) -> Result<()> {
        instructions::crank_funding::handler(ctx)
    }

    pub fn update_amm(ctx: Context<UpdateAmm>) -> Result<()> {
        instructions::update_amm::handler(ctx)
    }

    pub fn adapt_k(ctx: Context<AdaptK>) -> Result<()> {
        instructions::adapt_k::handler(ctx)
    }

    pub fn settle_pnl(ctx: Context<SettlePnl>) -> Result<()> {
        instructions::settle_pnl::handler(ctx)
    }
}
