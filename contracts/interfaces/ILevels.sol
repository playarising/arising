// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title ILevels
 * @notice Interface for the {Levels} contract.
 */
interface ILevels {
    /** @notice See {Levels.getLevel} */
    function getLevel(uint256 _experience) external view returns (uint256);

    /** @notice See {Levels.getExperience} */
    function getExperience(uint256 _level) external view returns (uint256);
}
