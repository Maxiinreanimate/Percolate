// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

/// @title LazyIndices
/// @notice Implements A/K from Percolator. Lazy side indices that handle
/// overhang clearing without an ADL queue.
library LazyIndices {
    uint256 internal constant POS_SCALE = 1e6;

    /// @notice effective_pos(i) = floor(basis_i * A / a_snapshot_i)
    function effectivePosition(
        int256 basis,
        uint256 currentA,
        uint256 aSnapshot
    ) internal pure returns (int256) {
        if (aSnapshot == 0) return 0;
        uint256 absBasis = basis < 0 ? uint256(-basis) : uint256(basis);
        uint256 scaled = (absBasis * currentA) / aSnapshot;
        return basis < 0 ? -int256(scaled) : int256(scaled);
    }

    /// @notice pnl_delta(i) = floor(|basis_i| * (K - k_snapshot_i) / (a_snapshot_i * POS_SCALE))
    function lazyPnlDelta(
        int256 basis,
        int256 currentK,
        int256 kSnapshot,
        uint256 aSnapshot
    ) internal pure returns (int256) {
        if (aSnapshot == 0) return 0;
        uint256 absBasis = basis < 0 ? uint256(-basis) : uint256(basis);
        int256 kDiff = currentK - kSnapshot;
        uint256 denom = aSnapshot * POS_SCALE;
        if (denom == 0) return 0;
        uint256 absDiff = kDiff < 0 ? uint256(-kDiff) : uint256(kDiff);
        uint256 scaled = (absBasis * absDiff) / denom;
        return kDiff < 0 ? -int256(scaled) : int256(scaled);
    }

    /// @notice Reduce the side A coefficient when liquidation removes OI.
    /// new_A = A * (oi - removed) / oi
    function reduceA(uint256 currentA, uint256 oi, uint256 removed) internal pure returns (uint256) {
        if (oi == 0) return currentA;
        uint256 remaining = oi > removed ? oi - removed : 0;
        return (currentA * remaining) / oi;
    }

    /// @notice Shift K to socialize a deficit across the side.
    /// new_K = K - (deficit / oi)
    function socializeDeficit(int256 currentK, uint256 deficit, uint256 oi) internal pure returns (int256) {
        if (oi == 0) return currentK;
        return currentK - int256(deficit / oi);
    }
}
