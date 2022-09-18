// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;
import "../interfaces/IStats.sol";

/**
 * @title ICraft
 * @notice Interface for the [Craft](/docs/core/Craft.md) contract.
 */
interface ICraft {
    /**
     * @notice Internal struct to containt all the information of a recipe.
     *
     * Requirements:
     * @param id                    ID of the recipe.
     * @param materials             Array of addresses of the require material instances.
     * @param amounts               Array of amounts for each required material.
     * @param stats_required        Amount of stats required to consume to create the recipe.
     * @param cooldown              Cooldown in seconds of the recipe.
     * @param level_required        Minimum level required to craft the recipe.
     * @param gold_cost             Cost of the recipe in gold.
     * @param reward                ID of the reward token.
     * @param available             Boolean to check if the recipe is available.
     */
    struct Recipe {
        uint256 id;
        address[] materials;
        uint256[] material_amounts;
        IStats.BasicStats stats_required;
        uint256 cooldown;
        uint256 level_required;
        uint256 gold_cost;
        uint256 reward;
        uint256 experience_reward;
        bool available;
    }

    /**
     * @notice Internal struct to containt all the information of an upgrade.
     *
     * Requirements:
     * @param id                    ID of the upgrade.
     * @param materials             Array of addresses of the require material instances.
     * @param amounts               Array of amounts for each required material.
     * @param stats_required        Amount of stats required to consume to create the upgrade.
     * @param stats_sacrificed      Amount of stats required to sacrifice to create the upgrade.
     * @param level_required        Minimum level required to craft the upgrade.
     * @param upgraded_item         ID of the item is beign upgraded.
     * @param gold_cost             Cost of the upgrade in gold.
     * @param reward                ID of the reward token.
     * @param available             Boolean to check if the upgrade is available.
     */
    struct Upgrade {
        uint256 id;
        address[] materials;
        uint256[] material_amounts;
        IStats.BasicStats stats_required;
        IStats.BasicStats stats_sacrificed;
        uint256 level_required;
        uint256 upgraded_item;
        uint256 gold_cost;
        uint256 reward;
        bool available;
    }

    /**
     * @notice Internal struct to store the information of a crafting or upgrading slot.
     *
     * Requirements:
     * @param cooldown      Timestamp on which the slot is claimable.
     * @param last_recipe   The last crafted recipe.
     * @param claimed       Boolean to know if the last crafted recipe is already claimed.
     */
    struct Slot {
        uint256 cooldown;
        uint256 last_recipe;
        bool claimed;
    }

    /** @notice See [Craft#pause](/docs/core/Craft.md#pause) */
    function pause() external;

    /** @notice See [Craft#unpause](/docs/core/Craft.md#unpause) */
    function unpause() external;

    /** @notice See [Craft#disableRecipe](/docs/core/Craft.md#disableRecipe) */
    function disableRecipe(uint256 _recipe_id) external;

    /** @notice See [Craft#enableRecipe](/docs/core/Craft.md#enableRecipe) */
    function enableRecipe(uint256 _recipe_id) external;

    /** @notice See [Craft#addRecipe](/docs/core/Craft.md#addRecipe) */
    function addRecipe(
        address[] memory _materials,
        uint256[] memory _amounts,
        IStats.BasicStats memory _stats,
        uint256 _cooldown,
        uint256 _level_required,
        uint256 _gold_cost,
        uint256 _reward,
        uint256 _experience_reward
    ) external;

    /** @notice See [Craft#craft](/docs/core/Craft.md#craft) */
    function craft(bytes memory _id, uint256 _recipe_id) external;

    /** @notice See [Craft#claim](/docs/core/Craft.md#claim) */
    function claim(bytes memory _id) external;

    /** @notice See [Craft#getRecipe](/docs/core/Craft.md#getRecipe) */
    function getRecipe(uint256 _recipe_id)
        external
        view
        returns (Recipe memory _recipe);

    /** @notice See [Craft#getCharacterCrafSlot](/docs/core/Craft.md#getCharacterCrafSlot) */
    function getCharacterCrafSlot(bytes memory _id)
        external
        view
        returns (Slot memory _slot);
}
