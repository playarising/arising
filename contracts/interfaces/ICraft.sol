// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;
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
     * @param name                  Name of the recipe.
     * @param description           Description of the recipe.
     * @param materials             Array of addresses of the require material instances.
     * @param amounts               Array of amounts for each required material.
     * @param stats_required        Amount of stats required to consume to create the recipe.
     * @param cooldown              Cooldown in seconds of the recipe.
     * @param level_required        Minimum level required to craft the recipe.
     * @param reward                ID of the reward token.
     * @param available             Boolean to check if the recipe is available.
     */
    struct Recipe {
        uint256 id;
        string name;
        string description;
        address[] materials;
        uint256[] material_amounts;
        IStats.BasicStats stats_required;
        uint256 cooldown;
        uint256 level_required;
        uint256 reward;
        uint256 experience_reward;
        bool available;
    }

    /**
     * @notice Internal struct to containt all the information of an upgrade.
     *
     * Requirements:
     * @param id                    ID of the upgrade.
     * @param name                  Name of the upgrade.
     * @param description           Description of the upgrade.
     * @param materials             Array of addresses of the require material instances.
     * @param amounts               Array of amounts for each required material.
     * @param stats_required        Amount of stats required to consume to create the upgrade.
     * @param stats_sacrificed      Amount of stats required to sacrifice to create the upgrade.
     * @param level_required        Minimum level required to craft the upgrade.
     * @param upgraded_item         ID of the item is beign upgraded.
     * @param reward                ID of the reward token.
     * @param available             Boolean to check if the upgrade is available.
     */
    struct Upgrade {
        uint256 id;
        string name;
        string description;
        address[] materials;
        uint256[] material_amounts;
        IStats.BasicStats stats_required;
        IStats.BasicStats stats_sacrificed;
        uint256 level_required;
        uint256 upgraded_item;
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

    /** @notice See [Craft#disableUpgrade](/docs/core/Craft.md#disableUpgrade) */
    function disableUpgrade(uint256 _upgrade_id) external;

    /** @notice See [Craft#enableUpgrade](/docs/core/Craft.md#enableUpgrade) */
    function enableUpgrade(uint256 _upgrade_id) external;

    /** @notice See [Craft#addRecipe](/docs/core/Craft.md#addRecipe) */
    function addRecipe(
        string memory _name,
        string memory _description,
        address[] memory _materials,
        uint256[] memory _amounts,
        IStats.BasicStats memory _stats,
        uint256 _cooldown,
        uint256 _level_required,
        uint256 _reward,
        uint256 _experience_reward
    ) external;

    /** @notice See [Craft#updateRecipe](/docs/core/Craft.md#updateRecipe) */
    function updateRecipe(Recipe memory _recipe) external;

    /** @notice See [Craft#addUpgrade](/docs/core/Craft.md#addUpgrade) */
    function addUpgrade(
        string memory _name,
        string memory _description,
        address[] memory _materials,
        uint256[] memory _amounts,
        IStats.BasicStats memory _stats,
        IStats.BasicStats memory _sacrifice,
        uint256 _level_required,
        uint256 _upgraded_item,
        uint256 _reward
    ) external;

    /** @notice See [Craft#updateUpgrade](/docs/core/Craft.md#updateUpgrade) */
    function updateUpgrade(Upgrade memory _upgrade) external;

    /** @notice See [Craft#craft](/docs/core/Craft.md#craft) */
    function craft(bytes memory _id, uint256 _recipe_id) external;

    /** @notice See [Craft#claim](/docs/core/Craft.md#claim) */
    function claim(bytes memory _id) external;

    /** @notice See [Craft#upgrade](/docs/core/Craft.md#upgrade) */
    function upgrade(bytes memory _id, uint256 _upgrade_id) external;

    /** @notice See [Craft#getRecipe](/docs/core/Craft.md#getRecipe) */
    function getRecipe(
        uint256 _recipe_id
    ) external view returns (Recipe memory _recipe);

    /** @notice See [Craft#getUpgrade](/docs/core/Craft.md#getUpgrade) */
    function getUpgrade(
        uint256 _upgrade_id
    ) external view returns (Upgrade memory _upgrade);

    /** @notice See [Craft#getCharacterCrafSlot](/docs/core/Craft.md#getCharacterCrafSlot) */
    function getCharacterCrafSlot(
        bytes memory _id
    ) external view returns (Slot memory _slot);
}
