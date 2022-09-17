// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

import "../interfaces/ICraft.sol";
import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/IStats.sol";
import "../interfaces/IItems.sol";
import "../interfaces/IBaseFungibleItem.sol";

/**
 * @title Craft
 * @notice This contract is used to store and craft recipes through the ecosystem. This is the only contract able to mint
 * items through the [Items](/docs/items/Items.md) `ERC1155` implementation.
 *
 * @notice Implementation of the [ICraft](/docs/interfaces/ICraft.md) interface.
 */
contract Craft is ICraft, Ownable, Pausable {
    // =============================================== Storage ========================================================

    /** @notice Map to track available recipes for craft. */
    mapping(uint256 => Recipe) public recipes;

    /** @notice Array to track all the recipes ids. */
    uint256[] private _recipes;

    /** @notice The address of the [Gold](/docs/gadgets/Gold.md) instance. */
    address public gold;

    /** @notice Address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;

    /** @notice Address of the [Experience](/docs/core/Experience.md) instance. */
    address public experience;

    /** @notice Address of the [Stats](/docs/core/Stats.md) instance. */
    address public stats;

    /** @notice Address of the [Items](/docs/items/Items.md) instance. */
    address public items;

    /** @notice Map to track craft slots and cooldowns for each character. */
    mapping(bytes => CraftSlot) public craft_slots;

    // =============================================== Modifiers ======================================================

    /**
     * @notice Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
     * has allowance to access a composed ID.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    modifier onlyAllowed(bytes memory _id) {
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "Craft: onlyAllowed() msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     * @param _stats            The address of the [Stats](/docs/core/Stats.md) instance.
     * @param _gold             The address of the [Gold](/docs/gadgets/Gold.md) instance.
     * @param _items            The address of the [Items](/docs/items/Items.md) instance.
     */
    constructor(
        address _civilizations,
        address _experience,
        address _stats,
        address _gold,
        address _items
    ) {
        civilizations = _civilizations;
        experience = _experience;
        stats = _stats;
        gold = _gold;
        items = _items;
    }

    /** @notice Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @notice Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
    }

    /**
     * @notice Disables a recipe from beign crafted.
     *
     * Requirements:
     * @param _recipe_id   ID of the recipe.
     */
    function disableRecipe(uint256 _recipe_id) public onlyOwner {
        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Craft: disableRecipe() invalid recipe id."
        );
        recipes[_recipe_id].available = false;
    }

    /**
     * @notice Enables a recipe to be crafted.
     *
     * Requirements:
     * @param _recipe_id   ID of the recipe.
     */
    function enableRecipe(uint256 _recipe_id) public onlyOwner {
        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Craft: enableRecipe() invalid recipe id."
        );
        recipes[_recipe_id].available = true;
    }

    /**
     * @notice Adds a new recipe to craft.
     *
     * Requirements:
     * @param _materials            Array of material [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances address.
     * @param _amounts              Array of amounts for each material.
     * @param _stats                Stats to consume from the pool for craft.
     * @param _cooldown             Number of seconds for the recipe cooldown.
     * @param _level_required       Minimum level required to craft the recipe.
     * @param _gold_cost            Cost of [Gold](/docs/gadgets/Gold.md) required to craft the recipe.
     * @param _reward               ID of the token to reward for the [Items](/docs/items/Items.md) instance.
     * @param _experience_reward    Amount of experience rewarded for the recipe.
     */
    function addRecipe(
        address[] memory _materials,
        uint256[] memory _amounts,
        IStats.BasicStats memory _stats,
        uint256 _cooldown,
        uint256 _level_required,
        uint256 _gold_cost,
        uint256 _reward,
        uint256 _experience_reward
    ) public onlyOwner {
        uint256 id = _recipes.length + 1;
        require(
            _materials.length == _amounts.length,
            "Craft: addRecipe() materials and amounts not match."
        );
        recipes[id] = Recipe(
            id,
            _materials,
            _amounts,
            _stats,
            _cooldown,
            _level_required,
            _gold_cost,
            _reward,
            _experience_reward,
            true
        );
        _recipes.push(id);
    }

    /**
     * @notice Initializes a recipe to be crafted.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _recipe_id    ID of the recipe.
     */
    function craft(bytes memory _id, uint256 _recipe_id)
        public
        whenNotPaused
        onlyAllowed(_id)
    {
        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Craft: enableRecipe() invalid recipe id."
        );
        require(
            _isSlotAvailable(_id),
            "Craft: craft() slot not available to craft."
        );

        Recipe memory _recipe = recipes[_recipe_id];
        require(_recipe.available, "Craft: craft() recipe is not available.");

        require(
            IExperience(experience).getLevel(_id) >= _recipe.level_required,
            "Craft: craft() not enough level."
        );

        if (_recipe.gold_cost > 0) {
            IBaseFungibleItem(gold).consume(_id, _recipe.gold_cost);
        }

        for (uint256 i = 0; i < _recipe.materials.length; i++) {
            IBaseFungibleItem(_recipe.materials[i]).consume(
                _id,
                _recipe.material_amounts[i]
            );
        }

        IStats(stats).consume(_id, _recipe.stats_required);

        craft_slots[_id] = CraftSlot(
            block.timestamp + _recipe.cooldown,
            _recipe.id,
            false
        );
    }

    /**
     * @notice Claims a recipe already crafted.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    function claim(bytes memory _id) public whenNotPaused onlyAllowed(_id) {
        require(_isSlotClaimable(_id), "Craft: claim() slot is not claimable.");

        uint256 reward = _claim(_id);
        IExperience(experience).assignExperience(_id, reward);
    }

    // =============================================== Getters ========================================================

    /**
     * @notice Returns the full information of a recipe.
     *
     * Requirements:
     * @param _recipe_id   ID of the recipe.
     *
     * @return _recipe     Full information of the recipe
     */
    function getRecipe(uint256 _recipe_id)
        public
        view
        returns (Recipe memory _recipe)
    {
        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Craft: getRecipe() invalid recipe id."
        );
        return recipes[_recipe_id];
    }

    /**
     * @notice Returns character craft slot information.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     *
     * @return _slot    Full information of character crafting slot.
     */
    function getCharacterSlot(bytes memory _id)
        public
        view
        returns (CraftSlot memory _slot)
    {
        return craft_slots[_id];
    }

    // =============================================== Internal =======================================================

    /**
     * @notice Internal check if the crafting slot is available to be used.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _available   Boolean to know if the slot is available.
     */
    function _isSlotAvailable(bytes memory _id)
        internal
        view
        returns (bool _available)
    {
        CraftSlot memory s = craft_slots[_id];

        if (s.cooldown == 0) {
            return true;
        }

        return s.cooldown <= block.timestamp && s.claimed;
    }

    /**
     * @notice Internal check if the crafting slot is claimable.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _available   Boolean to know if the slot is claimable.
     */
    function _isSlotClaimable(bytes memory _id) internal view returns (bool) {
        CraftSlot memory s = craft_slots[_id];
        return
            s.cooldown <= block.timestamp && !s.claimed && s.last_recipe != 0;
    }

    /**
     * @notice Internal function to claim the reward from the slot.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _experience  Amount of experience rewarded.
     */
    function _claim(bytes memory _id) internal returns (uint256 _experience) {
        Recipe memory _recipe = recipes[craft_slots[_id].last_recipe];
        IItems(items).mint(
            ICivilizations(civilizations).ownerOf(_id),
            _recipe.reward
        );

        return _recipe.experience_reward;
    }
}
