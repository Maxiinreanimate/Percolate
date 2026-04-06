// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

import "forge-std/Test.sol";
import {HaircutMath} from "../src/libraries/HaircutMath.sol";
import {LazyIndices} from "../src/libraries/LazyIndices.sol";

contract RiskEngineTest is Test {
    function test_HaircutFullyBacked() public {
        (uint256 num, uint256 denom) = HaircutMath.computeHaircut(
            2000e6, // vault
            1000e6, // capital
            500e6,  // insurance
            500e6   // matured profit
        );
        // residual = 2000 - 1000 - 500 = 500
        // h = min(500, 500) / 500 = 1
        assertEq(num, 500e6);
        assertEq(denom, 500e6);
    }

    function test_HaircutStressed() public {
        (uint256 num, uint256 denom) = HaircutMath.computeHaircut(
            1500e6, // vault
            1000e6, // capital
            200e6,  // insurance
            500e6   // matured profit
        );
        // residual = 1500 - 1000 - 200 = 300
        // h = min(300, 500) / 500 = 0.6
        assertEq(num, 300e6);
        assertEq(denom, 500e6);
    }

    function test_LazyEffectivePosition() public {
        int256 effective = LazyIndices.effectivePosition(
            int256(100e6),  // basis
            500_000,        // current A (after liquidation)
            1_000_000       // a snapshot
        );
        // effective = 100 * 500000 / 1000000 = 50
        assertEq(effective, int256(50e6));
    }

    function test_ReduceA() public {
        uint256 newA = LazyIndices.reduceA(1_000_000, 1000, 100);
        // new_A = 1000000 * 900 / 1000 = 900000
        assertEq(newA, 900_000);
    }
}
