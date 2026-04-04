// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

interface IPercolate {
    enum Side { Long, Short }
    enum SideState { Normal, DrainOnly, ResetPending }
    enum TriggerOrderType { Limit, StopLoss, TakeProfit }

    struct CreateMarketParams {
        address tokenMint;
        address oracle;
        uint32 maxLeverage;
        uint16 tradingFeeBps;
        uint256 initialK;
        uint256 kMin;
        uint256 kMax;
        uint16 maintenanceMarginBps;
    }

    struct OpenPositionParams {
        bytes32 marketId;
        Side side;
        uint256 baseSize;
        uint32 leverage;
        uint16 maxSlippageBps;
    }

    event MarketCreated(bytes32 indexed marketId, address indexed creator, address tokenMint);
    event PositionOpened(address indexed user, bytes32 indexed marketId, Side side, uint256 baseSize);
    event PositionClosed(address indexed user, bytes32 indexed marketId, int256 pnl);
    event Liquidated(address indexed user, bytes32 indexed marketId, address liquidator);
    event FundingPaid(bytes32 indexed marketId, int256 rate);
    event KAdapted(bytes32 indexed marketId, uint256 oldK, uint256 newK);

    function createMarket(CreateMarketParams calldata params) external returns (bytes32 marketId);
    function openUserAccount() external;
    function deposit(address collateral, uint256 amount) external;
    function withdraw(address collateral, uint256 amount) external;
    function openPosition(OpenPositionParams calldata params) external;
    function closePosition(bytes32 marketId, uint256 baseSize) external;
    function liquidate(address user, bytes32 marketId) external;
    function crankFunding(bytes32 marketId) external;
    function updateAmm(bytes32 marketId) external;
    function adaptK(bytes32 marketId) external;
}
