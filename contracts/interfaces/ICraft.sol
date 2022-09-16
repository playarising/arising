// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;
import "../interfaces/IStats.sol";

/**
 * @title ICraft
 * @notice Interface for the {Craft} contract.
 */
interface ICraft {
    struct Recipe {
        uint256 id;
        address[] materials;
        uint256[] material_amounts;
        IStats.BasicStats stats_required;
        uint256 cooldown;
        uint256 level_required;
        uint256 experience_reward;
        uint256 cost;
        bool available;
        uint256 item_reward;
    }

    struct CraftSlot {
        uint256 cooldown;
        uint256 last_recipe;
        bool claimed;
    }
}
