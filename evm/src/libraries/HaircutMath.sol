// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

/// @title HaircutMath
/// @notice Implements the H (haircut ratio) from Percolator.
/// Capital is senior. Profit is junior. A single global ratio determines
/// how much profit is real.
library HaircutMath {
    /// @notice Compute the haircut ratio as (numerator, denominator).
    /// H = min(Residual, ProfitTotal) / ProfitTotal
    /// Residual = max(0, vault - capital - insurance)
    function computeHaircut(
        uint256 vaultBalance,
        uint256 totalCapital,
        uint256 insurance,
        uint256 maturedProfitTotal
    ) internal pure returns (uint256 num, uint256 denom) {
        if (maturedProfitTotal == 0) return (1, 1);
        uint256 buffer = totalCapital + insurance;
        uint256 residual = vaultBalance > buffer ? vaultBalance - buffer : 0;
        num = residual < maturedProfitTotal ? residual : maturedProfitTotal;
        denom = maturedProfitTotal;
    }

    /// @notice Apply the haircut to released profit.
    /// effective = floor(released * num / denom)
    function applyHaircut(
        uint256 releasedPnl,
        uint256 num,
        uint256 denom
    ) internal pure returns (uint256) {
        if (denom == 0) return 0;
        return (releasedPnl * num) / denom;
    }
}
