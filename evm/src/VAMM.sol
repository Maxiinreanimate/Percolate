// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

/// @title VAMM
/// @notice Virtual constant-product AMM. Same model as the Solana program.
library VAMM {
    struct State {
        uint256 baseReserve;
        uint256 quoteReserve;
        uint256 k;
        uint256 pegMultiplier;
    }

    struct SwapResult {
        uint256 newBaseReserve;
        uint256 newQuoteReserve;
        uint256 quoteDelta;
        uint256 effectivePrice;
        uint16 slippageBps;
    }

    error InvalidSize();
    error PriceUnderflow();

    function simulateBuy(
        State memory s,
        uint256 baseSize
    ) internal pure returns (SwapResult memory r) {
        if (baseSize == 0 || baseSize >= s.baseReserve) revert InvalidSize();
        r.newBaseReserve = s.baseReserve - baseSize;
        r.newQuoteReserve = s.k / r.newBaseReserve;
        r.quoteDelta = r.newQuoteReserve - s.quoteReserve;
        r.effectivePrice = (r.quoteDelta * s.pegMultiplier) / baseSize;
        r.slippageBps = computeSlippage(s, r.effectivePrice);
    }

    function simulateSell(
        State memory s,
        uint256 baseSize
    ) internal pure returns (SwapResult memory r) {
        if (baseSize == 0) revert InvalidSize();
        r.newBaseReserve = s.baseReserve + baseSize;
        r.newQuoteReserve = s.k / r.newBaseReserve;
        if (s.quoteReserve < r.newQuoteReserve) revert PriceUnderflow();
        r.quoteDelta = s.quoteReserve - r.newQuoteReserve;
        r.effectivePrice = (r.quoteDelta * s.pegMultiplier) / baseSize;
        r.slippageBps = computeSlippage(s, r.effectivePrice);
    }

    function markPrice(State memory s) internal pure returns (uint256) {
        if (s.baseReserve == 0) return 0;
        return (s.quoteReserve * s.pegMultiplier) / s.baseReserve;
    }

    function computeNewPeg(
        uint256 currentPeg,
        uint256 oraclePrice,
        uint256 mark
    ) internal pure returns (uint256) {
        if (mark == 0) return currentPeg;
        return (currentPeg * oraclePrice) / mark;
    }

    function computeSlippage(State memory s, uint256 effective) private pure returns (uint16) {
        uint256 oracle = (s.quoteReserve * s.pegMultiplier) / s.baseReserve;
        if (oracle == 0) return 0;
        uint256 diff = effective > oracle ? effective - oracle : oracle - effective;
        uint256 bps = (diff * 10_000) / oracle;
        return bps > type(uint16).max ? type(uint16).max : uint16(bps);
    }
}
