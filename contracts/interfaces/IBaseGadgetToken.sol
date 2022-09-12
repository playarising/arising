// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface IBaseGadgetToken {
    function mint(uint256 amount) external;

    function withdraw() external;

    function totalCost(uint256 amount) external returns (uint256);
}
