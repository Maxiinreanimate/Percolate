// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.23;

interface IMarket {
    function getMarkPrice(bytes32 marketId) external view returns (uint256);
    function getOpenInterest(bytes32 marketId) external view returns (uint256 longOI, uint256 shortOI);
    function getFundingRate(bytes32 marketId) external view returns (int256);
    function getCreator(bytes32 marketId) external view returns (address);
}
