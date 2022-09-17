// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

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
contract Forge is IForge, Ownable, Pausable {
    // =============================================== Storage ========================================================

    /** @dev Address of the [Civilizations](/docs/core/Civilizations.md) instance. **/
    address public civilizations;

    /** @dev Address of the [Experience](/docs/core/Experience.md) instance. **/
    address public experience;

    /** @dev Address of the [Stats](/docs/core/Stats.md) instance. **/
    address public stats;

    /** @dev Map to track available recipes on the forge. **/
    mapping(uint256 => Recipe) public recipes;

    /** @dev Array to track all the recipes ids. **/
    uint256[] private _recipes;

    /** @dev Map to track forges and cooldowns for each character. **/
    mapping(bytes => Forges) public forges;

    /** @dev Address of the token used to charge the mint. **/
    address public token;

    /** @dev Price of forge upgrades. **/
    uint256 public price;

    /** @dev The address of the [Gold](/docs/gadgets/Gold.md) instance. **/
    address public gold;

    // =============================================== Modifiers ======================================================

    /**
     * @notice Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
     * has allowance to access a composed ID.
     *
     * Requirements:
     * @param _id    Composed ID of the token.
     */
    modifier onlyAllowed(bytes memory _id) {
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "Forge: onlyAllowed() msg.sender is not allowed to access this token."
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
     * @param _token            Address of the token used to purchase.
     * @param _price            Price for each token.
     */
    constructor(
        address _civilizations,
        address _experience,
        address _stats,
        address _gold,
        address _token,
        uint256 _price
    ) {
        civilizations = _civilizations;
        experience = _experience;
        stats = _stats;
        token = _token;
        price = _price;
        gold = _gold;
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
     * @dev Disables a forge recipe.
     * @param id  ID of the recipe.
     */
    function disableRecipe(uint256 id) public onlyOwner {
        require(
            id != 0 && id <= _recipes.length,
            "Forge: disableRecipe() invalid recipe id."
        );
        recipes[id].available = false;
    }

    /**
     * @dev Enables a forge recipe.
     * @param id  ID of the recipe.
     */
    function enableRecipe(uint256 id) public onlyOwner {
        require(
            id != 0 && id <= _recipes.length,
            "Forge: enableRecipe() invalid recipe id."
        );
        recipes[id].available = true;
    }

    /**
     * @dev Adds a new recipe to the forge.
     * @param materials            Addresses of the raw resources for the creation.
     * @param amounts              Amounts for each raw resource.
     * @param cooldown              Cooldown in seconds for the recipe.
     * @param level_required        Minimum level required.
     * @param cost                  Gold cost of the recipe.
     * @param stats                 Stat cost for the recipe.
     * @param reward                Address of the reward contract.
     * @param experience_reward     Amount of experience rewarded.
     */
    function addRecipe(
        address[] memory materials,
        uint256[] memory amounts,
        IStats.BasicStats memory stats,
        uint256 cooldown,
        uint256 level_required,
        uint256 cost,
        uint256 experience_reward,
        address reward
    ) public onlyOwner {
        uint256 id = _recipes.length + 1;
        require(
            materials.length == amounts.length,
            "Forge: addRecipe() materials and amounts not match."
        );
        recipes[id] = Recipe(
            id,
            materials,
            amounts,
            stats,
            cooldown,
            level_required,
            reward,
            experience_reward,
            cost,
            true
        );
        _recipes.push(id);
    }

    /**
     * @dev Upgrades character to use another forge slot.
     * @param id  Composed ID of the token.
     */
    function buyUpgrade(bytes memory id) public whenNotPaused onlyAllowed(id) {
        require(
            IERC20(token).balanceOf(msg.sender) >= price,
            "Forge: buyUpgrade() not enough balance to buy upgrade."
        );
        require(
            IERC20(token).allowance(msg.sender, address(this)) >= price,
            "Forge: buyUpgrade() not enough allowance to buy upgrade."
        );
        bool canUpgrade = false;
        if (!forges[id].forge_2.available) {
            canUpgrade = true;
        }
        if (!forges[id].forge_3.available) {
            canUpgrade = true;
        }

        require(canUpgrade, "Forge: buyUpgrade() no spot available.");
        IERC20(token).transferFrom(msg.sender, address(this), price);

        if (!forges[id].forge_2.available) {
            forges[id].forge_2.available = true;
            return;
        }

        if (!forges[id].forge_3.available) {
            forges[id].forge_3.available = true;
            return;
        }
    }

    /**
     * @dev Forges a recipe.
     * @param id        Composed ID of the character.
     * @param recipe    ID of the recipe to forge.
     * @param _forge    Number of the forge to use.
     */
    function forge(
        bytes memory id,
        uint256 recipe,
        uint256 _forge
    ) public whenNotPaused onlyAllowed(id) {
        if (_forge == 2) {
            require(
                forges[id].forge_2.available,
                "Forge: forge() forge_2 is not available."
            );
        }
        if (_forge == 3) {
            require(
                forges[id].forge_3.available,
                "Forge: forge() forge_3 is not available."
            );
        }
        require(
            recipe != 0 && recipe <= _recipes.length,
            "Forge: forge() invalid recipe id."
        );
        require(
            _isForgeAvailable(id, _forge),
            "Forge: forge() forge not available."
        );

        Recipe memory r = recipes[recipe];
        require(r.available, "Forge: forge() recipe not available.");

        require(
            IExperience(experience).getLevel(id) >= r.level_required,
            "Forge: forge() not enough level."
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

        _assignRecipeToForge(id, _forge, r);
    }

    /**
     * @dev Claims a recipe already forged.
     * @param id        Composed ID of the character.
     * @param _forge    Number of the forge to use.
     */
    function claim(bytes memory id, uint256 _forge)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        if (_forge == 2) {
            require(
                forges[id].forge_2.available,
                "Forge: claim() forge_2 is not available."
            );
        }
        if (_forge == 3) {
            require(
                forges[id].forge_3.available,
                "Forge: claim() forge_3 is not available."
            );
        }

        require(
            _isForgeClaimable(id, _forge),
            "Forge: claim() forge not claimable."
        );

        uint256 reward = _claimForge(id, _forge);
        IExperience(experience).assignExperience(id, reward);
    }

    /**
     * @dev Transfers the total amount of `token` stored in the contract to `owner`.
     */
    function withdraw() public onlyOwner {
        uint256 balance = IERC20(token).balanceOf(address(this));
        IERC20(token).transfer(owner(), balance);
    }

    // =============================================== Getters ========================================================

    /**
     * @dev Reurns the recipe information of a recipe id.
     * @param id  ID of the recipe.
     */
    function getRecipe(uint256 id) public view returns (Recipe memory) {
        require(
            id != 0 && id <= _recipes.length,
            "Forge: getRecipe() invalid recipe id."
        );
        return recipes[id];
    }

    /**
     * @dev Reurns the forge information of a composed ID.
     * @param id    Composed ID of the character.
     * @param _forge ID of the forge.
     */
    function getCharacterForge(bytes memory id, uint256 _forge)
        public
        view
        returns (Forge memory)
    {
        (bool valid, Forge memory f) = _getForgeFromID(id, _forge);
        require(valid, "Forge: getCharacterForge() invalid forge id.");
        return f;
    }

    /**
     * @dev Reurns an array of booleans for the character forges upgraded.
     * @param id  Composed ID of the character.
     */
    function getCharacterForgesUpgrades(bytes memory id)
        public
        view
        returns (bool[3] memory)
    {
        return [
            true,
            forges[id].forge_2.available,
            forges[id].forge_3.available
        ];
    }

    /**
     * @dev Reurns an array of booleans for the character forges available to use.
     * @param id  Composed ID of the character.
     */
    function getCharacterForgesAvailability(bytes memory id)
        public
        view
        returns (bool[3] memory)
    {
        bool[3] memory upgrades = getCharacterForgesUpgrades(id);
        return [
            upgrades[0] && _isForgeAvailable(id, 1),
            upgrades[1] && _isForgeAvailable(id, 2),
            upgrades[2] && _isForgeAvailable(id, 3)
        ];
    }

    // =============================================== Internal =======================================================

    /**
     * @dev Internal function to check if a forge id is available to use.
     * @param id        Composed ID of the character.
     * @param _forge     Number of the forge to use.
     */
    function _isForgeAvailable(bytes memory id, uint256 _forge)
        internal
        view
        returns (bool)
    {
        (bool valid, Forge memory f) = _getForgeFromID(id, _forge);
        require(valid, "Forge: _isForgeAvailable() invalid forge id.");

        if (f.cooldown == 0) {
            return true;
        }

        return f.cooldown <= block.timestamp && f.last_recipe_claimed;
    }

    /**
     * @dev Internal function to check if a forge id is ready to claim.
     * @param id        Composed ID of the character.
     * @param _forge     Number of the forge to use.
     */
    function _isForgeClaimable(bytes memory id, uint256 _forge)
        internal
        view
        returns (bool)
    {
        (bool valid, Forge memory f) = _getForgeFromID(id, _forge);
        require(valid, "Forge: _isForgeClaimable() invalid forge id.");
        return
            f.cooldown <= block.timestamp &&
            !f.last_recipe_claimed &&
            f.last_recipe != 0;
    }

    /**
     * @dev Internal function to assign a recipe to a forge to create. This function assumes the forge trying to accessing
     *      is available (upgraded) and usable.
     * @param id        Composed ID of the character.
     * @param _forge    Number of the forge to use.
     * @param r         Recipe to be assigned.
     */
    function _assignRecipeToForge(
        bytes memory id,
        uint256 _forge,
        Recipe memory r
    ) internal {
        Forge memory f = Forge(true, block.timestamp + r.cooldown, r.id, false);
        if (_forge == 1) {
            forges[id].forge_1 = f;
        }
        if (_forge == 2) {
            forges[id].forge_2 = f;
        }
        if (_forge == 3) {
            forges[id].forge_3 = f;
        }
    }

    /**
     * @dev Internal function claim a reward from a forge.
     * @param id        Composed ID of the character.
     * @param _forge    Number of the forge to use.
     */
    function _claimForge(bytes memory id, uint256 _forge)
        internal
        returns (uint256)
    {
        Recipe memory r;
        if (_forge == 1) {
            r = recipes[forges[id].forge_1.last_recipe];
            forges[id].forge_1.last_recipe_claimed = true;
        }
        if (_forge == 2) {
            r = recipes[forges[id].forge_2.last_recipe];
            forges[id].forge_2.last_recipe_claimed = true;
        }

        if (_forge == 3) {
            r = recipes[forges[id].forge_3.last_recipe];
            forges[id].forge_3.last_recipe_claimed = true;
        }

        IBaseFungibleItem(r.reward).mintTo(id, 1);

        return r.experience_reward;
    }

    /**
     * @dev Internal function to return a forge instance from a number.
     * @param id        Composed ID of the character.
     * @param _forge    Number of the forge to use.
     */
    function _getForgeFromID(bytes memory id, uint256 _forge)
        internal
        view
        returns (bool, Forge memory)
    {
        Forge memory f;
        if (_forge == 1) {
            f = forges[id].forge_1;
            return (true, f);
        } else if (_forge == 2) {
            f = forges[id].forge_2;
            return (true, f);
        } else if (_forge == 3) {
            f = forges[id].forge_3;
            return (true, f);
        } else {
            return (false, f);
        }
    }
}
