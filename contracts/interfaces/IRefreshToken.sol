// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

interface IRefreshToken {
    function mint(uint256 amount) external;

    function withdraw() external;
}
