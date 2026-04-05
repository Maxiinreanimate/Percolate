// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

/// @title MarginMath
/// @notice Cross-margin equity and margin requirement calculations.
library MarginMath {
    uint256 internal constant BPS = 10_000;
    uint256 internal constant LEVERAGE_SCALE = 100;

    function initialMargin(uint256 notional, uint32 leverage) internal pure returns (uint256) {
        if (leverage == 0) return notional;
        return (notional * LEVERAGE_SCALE) / uint256(leverage);
    }

    function maintenanceMargin(uint256 notional, uint16 mmBps) internal pure returns (uint256) {
        return (notional * uint256(mmBps)) / BPS;
    }

    function isHealthy(int256 equity, uint256 mm) internal pure returns (bool) {
        if (equity < 0) return false;
        return uint256(equity) >= mm;
    }

    function positionNotional(int256 baseSize, uint256 markPrice) internal pure returns (uint256) {
        if (baseSize == 0) return 0;
        uint256 absBase = baseSize < 0 ? uint256(-baseSize) : uint256(baseSize);
        return (absBase * markPrice) / 1e6;
    }

    function unrealizedPnl(
        int256 baseSize,
        uint256 quoteEntry,
        uint256 markPrice
    ) internal pure returns (int256) {
        if (baseSize == 0) return 0;
        uint256 absBase = baseSize < 0 ? uint256(-baseSize) : uint256(baseSize);
        uint256 currentQuote = (absBase * markPrice) / 1e6;
        if (baseSize > 0) {
            return int256(currentQuote) - int256(quoteEntry);
        } else {
            return int256(quoteEntry) - int256(currentQuote);
        }
    }
}
