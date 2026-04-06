// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

import "forge-std/Test.sol";
import {VAMM} from "../src/VAMM.sol";

contract VAMMTest is Test {
    using VAMM for VAMM.State;

    function test_BuyDecreasesBaseReserve() public {
        VAMM.State memory s = VAMM.State({
            baseReserve: 1000e6,
            quoteReserve: 1000e6,
            k: 1e18,
            pegMultiplier: 150e6
        });
        VAMM.SwapResult memory r = VAMM.simulateBuy(s, 10e6);
        assertTrue(r.newBaseReserve < s.baseReserve);
        assertTrue(r.newQuoteReserve > s.quoteReserve);
    }

    function test_SellIncreasesBaseReserve() public {
        VAMM.State memory s = VAMM.State({
            baseReserve: 1000e6,
            quoteReserve: 1000e6,
            k: 1e18,
            pegMultiplier: 150e6
        });
        VAMM.SwapResult memory r = VAMM.simulateSell(s, 10e6);
        assertTrue(r.newBaseReserve > s.baseReserve);
        assertTrue(r.newQuoteReserve < s.quoteReserve);
    }

    function test_MarkPriceUsesPeg() public {
        VAMM.State memory s = VAMM.State({
            baseReserve: 1000e6,
            quoteReserve: 1000e6,
            k: 1e18,
            pegMultiplier: 150e6
        });
        uint256 mark = VAMM.markPrice(s);
        assertEq(mark, 150e6);
    }

    function testFuzz_BuyAlwaysHasSlippage(uint128 baseSize) public {
        vm.assume(baseSize > 0 && baseSize < 500e6);
        VAMM.State memory s = VAMM.State({
            baseReserve: 1000e6,
            quoteReserve: 1000e6,
            k: 1e18,
            pegMultiplier: 150e6
        });
        VAMM.SwapResult memory r = VAMM.simulateBuy(s, baseSize);
        assertTrue(r.slippageBps > 0 || baseSize == 0);
    }
}
