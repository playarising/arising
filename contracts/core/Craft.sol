// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

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
contract Craft is
    ICraft,
    Initializable,
    PausableUpgradeable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    // =============================================== Storage ========================================================

    /** @notice Map to track available recipes for craft. */
    mapping(uint256 => Recipe) public recipes;

    /** @notice Array to track all the recipes IDs. */
    uint256[] private _recipes;

    /** @notice Map to track available upgrades to craft. */
    mapping(uint256 => Upgrade) public upgrades;

    /** @notice Array to track all the upgrades IDs. */
    uint256[] private _upgrades;

    /** @notice Address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;

    /** @notice Address of the [Experience](/docs/core/Experience.md) instance. */
    address public experience;

    /** @notice Address of the [Stats](/docs/core/Stats.md) instance. */
    address public stats;

    /** @notice Address of the [Items](/docs/items/Items.md) instance. */
    address public items;

    /** @notice Map to track craft slots and cooldowns for each character. */
    mapping(bytes => Slot) public craft_slots;

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
            ICivilizations(civilizations).exists(_id),
            "Craft: onlyAllowed() token not minted."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "Craft: onlyAllowed() msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Events =========================================================

    /**
     * @notice Event emmited when the [addRecipe](#addRecipe) function is called.
     *
     * Requirements:
     * @param _recipe_id    ID of the recipe added.
     * @param _name         Name of the recipe.
     * @param _description  Recipe description
     */
    event AddRecipe(uint256 _recipe_id, string _name, string _description);

    /**
     * @notice Event emmited when the [updateRecipe](#updateRecipe) function is called.
     *
     * Requirements:
     * @param _recipe_id    ID of the recipe added.
     * @param _name         Name of the recipe.
     * @param _description  Recipe description
     */
    event RecipeUpdate(uint256 _recipe_id, string _name, string _description);

    /**
     * @notice Event emmited when the [enableRecipe](#enableRecipe) function is called.
     *
     * Requirements:
     * @param _recipe_id    ID of the recipe enabled.
     */
    event EnableRecipe(uint256 _recipe_id);

    /**
     * @notice Event emmited when the [disableRecipe](#disableRecipe) function is called.
     *
     * Requirements:
     * @param _recipe_id    ID of the recipe disabled.
     */
    event DisableRecipe(uint256 _recipe_id);

    /**
     * @notice Event emmited when the [addUpgrade](#addUpgrade) function is called.
     *
     * Requirements:
     * @param _upgrade_id       ID of the the upgrade added.
     * @param _name             Name of the recipe.
     * @param _description      Recipe description
     */
    event AddUpgrade(uint256 _upgrade_id, string _name, string _description);

    /**
     * @notice Event emmited when the [updateUpgrade](#updateUpgrade) function is called.
     *
     * Requirements:
     * @param _upgrade_id       ID of the the upgrade added.
     * @param _name             Name of the recipe.
     * @param _description      Recipe description
     */
    event UpgradeUpdate(uint256 _upgrade_id, string _name, string _description);

    /**
     * @notice Event emmited when the [enableUpgrade](#enableUpgrade) function is called.
     *
     * Requirements:
     * @param _upgrade_id    ID of the the recipe added.
     */
    event EnableUpgrade(uint256 _upgrade_id);

    /**
     * @notice Event emmited when the [disableUpgrade](#disableUpgrade) function is called.
     *
     * Requirements:
     * @param _upgrade_id    ID of the the recipe added.
     */
    event DisableUpgrade(uint256 _upgrade_id);

    // =============================================== Setters ========================================================

    /**
     * @notice Initialize.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     * @param _stats            The address of the [Stats](/docs/core/Stats.md) instance.
     * @param _items            The address of the [Items](/docs/items/Items.md) instance.
     */
    function initialize(
        address _civilizations,
        address _experience,
        address _stats,
        address _items
    ) public initializer {
        __Ownable_init();
        __Pausable_init();
        __UUPSUpgradeable_init();

        civilizations = _civilizations;
        experience = _experience;
        stats = _stats;
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
        emit DisableRecipe(_recipe_id);
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
        emit EnableRecipe(_recipe_id);
    }

    /**
     * @notice Disables an upgrade from beign crafted.
     *
     * Requirements:
     * @param _upgrade_id   ID of the upgrade.
     */
    function disableUpgrade(uint256 _upgrade_id) public onlyOwner {
        require(
            _upgrade_id != 0 && _upgrade_id <= _recipes.length,
            "Craft: disableUpgrade() invalid upgrade id."
        );
        upgrades[_upgrade_id].available = false;
        emit DisableUpgrade(_upgrade_id);
    }

    /**
     * @notice Enables an upgrade to be crafted.
     *
     * Requirements:
     * @param _upgrade_id   ID of the upgrade.
     */
    function enableUpgrade(uint256 _upgrade_id) public onlyOwner {
        require(
            _upgrade_id != 0 && _upgrade_id <= _upgrades.length,
            "Craft: enableUpgrade() invalid upgrade id."
        );
        upgrades[_upgrade_id].available = true;
        emit EnableUpgrade(_upgrade_id);
    }

    /**
     * @notice Adds a new recipe to craft.
     *
     * Requirements:
     * @param _name                 Name of the recipe.
     * @param _description          Description of the recipe.
     * @param _materials            Array of material [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances address.
     * @param _amounts              Array of amounts for each material.
     * @param _stats                Stats to consume from the pool for craft.
     * @param _cooldown             Number of seconds for the recipe cooldown.
     * @param _level_required       Minimum level required to craft the recipe.
     * @param _reward               ID of the token to reward for the [Items](/docs/items/Items.md) instance.
     * @param _experience_reward    Amount of experience rewarded for the recipe.
     */
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
    ) public onlyOwner {
        uint256 _recipe_id = _recipes.length + 1;
        require(
            _materials.length == _amounts.length,
            "Craft: addRecipe() materials and amounts not match."
        );
        recipes[_recipe_id] = Recipe(
            _recipe_id,
            _name,
            _description,
            _materials,
            _amounts,
            _stats,
            _cooldown,
            _level_required,
            _reward,
            _experience_reward,
            true
        );
        _recipes.push(_recipe_id);
        emit AddRecipe(_recipe_id, _name, _description);
    }

    /**
     * @notice Updates a previously added craft recipe.
     *
     * Requirements:
     * @param _recipe   Full information of the recipe.
     */
    function updateRecipe(Recipe memory _recipe) public onlyOwner {
        require(
            _recipe.id != 0 && _recipe.id <= _recipes.length,
            "Craft: updateRecipe() invalid recipe id."
        );
        recipes[_recipe.id] = _recipe;
        emit RecipeUpdate(_recipe.id, _recipe.name, _recipe.description);
    }

    /**
     * @notice Adds a new recipe to craft.
     *
     * Requirements:
     * @param _name                 Name of the upgrade.
     * @param _description          Description of the upgrade.
     * @param _materials            Array of material [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances address.
     * @param _amounts              Array of amounts for each material.
     * @param _stats                Stats to consume from the pool for upgrade.
     * @param _sacrifice            Stats to sacrficed from the base stats for upgrade.
     * @param _level_required       Minimum level required to craft the recipe.
     * @param _upgraded_item        ID of the token item that is being upgraded from the [Items](/docs/items/Items.md) instance.
     * @param _reward               ID of the token to reward for the [Items](/docs/items/Items.md) instance.
     */
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
    ) public onlyOwner {
        uint256 _upgrade_id = _upgrades.length + 1;
        require(
            _materials.length == _amounts.length,
            "Craft: addUpgrade() materials and amounts not match."
        );
        upgrades[_upgrade_id] = Upgrade(
            _upgrade_id,
            _name,
            _description,
            _materials,
            _amounts,
            _stats,
            _sacrifice,
            _level_required,
            _upgraded_item,
            _reward,
            true
        );
        _upgrades.push(_upgrade_id);
        emit AddUpgrade(_upgrade_id, _name, _description);
    }

    /**
     * @notice Updates a previously added upgrade recipe.
     *
     * Requirements:
     * @param _upgrade   Full information of the recipe.
     */
    function updateUpgrade(Upgrade memory _upgrade) public onlyOwner {
        require(
            _upgrade.id != 0 && _upgrade.id <= _recipes.length,
            "Craft: updateUpgrade() invalid upgrade id."
        );
        upgrades[_upgrade.id] = _upgrade;
        emit UpgradeUpdate(_upgrade.id, _upgrade.name, _upgrade.description);
    }

    /**
     * @notice Initializes a recipe to be crafted.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _recipe_id    ID of the recipe.
     */
    function craft(
        bytes memory _id,
        uint256 _recipe_id
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Craft: craft() invalid recipe id."
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

        for (uint256 i = 0; i < _recipe.materials.length; i++) {
            IBaseFungibleItem(_recipe.materials[i]).consume(
                _id,
                _recipe.material_amounts[i]
            );
        }

        IStats(stats).consume(_id, _recipe.stats_required);

        craft_slots[_id] = Slot(
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
        craft_slots[_id].claimed = true;
        Recipe memory _recipe = recipes[craft_slots[_id].last_recipe];
        IItems(items).mint(
            ICivilizations(civilizations).ownerOf(_id),
            _recipe.reward
        );
    }

    /**
     * @notice Upgrades an item.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _upgrade_id   ID of the upgrade to perform.
     */
    function upgrade(
        bytes memory _id,
        uint256 _upgrade_id
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            _upgrade_id != 0 && _upgrade_id <= _recipes.length,
            "Craft: upgrade() invalid recipe id."
        );

        Upgrade memory _upgrade = upgrades[_upgrade_id];
        require(
            _upgrade.available,
            "Craft: upgrade() upgrade is not available."
        );

        require(
            IExperience(experience).getLevel(_id) >= _upgrade.level_required,
            "Craft: upgrade() not enough level."
        );

        for (uint256 i = 0; i < _upgrade.materials.length; i++) {
            IBaseFungibleItem(_upgrade.materials[i]).consume(
                _id,
                _upgrade.material_amounts[i]
            );
        }

        IStats(stats).consume(_id, _upgrade.stats_required);

        IStats(stats).sacrifice(_id, _upgrade.stats_sacrificed);

        IItems(items).burn(
            ICivilizations(civilizations).ownerOf(_id),
            _upgrade.upgraded_item
        );

        IItems(items).mint(
            ICivilizations(civilizations).ownerOf(_id),
            _upgrade.reward
        );
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
    function getRecipe(
        uint256 _recipe_id
    ) public view returns (Recipe memory _recipe) {
        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Craft: getRecipe() invalid recipe id."
        );
        return recipes[_recipe_id];
    }

    /**
     * @notice Returns the full information of an upgrade.
     *
     * Requirements:
     * @param _upgrade_id   ID of the upgrade.
     *
     * @return _upgrade     Full information of the upgrade
     */
    function getUpgrade(
        uint256 _upgrade_id
    ) public view returns (Upgrade memory _upgrade) {
        require(
            _upgrade_id != 0 && _upgrade_id <= _recipes.length,
            "Craft: getUpgrade() invalid recipe id."
        );
        return upgrades[_upgrade_id];
    }

    /**
     * @notice Returns character craft slot information.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     *
     * @return _slot    Full information of character crafting slot.
     */
    function getCharacterCrafSlot(
        bytes memory _id
    ) public view returns (Slot memory _slot) {
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
    function _isSlotAvailable(
        bytes memory _id
    ) internal view returns (bool _available) {
        Slot memory s = craft_slots[_id];

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
        Slot memory s = craft_slots[_id];
        return
            s.cooldown <= block.timestamp && !s.claimed && s.last_recipe != 0;
    }

    /** @notice Internal function make sure upgrade proxy caller is the owner. */
    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyOwner {}
}
