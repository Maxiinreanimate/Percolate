// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

interface IOracle {
    struct Price {
        uint256 value;
        uint256 confidence;
        uint64 lastUpdated;
    }

    function getPrice(address token) external view returns (Price memory);
    function isFresh(address token) external view returns (bool);
}
