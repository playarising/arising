// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface IRefresher {
    function mint(uint256 amount) external;

    function withdraw() external;
}
