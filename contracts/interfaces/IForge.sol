// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "../interfaces/IStats.sol";

/**
 * @title IForge
 * @notice Interface for the [Forge](/docs/core/Forge.md) contract.
 */
interface IForge {
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
     * @param level_required        Minimum level required to forge the recipe.
     * @param reward                Address of the resulting item of the recipe.
     * @param experience_reward     Amount of experience rewarded from the recipe.
     * @param available             Boolean to check if the recipe is available.
     */
    struct Recipe {
        uint256 id;
        string name;
        string description;
        address[] materials;
        uint256[] amounts;
        IStats.BasicStats stats_required;
        uint256 cooldown;
        uint256 level_required;
        address reward;
        uint256 experience_reward;
        bool available;
    }

    struct Forge {
        bool available;
        uint256 cooldown;
        uint256 last_recipe;
        bool last_recipe_claimed;
    }

    /** @notice See [Forge#pause](/docs/core/Forge.md#pause) */
    function pause() external;

    /** @notice See [Forge#unpause](/docs/core/Forge.md#unpause) */
    function unpause() external;

    /** @notice See [Forge#disableRecipe](/docs/core/Forge.md#disableRecipe) */
    function disableRecipe(uint256 _recipe_id) external;

    /** @notice See [Forge#enableRecipe](/docs/core/Forge.md#enableRecipe) */
    function enableRecipe(uint256 _recipe_id) external;

    /** @notice See [Forge#addRecipe](/docs/core/Forge.md#addRecipe) */
    function addRecipe(
        string memory _name,
        string memory _description,
        address[] memory _materials,
        uint256[] memory _amounts,
        IStats.BasicStats memory _stats,
        uint256 _cooldown,
        uint256 _level_required,
        address _reward,
        uint256 _experience_reward
    ) external;

    /** @notice See [Forge#updateRecipe](/docs/core/Forge.md#updateRecipe) */
    function updateRecipe(Recipe memory _recipe) external;

    /** @notice See [Forge#buyUpgrade](/docs/core/Forge.md#buyUpgrade) */
    function buyUpgrade(bytes memory _id) external;

    /** @notice See [Forge#forge](/docs/core/Forge.md#forge) */
    function forge(
        bytes memory _id,
        uint256 _recipe_id,
        uint256 _forge_id
    ) external;

    /** @notice See [Forge#claim](/docs/core/Forge.md#claim) */
    function claim(bytes memory _id, uint256 _forge_id) external;

    /** @notice See [Forge#getRecipe](/docs/core/Forge.md#getRecipe) */
    function getRecipe(
        uint256 _recipe_id
    ) external view returns (Recipe memory _recipe);

    /** @notice See [Forge#getCharacterForge](/docs/core/Forge.md#getCharacterForge) */
    function getCharacterForge(
        bytes memory _id,
        uint256 _forge_id
    ) external view returns (Forge memory _forge);

    /** @notice See [Forge#getCharacterForgesUpgrades](/docs/core/Forge.md#getCharacterForgesUpgrades) */
    function getCharacterForgesUpgrades(
        bytes memory _id
    ) external view returns (bool[3] memory _upgrades);

    /** @notice See [Forge#getCharacterForgesAvailability](/docs/core/Forge.md#getCharacterForgesAvailability) */
    function getCharacterForgesAvailability(
        bytes memory _id
    ) external view returns (bool[3] memory _availability);
}
