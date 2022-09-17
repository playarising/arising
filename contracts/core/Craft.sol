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

    /** @dev Map to track available recipes on the forge. **/
    mapping(uint256 => Recipe) public recipes;

    /** @dev Array to track all the recipes ids. **/
    uint256[] private _recipes;

    /** @dev The address of the [Gold](/docs/gadgets/Gold.md) instance. **/
    address public gold;

    /** @dev Address of the [Civilizations](/docs/core/Civilizations.md) instance. **/
    address public civilizations;

    /** @dev Address of the [Experience](/docs/core/Experience.md) instance. **/
    address public experience;

    /** @dev Address of the [Stats](/docs/core/Stats.md) instance. **/
    address public stats;

    /** @dev Address of the [Items](/docs/items/Items.md) instance. **/
    address public items;

    /** @dev Map to track craft slots and cooldowns for each character. **/
    mapping(bytes => CraftSlot) public slots;

    // =============================================== Modifiers ======================================================

    /**
     * @dev Checks if `msg.sender` is owner or allowed to manipulate a composed ID.
     */
    modifier onlyAllowed(bytes memory id) {
        require(
            ICivilizations(civilizations).exists(id),
            "Craft: can't get access to a non minted token."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, id),
            "Craft: msg.sender is not allowed to access this token."
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
     * @dev Disables a craft recipe.
     * @param id  ID of the recipe.
     */
    function disableRecipe(uint256 id) public onlyOwner {
        require(
            id != 0 && id <= _recipes.length,
            "Craft: recipe id doesn't exist."
        );
        recipes[id].available = false;
    }

    /**
     * @dev Enables a craft recipe.
     * @param id  ID of the recipe.
     */
    function enableRecipe(uint256 id) public onlyOwner {
        require(
            id != 0 && id <= _recipes.length,
            "Craft: recipe id doesn't exist."
        );
        recipes[id].available = true;
    }

    /**
     * @dev Craft a recipe.
     * @param id        Composed ID of the character.
     * @param recipe    ID of the recipe to craft.
     */
    function craft(bytes memory id, uint256 recipe)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        require(
            recipe != 0 && recipe <= _recipes.length,
            "Craft: recipe id doesn't exist."
        );
        require(
            _isSlotAvailable(id),
            "Craft: the slot is not available to craft."
        );

        Recipe memory r = recipes[recipe];
        require(
            r.available,
            "Craft: the recipe trying to craft is not available at the moment."
        );

        require(
            IExperience(experience).getLevel(id) >= r.level_required,
            "Craft: the character doesn't have the level required to forge the material."
        );

        if (r.cost > 0) {
            IBaseFungibleItem(gold).consume(id, r.cost);
        }

        for (uint256 i = 0; i < r.materials.length; i++) {
            IBaseFungibleItem(r.materials[i]).consume(
                id,
                r.material_amounts[i]
            );
        }

        IStats(stats).consume(id, r.stats_required);

        _assignRecipeToSlot(id, r);
    }

    /**
     * @dev Claims a recipe already crafted.
     * @param id        Composed ID of the character.
     */
    function claim(bytes memory id) public whenNotPaused onlyAllowed(id) {
        require(
            _isSlotClaimable(id),
            "Craft: the slot is not available for claim."
        );

        uint256 reward = _claim(id);
        IExperience(experience).assignExperience(id, reward);
    }

    // =============================================== Getters ========================================================

    /**
     * @dev Reurns the recipe information of a recipe id.
     * @param id  ID of the recipe.
     */
    function getRecipe(uint256 id) public view returns (Recipe memory) {
        require(
            id != 0 && id <= _recipes.length,
            "Craft: recipe id doesn't exist."
        );
        return recipes[id];
    }

    /**
     * @dev Reurns the craft slot information of a composed ID.
     * @param id    Composed ID of the character.
     */
    function getCharacterSlot(bytes memory id)
        public
        view
        returns (CraftSlot memory)
    {
        return slots[id];
    }

    // =============================================== Internal =======================================================

    /**
     * @dev Internal function to check if the craft slot is available to craft.
     * @param id    Composed ID of the character.
     */
    function _isSlotAvailable(bytes memory id) internal view returns (bool) {
        CraftSlot memory s = slots[id];

        if (s.cooldown == 0) {
            return true;
        }

        return s.cooldown <= block.timestamp && s.claimed;
    }

    /**
     * @dev Internal function to check if a craft slot is ready to claim.
     * @param id    Composed ID of the character.
     */
    function _isSlotClaimable(bytes memory id) internal view returns (bool) {
        CraftSlot memory s = slots[id];
        return
            s.cooldown <= block.timestamp && !s.claimed && s.last_recipe != 0;
    }

    /**
     * @dev Internal function to assign a recipe to a slot for craft
     * @param id        Composed ID of the character.
     * @param r         Recipe to be assigned.
     */
    function _assignRecipeToSlot(bytes memory id, Recipe memory r) internal {
        slots[id] = CraftSlot(block.timestamp + r.cooldown, r.id, false);
    }

    /**
     * @dev Internal function claim a reward from the slot and return the reward experience.
     *      This function assumes the slot is claimable
     * @param id    Composed ID of the character.
     */
    function _claim(bytes memory id) internal returns (uint256) {
        Recipe memory r = recipes[slots[id].last_recipe];
        IItems(items).mint(
            ICivilizations(civilizations).ownerOf(id),
            r.item_reward
        );

        return r.experience_reward;
    }
}
