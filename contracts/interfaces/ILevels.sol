// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

/**
 * @title ILevels
 * @notice Interface for the [Levels](/docs/codex/Levels.md) contract.
 */
interface ILevels {
    /**
     * @notice Internal struct to define the level ranges.
     *
     * Requirements:
     * @param min   The minimum amount of experience to achieve the level.
     * @param max   The maximum amount of experience for this level (non inclusive).
     */
    struct Level {
        uint256 min;
        uint256 max;
    }

    /** @notice See [Levels#getLevel](/docs/codex/Levels.md#getLevel) */
    function getLevel(uint256 _experience) external view returns (uint256);

    /** @notice See [Levels#getExperience](/docs/codex/Levels.md#getExperience) */
    function getExperience(uint256 _level) external view returns (uint256);
}
