// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

import "forge-std/Script.sol";
import {Percolate} from "../src/Percolate.sol";
import {Vault} from "../src/Vault.sol";
import {RiskEngine} from "../src/RiskEngine.sol";
import {Oracle} from "../src/Oracle.sol";

contract DeployScript is Script {
    function run() external {
        vm.startBroadcast();

        address admin = msg.sender;
        address cranker = msg.sender;

        RiskEngine riskEngine = new RiskEngine();
        Oracle oracle = new Oracle(cranker);

        // Vault and Percolate need to know about each other.
        // Two-step deploy: deploy Percolate first with placeholder vault,
        // then deploy real Vault with Percolate address, then update.
        // For simplicity here we deploy Vault with admin and trust admin to wire it.
        Vault vault = new Vault(address(0), admin);
        Percolate percolate = new Percolate(admin, address(vault), address(riskEngine));

        console.log("RiskEngine:", address(riskEngine));
        console.log("Oracle:", address(oracle));
        console.log("Vault:", address(vault));
        console.log("Percolate:", address(percolate));

        vm.stopBroadcast();
    }
}
