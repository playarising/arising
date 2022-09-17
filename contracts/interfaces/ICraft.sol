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
     * @param material_amounts      Array of amounts for each required material.
     * @param stats_required        Amount of stats required to consume to create the recipe.
     * @param cooldown              Cooldown in seconds of the recipe.
     * @param level_required        Minimum level required to craft the recipe.
     * @param experience_reward     Amount of experience rewarded from the craft.
     * @param cost                  Cost of the craft in gold.
     * @param available             Boolean to check if the recipe is available.
     * @param item_reward           Address of the craft resulting item.
     */
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

    /**
     * @notice Internal struct to store the information of a crafting slot.
     *
     * Requirements:
     * @param cooldown      Timestamp on which the slot is claimable.
     * @param last_recipe   The last crafted recipe.
     * @param claimed       Boolean to know if the last crafted recipe is already claimed.
     */
    struct CraftSlot {
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

    /** @notice See [Craft#craft](/docs/core/Craft.md#craft) */
    function craft(bytes memory _id, uint256 _recipe_id) external;

    /** @notice See [Craft#claim](/docs/core/Craft.md#claim) */
    function claim(bytes memory _id) external;

    /** @notice See [Craft#getRecipe](/docs/core/Craft.md#getRecipe) */
    function getRecipe(uint256 _recipe_id)
        external
        view
        returns (Recipe memory _recipe);

    /** @notice See [Craft#getCharacterSlot](/docs/core/Craft.md#getCharacterSlot) */
    function getCharacterSlot(bytes memory _id)
        external
        view
        returns (CraftSlot memory _slot);
}
