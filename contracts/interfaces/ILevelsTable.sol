// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

interface ILevelsTable {
    function getLevel(uint256 exp) external view returns (uint256);
}
