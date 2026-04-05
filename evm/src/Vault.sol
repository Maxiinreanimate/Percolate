// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

interface IERC20 {
    function transferFrom(address, address, uint256) external returns (bool);
    function transfer(address, uint256) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

/// @title Vault
/// @notice Multi-collateral vault for the EVM deployment.
/// Holds USDC, USDT, SOL (wrapped), ETH, wBTC, etc. with per-asset
/// haircuts for cross-margin equity calculation.
contract Vault {
    struct CollateralConfig {
        uint16 staticHaircutBps;
        uint16 dynamicHaircutBps;
        address priceOracle;
        bool enabled;
    }

    mapping(address => CollateralConfig) public collateralConfigs;
    mapping(address => uint256) public totalDeposited;
    mapping(address => mapping(address => uint256)) public balances; // user => collateral => amount

    address public immutable percolate;
    address public immutable admin;

    error NotPercolate();
    error CollateralDisabled();
    error InsufficientBalance();
    error TransferFailed();

    modifier onlyPercolate() {
        if (msg.sender != percolate) revert NotPercolate();
        _;
    }

    constructor(address _percolate, address _admin) {
        percolate = _percolate;
        admin = _admin;
    }

    function registerCollateral(
        address token,
        uint16 staticHaircutBps,
        address priceOracle
    ) external {
        require(msg.sender == admin, "not admin");
        collateralConfigs[token] = CollateralConfig({
            staticHaircutBps: staticHaircutBps,
            dynamicHaircutBps: 0,
            priceOracle: priceOracle,
            enabled: true
        });
    }

    function deposit(address user, address token, uint256 amount) external onlyPercolate {
        if (!collateralConfigs[token].enabled) revert CollateralDisabled();
        if (!IERC20(token).transferFrom(user, address(this), amount)) revert TransferFailed();
        balances[user][token] += amount;
        totalDeposited[token] += amount;
    }

    function withdraw(address user, address token, uint256 amount) external onlyPercolate {
        if (balances[user][token] < amount) revert InsufficientBalance();
        balances[user][token] -= amount;
        totalDeposited[token] -= amount;
        if (!IERC20(token).transfer(user, amount)) revert TransferFailed();
    }

    function effectiveValue(
        address user,
        address token,
        uint256 price
    ) external view returns (uint256) {
        CollateralConfig memory cfg = collateralConfigs[token];
        uint256 raw = (balances[user][token] * price) / 1e18;
        uint256 totalHaircut = uint256(cfg.staticHaircutBps + cfg.dynamicHaircutBps);
        return (raw * (10_000 - totalHaircut)) / 10_000;
    }
}
