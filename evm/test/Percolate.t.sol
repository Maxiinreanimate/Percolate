// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

import "forge-std/Test.sol";
import {Percolate} from "../src/Percolate.sol";
import {Vault} from "../src/Vault.sol";
import {RiskEngine} from "../src/RiskEngine.sol";
import {IPercolate} from "../src/interfaces/IPercolate.sol";

contract PercolateTest is Test {
    Percolate percolate;
    Vault vault;
    RiskEngine riskEngine;
    address admin = address(0xA);

    function setUp() public {
        riskEngine = new RiskEngine();
        // Deploy vault first with placeholder, then percolate, then update
        // For test simplicity we accept the constructor ordering
        vault = new Vault(address(this), admin);
        percolate = new Percolate(admin, address(vault), address(riskEngine));
    }

    function test_CreateMarket() public {
        IPercolate.CreateMarketParams memory params = IPercolate.CreateMarketParams({
            tokenMint: address(0xBEEF),
            oracle: address(0xC0DE),
            maxLeverage: 1000,
            tradingFeeBps: 10,
            initialK: 1_000_000_000,
            kMin: 100_000_000,
            kMax: 10_000_000_000,
            maintenanceMarginBps: 500
        });
        bytes32 id = percolate.createMarket(params);
        assertTrue(id != bytes32(0));
    }

    function test_AdminCanPause() public {
        vm.prank(admin);
        percolate.setPaused(true);
        assertTrue(percolate.paused());
    }

    function testFail_NonAdminCannotPause() public {
        percolate.setPaused(true);
    }
}
