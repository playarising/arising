// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "../interfaces/IForge.sol";
import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/IStats.sol";
import "../interfaces/IBaseFungibleItem.sol";

/**
 * @title Forge
 * @notice This contract convets the raw resources into craftable material. It uses multiple instances of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) items.
 * Each character has access to a maximum of three usable forges to convert the resources.
 *
 * @notice Implementation of the [IForge](/docs/interfaces/IForge.md) interface.
 */
contract Forge is
    IForge,
    Initializable,
    PausableUpgradeable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    // =============================================== Storage ========================================================

    /** @notice Address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;

    /** @notice Address of the [Experience](/docs/core/Experience.md) instance. */
    address public experience;

    /** @notice Address of the [Stats](/docs/core/Stats.md) instance. */
    address public stats;

    /** @notice Map to track available recipes on the forge. */
    mapping(uint256 => Recipe) public recipes;

    /** @notice Array to track all the forge recipes IDs. */
    uint256[] private _recipes;

    /** @notice Map to track forges and cooldowns for characters. */
    mapping(bytes => mapping(uint256 => Forge)) public forges;

    /** @notice Constant for address of the `ERC20` token used to purchase forge upgrades. */
    address public token;

    /** @notice Constant for the price of each forge upgrade (in wei). */
    uint256 public price;

    // =============================================== Modifiers ======================================================

    /**
     * @notice Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
     * has allowance to access a composed ID.
     *
     * Requirements:
     * @param _id    Composed ID of the character.
     */
    modifier onlyAllowed(bytes memory _id) {
        require(
            ICivilizations(civilizations).exists(_id),
            "Forge: onlyAllowed() token not minted."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "Forge: onlyAllowed() msg.sender is not allowed to access this token."
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

    // =============================================== Setters ========================================================

    /**
     * @notice Initialize.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     * @param _stats            The address of the [Stats](/docs/core/Stats.md) instance.
     * @param _token            Address of the token used to purchase.
     * @param _price            Price for each upgrade.
     */
    function initialize(
        address _civilizations,
        address _experience,
        address _stats,
        address _token,
        uint256 _price
    ) public initializer {
        __Ownable_init();
        __Pausable_init();
        __UUPSUpgradeable_init();

        civilizations = _civilizations;
        experience = _experience;
        stats = _stats;
        token = _token;
        price = _price;
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
     * @notice Sets the price to upgrade a character.
     *
     * Requirements:
     * @param _price     Amount of tokens to pay for the upgrade.
     */
    function setUpgradePrice(uint256 _price) public onlyOwner {
        price = _price;
    }

    /**
     * @notice Disables a recipe from beign forged.
     *
     * Requirements:
     * @param _recipe_id   ID of the recipe.
     */
    function disableRecipe(uint256 _recipe_id) public onlyOwner {
        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Forge: disableRecipe() invalid recipe id."
        );
        recipes[_recipe_id].available = false;
        emit DisableRecipe(_recipe_id);
    }

    /**
     * @notice Enables a recipe to be forged.
     *
     * Requirements:
     * @param _recipe_id   ID of the recipe.
     */
    function enableRecipe(uint256 _recipe_id) public onlyOwner {
        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Forge: enableRecipe() invalid recipe id."
        );
        recipes[_recipe_id].available = true;
        emit EnableRecipe(_recipe_id);
    }

    /**
     * @notice Adds a new recipe to the forge.
     *
     * Requirements:
     * @param _name                 Name of the recipe.
     * @param _description          Description of the recipe.
     * @param _materials            Array of material [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances address.
     * @param _amounts              Array of amounts for each material.
     * @param _stats                Stats to consume from the pool for recipe.
     * @param _cooldown             Number of seconds for the recipe cooldown.
     * @param _level_required       Minimum level required to forge the recipe.
     * @param _reward               Address of the [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances to be rewarded for the recipe.
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
        address _reward,
        uint256 _experience_reward
    ) public onlyOwner {
        uint256 _recipe_id = _recipes.length + 1;
        require(
            _materials.length == _amounts.length,
            "Forge: addRecipe() materials and amounts not match."
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
     * @notice Updates a previously added forge recipe.
     *
     * Requirements:
     * @param _recipe   Full information of the recipe.
     */
    function updateRecipe(Recipe memory _recipe) public onlyOwner {
        require(
            _recipe.id != 0 && _recipe.id <= _recipes.length,
            "Forge: updateRecipe() invalid recipe id."
        );
        recipes[_recipe.id] = _recipe;
        emit RecipeUpdate(_recipe.id, _recipe.name, _recipe.description);
    }

    /**
     * @notice Purchases a forge upgrade for the character provided.
     *
     * Requirements:
     * @param _id  Composed ID of the characrter.
     */
    function buyUpgrade(
        bytes memory _id
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            IERC20(token).balanceOf(msg.sender) >= price,
            "Forge: buyUpgrade() not enough balance to buy upgrade."
        );
        require(
            IERC20(token).allowance(msg.sender, address(this)) >= price,
            "Forge: buyUpgrade() not enough allowance to buy upgrade."
        );

        bool canUpgrade = false;
        if (!forges[_id][2].available) {
            canUpgrade = true;
        }
        if (!forges[_id][3].available) {
            canUpgrade = true;
        }

        require(canUpgrade, "Forge: buyUpgrade() no spot available.");
        IERC20(token).transferFrom(msg.sender, owner(), price);

        if (!forges[_id][2].available) {
            forges[_id][2].available = true;
            return;
        }

        if (!forges[_id][3].available) {
            forges[_id][3].available = true;
            return;
        }
    }

    /**
     * @notice Forges a recipe and assigns it to the forge provided.
     *
     * Requirements:
     * @param _id           Composed ID of the characrter.
     * @param _recipe_id    ID of the recipe to forge.
     * @param _forge_id     ID of the forge to assign the recipe.
     */
    function forge(
        bytes memory _id,
        uint256 _recipe_id,
        uint256 _forge_id
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            _forge_id > 0 && _forge_id <= 3,
            "Forge: forge() invalid forge id."
        );

        if (_forge_id != 1) {
            require(
                forges[_id][_forge_id].available,
                "Forge: forge() forge is not available."
            );
        }

        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Forge: forge() invalid recipe id."
        );
        require(
            _isForgeAvailable(_id, _forge_id),
            "Forge: forge() forge is already being used."
        );

        Recipe memory _recipe = recipes[_recipe_id];
        require(_recipe.available, "Forge: forge() recipe not available.");

        require(
            IExperience(experience).getLevel(_id) >= _recipe.level_required,
            "Forge: forge() not enough level."
        );

        for (uint256 i = 0; i < _recipe.materials.length; i++) {
            IBaseFungibleItem(_recipe.materials[i]).consume(
                _id,
                _recipe.amounts[i]
            );
        }

        IStats(stats).consume(_id, _recipe.stats_required);
        forges[_id][_forge_id] = Forge(
            true,
            block.timestamp + _recipe.cooldown,
            _recipe.id,
            false
        );
    }

    /**
     * @notice Claims a recipe already forged.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _forge_id     ID of the forge to assign the recipe.
     */
    function claim(
        bytes memory _id,
        uint256 _forge_id
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            _forge_id > 0 && _forge_id <= 3,
            "Forge: claim() invalid forge id."
        );
        if (_forge_id != 1) {
            require(
                forges[_id][_forge_id].available,
                "Forge: claim() forge is not available."
            );
        }
        require(
            _isForgeClaimable(_id, _forge_id),
            "Forge: claim() forge not claimable."
        );

        forges[_id][_forge_id].last_recipe_claimed = true;

        Recipe memory _recipe = recipes[forges[_id][_forge_id].last_recipe];

        IBaseFungibleItem(_recipe.reward).mintTo(_id, 1);

        IExperience(experience).assignExperience(
            _id,
            _recipe.experience_reward
        );
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to return the recipe information.
     *
     * Requirements:
     * @param _recipe_id    ID of the forge recipe.
     *
     * @return _recipe      Full information of the recipe.
     */
    function getRecipe(
        uint256 _recipe_id
    ) public view returns (Recipe memory _recipe) {
        require(
            _recipe_id != 0 && _recipe_id <= _recipes.length,
            "Forge: getRecipe() invalid recipe id."
        );
        return recipes[_recipe_id];
    }

    /**
     * @notice External function to return the information of a character forge.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _forge_id     ID of the forge.
     *
     * @return _forge       Full information of the forge.
     */
    function getCharacterForge(
        bytes memory _id,
        uint256 _forge_id
    ) public view returns (Forge memory _forge) {
        require(
            _forge_id > 0 && _forge_id <= 3,
            "Forge: getCharacterForge() invalid forge id."
        );
        return forges[_id][_forge_id];
    }

    /**
     * @notice External function to return an array of booleans with the purchased forge upgrades for a character.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _upgrades    Array of booleans of upgrades purchases.
     */
    function getCharacterForgesUpgrades(
        bytes memory _id
    ) public view returns (bool[3] memory _upgrades) {
        return [true, forges[_id][2].available, forges[_id][3].available];
    }

    /**
     * @notice External function to return an array of booleans with the availability of the character forges.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _availability    Array of booleans of forge availability.
     */
    function getCharacterForgesAvailability(
        bytes memory _id
    ) public view returns (bool[3] memory _availability) {
        bool[3] memory upgrades = getCharacterForgesUpgrades(_id);
        return [
            upgrades[0] && _isForgeAvailable(_id, 1),
            upgrades[1] && _isForgeAvailable(_id, 2),
            upgrades[2] && _isForgeAvailable(_id, 3)
        ];
    }

    // =============================================== Internal =======================================================

    /**
     * @notice Internal function to check if a character forge is available.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _forge_id     ID of the forge.
     *
     * @return _available   Boolean to know if the forge is available.
     */
    function _isForgeAvailable(
        bytes memory _id,
        uint256 _forge_id
    ) internal view returns (bool _available) {
        require(
            _forge_id > 0 && _forge_id <= 3,
            "Forge: _isForgeAvailable() invalid forge id."
        );
        if (forges[_id][_forge_id].cooldown == 0) {
            return true;
        }

        return
            forges[_id][_forge_id].cooldown <= block.timestamp &&
            forges[_id][_forge_id].last_recipe_claimed;
    }

    /**
     * @notice Internal function to check if a character forge is claimable.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _forge_id     ID of the forge.
     *
     * @return _claimable   Boolean to know if the forge is claimable.
     */
    function _isForgeClaimable(
        bytes memory _id,
        uint256 _forge_id
    ) internal view returns (bool _claimable) {
        require(
            _forge_id > 0 && _forge_id <= 3,
            "Forge: _isForgeClaimable() invalid forge id."
        );
        return
            forges[_id][_forge_id].cooldown <= block.timestamp &&
            !forges[_id][_forge_id].last_recipe_claimed &&
            forges[_id][_forge_id].last_recipe != 0;
    }

    /** @notice Internal function make sure upgrade proxy caller is the owner. */
    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyOwner {}
}
