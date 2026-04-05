// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

import {IPercolate} from "./interfaces/IPercolate.sol";
import {Market} from "./Market.sol";
import {VAMM} from "./VAMM.sol";
import {RiskEngine} from "./RiskEngine.sol";
import {Vault} from "./Vault.sol";
import {MarginMath} from "./libraries/MarginMath.sol";

/// @title Percolate
/// @notice Multichain permissionless perpetual futures DEX.
/// EVM implementation of the same protocol that runs on Solana.
/// Same risk engine, same instructions, same SDK API.
contract Percolate is IPercolate {
    using Market for Market.State;
    using VAMM for VAMM.State;
    using MarginMath for *;

    // ─── Cross-margin User Account ───
    struct PositionRef {
        bytes32 marketId;
        int256 baseSize;
        uint256 quoteEntry;
        int256 lastFundingIndex;
        uint256 aSnapshot;
        int256 kSnapshot;
        uint64 epochSnapshot;
        uint256 reservedPnl;
        uint64 warmupStartedAt;
        uint32 leverage;
    }

    struct UserAccount {
        bool open;
        uint256 totalMarginUsed;
        int256 totalUnrealizedPnl;
        int256 totalRealizedPnl;
        int256 feeCredits;
        uint64 lastSettledAt;
        bytes32[] marketsTouched;
        mapping(bytes32 => PositionRef) positions;
    }

    // ─── Storage ───
    address public admin;
    bool public paused;
    uint64 public marketCount;

    Vault public immutable vault;
    RiskEngine public immutable riskEngine;

    mapping(bytes32 => Market.State) public markets;
    mapping(address => UserAccount) internal userAccounts;

    uint16 public constant CREATOR_FEE_SHARE_BPS = 800;
    uint16 public constant MIN_TRADING_FEE_BPS = 3;
    uint16 public constant MAX_TRADING_FEE_BPS = 100;
    uint32 public constant ADAPTIVE_K_WINDOW_SECONDS = 3600;

    error Paused();
    error NotAdmin();
    error MarketInactive();
    error LeverageOutOfBounds();
    error TradingFeeOutOfBounds();
    error AccountNotOpen();
    error AccountAlreadyOpen();
    error PositionNotFound();
    error InsufficientMargin();
    error SlippageExceeded();
    error NotLiquidatable();
    error FundingNotReady();

    modifier onlyAdmin() {
        if (msg.sender != admin) revert NotAdmin();
        _;
    }

    modifier whenNotPaused() {
        if (paused) revert Paused();
        _;
    }

    constructor(address _admin, address _vault, address _riskEngine) {
        admin = _admin;
        vault = Vault(_vault);
        riskEngine = RiskEngine(_riskEngine);
    }

    // ─── Admin ───

    function setPaused(bool _paused) external onlyAdmin {
        paused = _paused;
    }

    // ─── Permissionless Market Creation ───

    function createMarket(CreateMarketParams calldata params) external whenNotPaused returns (bytes32 marketId) {
        if (params.maxLeverage < 100 || params.maxLeverage > 2000) revert LeverageOutOfBounds();
        if (params.tradingFeeBps < MIN_TRADING_FEE_BPS || params.tradingFeeBps > MAX_TRADING_FEE_BPS) {
            revert TradingFeeOutOfBounds();
        }

        marketId = keccak256(abi.encodePacked(params.tokenMint, msg.sender));

        uint256 initialBase = sqrt(params.initialK);
        uint256 initialQuote = params.initialK / initialBase;

        markets[marketId] = Market.State({
            marketIndex: marketCount,
            tokenMint: params.tokenMint,
            creator: msg.sender,
            creatorFeeAccount: msg.sender,
            amm: VAMM.State({
                baseReserve: initialBase,
                quoteReserve: initialQuote,
                k: params.initialK,
                pegMultiplier: 1e6
            }),
            kTarget: params.initialK,
            kBase: params.initialK,
            kMin: params.kMin,
            kMax: params.kMax,
            kLastAdjusted: uint64(block.timestamp),
            totalLongPosition: 0,
            totalShortPosition: 0,
            volume24h: 0,
            volumeAvg7d: 0,
            volatilityScore: 0,
            maxLeverage: params.maxLeverage,
            tradingFeeBps: params.tradingFeeBps,
            liquidationFeeBps: 100,
            maintenanceMarginBps: params.maintenanceMarginBps,
            oracle: params.oracle,
            insuranceFundBalance: 0,
            lastFundingTime: uint64(block.timestamp),
            cumulativeLongFunding: 0,
            cumulativeShortFunding: 0,
            fundingPeriodSeconds: 3600,
            fundingRateCapBps: 10,
            creatorFeesEarned: 0,
            protocolFeesEarned: 0,
            totalVolume: 0,
            active: true,
            createdAt: uint64(block.timestamp)
        });

        marketCount++;
        emit MarketCreated(marketId, msg.sender, params.tokenMint);
    }

    // ─── User Account ───

    function openUserAccount() external whenNotPaused {
        UserAccount storage acc = userAccounts[msg.sender];
        if (acc.open) revert AccountAlreadyOpen();
        acc.open = true;
        acc.lastSettledAt = uint64(block.timestamp);
    }

    function deposit(address collateral, uint256 amount) external whenNotPaused {
        UserAccount storage acc = userAccounts[msg.sender];
        if (!acc.open) revert AccountNotOpen();
        vault.deposit(msg.sender, collateral, amount);
    }

    function withdraw(address collateral, uint256 amount) external {
        UserAccount storage acc = userAccounts[msg.sender];
        if (!acc.open) revert AccountNotOpen();
        // TODO: enforce initial margin maintained across all positions
        vault.withdraw(msg.sender, collateral, amount);
    }

    // ─── Trading ───

    function openPosition(OpenPositionParams calldata params) external whenNotPaused {
        Market.State storage m = markets[params.marketId];
        if (!m.active) revert MarketInactive();
        if (params.leverage < 100 || params.leverage > m.maxLeverage) revert LeverageOutOfBounds();

        UserAccount storage acc = userAccounts[msg.sender];
        if (!acc.open) revert AccountNotOpen();

        VAMM.SwapResult memory result;
        if (params.side == Side.Long) {
            result = VAMM.simulateBuy(m.amm, params.baseSize);
        } else {
            result = VAMM.simulateSell(m.amm, params.baseSize);
        }

        if (result.slippageBps > params.maxSlippageBps) revert SlippageExceeded();

        m.amm.baseReserve = result.newBaseReserve;
        m.amm.quoteReserve = result.newQuoteReserve;
        m.totalVolume += result.quoteDelta;
        m.volume24h += result.quoteDelta;

        if (params.side == Side.Long) {
            m.totalLongPosition += params.baseSize;
        } else {
            m.totalShortPosition += params.baseSize;
        }

        // Update position
        PositionRef storage pos = acc.positions[params.marketId];
        if (pos.marketId == bytes32(0)) {
            pos.marketId = params.marketId;
            acc.marketsTouched.push(params.marketId);
        }
        int256 signedSize = params.side == Side.Long ? int256(params.baseSize) : -int256(params.baseSize);
        pos.baseSize += signedSize;
        pos.quoteEntry += result.quoteDelta;
        pos.leverage = params.leverage;

        emit PositionOpened(msg.sender, params.marketId, params.side, params.baseSize);
    }

    function closePosition(bytes32 marketId, uint256 baseSize) external {
        Market.State storage m = markets[marketId];
        UserAccount storage acc = userAccounts[msg.sender];
        PositionRef storage pos = acc.positions[marketId];
        if (pos.baseSize == 0) revert PositionNotFound();

        bool isLong = pos.baseSize > 0;
        uint256 closeSize = baseSize == 0 ? uint256(isLong ? pos.baseSize : -pos.baseSize) : baseSize;

        VAMM.SwapResult memory result = isLong
            ? VAMM.simulateSell(m.amm, closeSize)
            : VAMM.simulateBuy(m.amm, closeSize);

        m.amm.baseReserve = result.newBaseReserve;
        m.amm.quoteReserve = result.newQuoteReserve;
        m.totalVolume += result.quoteDelta;

        int256 closeSizeI = int256(closeSize);
        if (isLong) {
            pos.baseSize -= closeSizeI;
            m.totalLongPosition -= closeSize;
        } else {
            pos.baseSize += closeSizeI;
            m.totalShortPosition -= closeSize;
        }

        if (pos.baseSize == 0) {
            pos.quoteEntry = 0;
        } else {
            pos.quoteEntry = pos.quoteEntry > result.quoteDelta ? pos.quoteEntry - result.quoteDelta : 0;
        }

        emit PositionClosed(msg.sender, marketId, 0);
    }

    // ─── Maintenance ───

    function liquidate(address user, bytes32 marketId) external {
        // Read user account, find worst position, close via vAMM
        // Charge liquidation fee, split 50/50 liquidator/insurance
        // Apply A/K via riskEngine if account ends in deficit
        emit Liquidated(user, marketId, msg.sender);
    }

    function crankFunding(bytes32 marketId) external {
        Market.State storage m = markets[marketId];
        if (block.timestamp < m.lastFundingTime + m.fundingPeriodSeconds) revert FundingNotReady();

        uint256 mark = VAMM.markPrice(m.amm);
        uint256 oraclePrice = mark; // placeholder

        int256 premiumBps = mark > oraclePrice
            ? int256(((mark - oraclePrice) * 10_000) / oraclePrice)
            : -int256(((oraclePrice - mark) * 10_000) / oraclePrice);

        int256 cap = int256(uint256(m.fundingRateCapBps));
        int256 rate = premiumBps > cap ? cap : (premiumBps < -cap ? -cap : premiumBps);

        m.cumulativeLongFunding -= rate;
        m.cumulativeShortFunding += rate;
        m.lastFundingTime = uint64(block.timestamp);

        emit FundingPaid(marketId, rate);
    }

    function updateAmm(bytes32 marketId) external {
        Market.State storage m = markets[marketId];
        uint256 mark = VAMM.markPrice(m.amm);
        uint256 oraclePrice = mark; // placeholder
        m.amm.pegMultiplier = VAMM.computeNewPeg(m.amm.pegMultiplier, oraclePrice, mark);
    }

    function adaptK(bytes32 marketId) external {
        Market.State storage m = markets[marketId];
        if (block.timestamp - m.kLastAdjusted < 60) return;

        uint256 oldK = m.amm.k;
        m.kTarget = Market.computeKTarget(
            m.kBase,
            m.kMin,
            m.kMax,
            m.volume24h,
            m.volumeAvg7d,
            m.volatilityScore
        );
        m.amm.k = Market.smoothK(
            m.amm.k,
            m.kTarget,
            uint32(block.timestamp - m.kLastAdjusted),
            ADAPTIVE_K_WINDOW_SECONDS
        );
        m.kLastAdjusted = uint64(block.timestamp);

        emit KAdapted(marketId, oldK, m.amm.k);
    }

    // ─── Helpers ───

    function sqrt(uint256 x) private pure returns (uint256 y) {
        uint256 z = (x + 1) / 2;
        y = x;
        while (z < y) {
            y = z;
            z = (x / z + z) / 2;
        }
    }
}
