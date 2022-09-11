// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

interface ILevels {
    function getLevel(uint256 exp) external view returns (uint256);

    function getExperience(uint256 level) external view returns (uint256);
}
