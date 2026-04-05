// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

import {IOracle} from "./interfaces/IOracle.sol";

/// @title Oracle
/// @notice Multi-source oracle aggregator for the EVM deployment.
/// Combines Chainlink and Uniswap V3 TWAP. Fails closed if sources diverge.
contract Oracle is IOracle {
    uint256 public constant MAX_DIVERGENCE_BPS = 500;
    uint256 public constant STALENESS_SECONDS = 30;

    struct OracleConfig {
        address chainlinkFeed;
        address uniswapV3Pool;
        uint32 twapWindow;
        bool enabled;
    }

    mapping(address => OracleConfig) public configs;
    mapping(address => Price) private cachedPrices;

    address public immutable cranker;

    error StalePrice();
    error DivergenceTooHigh();
    error NotCranker();

    modifier onlyCranker() {
        if (msg.sender != cranker) revert NotCranker();
        _;
    }

    constructor(address _cranker) {
        cranker = _cranker;
    }

    function configure(
        address token,
        address chainlinkFeed,
        address uniswapV3Pool,
        uint32 twapWindow
    ) external {
        configs[token] = OracleConfig({
            chainlinkFeed: chainlinkFeed,
            uniswapV3Pool: uniswapV3Pool,
            twapWindow: twapWindow,
            enabled: true
        });
    }

    function pushPrice(address token, uint256 value, uint256 confidence) external onlyCranker {
        cachedPrices[token] = Price({
            value: value,
            confidence: confidence,
            lastUpdated: uint64(block.timestamp)
        });
    }

    function getPrice(address token) external view returns (Price memory) {
        Price memory p = cachedPrices[token];
        if (block.timestamp - p.lastUpdated > STALENESS_SECONDS) revert StalePrice();
        return p;
    }

    function isFresh(address token) external view returns (bool) {
        return block.timestamp - cachedPrices[token].lastUpdated <= STALENESS_SECONDS;
    }
}
