// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

import {HaircutMath} from "./libraries/HaircutMath.sol";
import {LazyIndices} from "./libraries/LazyIndices.sol";

/// @title RiskEngine
/// @notice Percolator H + A/K risk engine for the EVM deployment.
contract RiskEngine {
    using HaircutMath for uint256;
    using LazyIndices for int256;

    struct SideRiskState {
        uint256 a;            // Lazy A coefficient
        int256 kIndex;        // Lazy K index
        uint64 epoch;
        uint8 state;          // 0 = Normal, 1 = DrainOnly, 2 = ResetPending
    }

    struct MarketRiskState {
        SideRiskState long;
        SideRiskState shortSide;
        uint256 haircutNumerator;
        uint256 haircutDenominator;
        uint256 insuranceFundBalance;
        uint256 vaultBalance;
        uint256 totalCapital;
        uint256 maturedProfitTotal;
    }

    /// @notice Recompute the haircut for a market.
    function refreshHaircut(MarketRiskState storage m) public {
        (uint256 num, uint256 denom) = HaircutMath.computeHaircut(
            m.vaultBalance,
            m.totalCapital,
            m.insuranceFundBalance,
            m.maturedProfitTotal
        );
        m.haircutNumerator = num;
        m.haircutDenominator = denom;
    }

    /// @notice Apply a liquidation: reduce A on the affected side and
    /// optionally socialize the deficit through K.
    function applyLiquidation(
        MarketRiskState storage m,
        bool isLong,
        uint256 oi,
        uint256 removedSize,
        uint256 deficit
    ) external {
        SideRiskState storage side = isLong ? m.long : m.shortSide;
        side.a = LazyIndices.reduceA(side.a, oi, removedSize);
        if (deficit > 0) {
            side.kIndex = LazyIndices.socializeDeficit(side.kIndex, deficit, oi);
        }
        // Check if A dropped below threshold → DrainOnly
        if (side.a < 1000) {
            side.state = 1;
        }
    }

    /// @notice Compute effective position size for a single account.
    function effectivePosition(
        int256 basis,
        uint256 currentA,
        uint256 aSnapshot
    ) external pure returns (int256) {
        return LazyIndices.effectivePosition(basis, currentA, aSnapshot);
    }

    /// @notice Compute lazy PnL delta for a single account.
    function lazyPnlDelta(
        int256 basis,
        int256 currentK,
        int256 kSnapshot,
        uint256 aSnapshot
    ) external pure returns (int256) {
        return LazyIndices.lazyPnlDelta(basis, currentK, kSnapshot, aSnapshot);
    }
}
