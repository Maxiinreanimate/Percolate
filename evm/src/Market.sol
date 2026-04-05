// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

import {VAMM} from "./VAMM.sol";

/// @title Market
/// @notice Per-market storage struct and adaptive k logic.
library Market {
    struct State {
        // Identity
        uint64 marketIndex;
        address tokenMint;
        address creator;
        address creatorFeeAccount;

        // vAMM
        VAMM.State amm;
        uint256 kTarget;
        uint256 kBase;
        uint256 kMin;
        uint256 kMax;
        uint64 kLastAdjusted;
        uint256 totalLongPosition;
        uint256 totalShortPosition;

        // Adaptive k
        uint256 volume24h;
        uint256 volumeAvg7d;
        uint32 volatilityScore;

        // Params (immutable after creation)
        uint32 maxLeverage;
        uint16 tradingFeeBps;
        uint16 liquidationFeeBps;
        uint16 maintenanceMarginBps;

        // Oracle
        address oracle;

        // Risk state references (held in RiskEngine contract)
        uint256 insuranceFundBalance;

        // Funding
        uint64 lastFundingTime;
        int256 cumulativeLongFunding;
        int256 cumulativeShortFunding;
        uint32 fundingPeriodSeconds;
        uint16 fundingRateCapBps;

        // Stats
        uint256 creatorFeesEarned;
        uint256 protocolFeesEarned;
        uint256 totalVolume;

        bool active;
        uint64 createdAt;
    }

    /// @notice Compute the target k for adaptive liquidity.
    function computeKTarget(
        uint256 kBase,
        uint256 kMin,
        uint256 kMax,
        uint256 volume24h,
        uint256 volumeAvg7d,
        uint32 volatilityScore
    ) internal pure returns (uint256) {
        uint256 volumeFactorBps;
        if (volumeAvg7d == 0) {
            volumeFactorBps = 10_000;
        } else {
            uint256 raw = (volume24h * 10_000) / volumeAvg7d;
            volumeFactorBps = raw > 50_000 ? 50_000 : raw;
        }

        uint256 volatilityFactorBps = uint256(volatilityScore) * 2;
        if (volatilityFactorBps > 20_000) volatilityFactorBps = 20_000;

        uint256 combined = (10_000 + volumeFactorBps + volatilityFactorBps) / 3;
        uint256 raw = (kBase * combined) / 10_000;

        if (raw < kMin) return kMin;
        if (raw > kMax) return kMax;
        return raw;
    }

    /// @notice Smooth current k toward target over the smoothing window.
    function smoothK(
        uint256 currentK,
        uint256 targetK,
        uint32 elapsedSeconds,
        uint32 windowSeconds
    ) internal pure returns (uint256) {
        if (windowSeconds == 0 || elapsedSeconds == 0) return currentK;
        uint256 elapsed = elapsedSeconds > windowSeconds ? windowSeconds : elapsedSeconds;
        if (targetK > currentK) {
            uint256 diff = targetK - currentK;
            return currentK + (diff * elapsed) / windowSeconds;
        } else {
            uint256 diff = currentK - targetK;
            return currentK - (diff * elapsed) / windowSeconds;
        }
    }
}
