// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../interfaces/IForge.sol";
import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/IStats.sol";
import "../interfaces/IBaseFungibleItem.sol";

/**
 * @dev `Forge` is a contract to convert the raw material to craftable pieces.
 */

contract Forge is Ownable, IForge {
    // =============================================== Storage ========================================================

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Address of the `Experience` instance. **/
    address public experience;

    /** @dev Address of the `Stats` instance. **/
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

    /** @dev The address of the `Gold` instance. **/
    address public gold;

    // =============================================== Modifiers ======================================================

    /**
     * @dev Checks if `msg.sender` is owner or allowed to manipulate a composed ID.
     */
    modifier onlyAllowed(bytes memory id) {
        require(
            ICivilizations(civilizations).exists(id),
            "Forge: can't get access to a non minted token."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, id),
            "Forge: msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _civilizations        The address of the `Civilizations` instance.
     * @param _experience           The address of the `Experience` instance.
     * @param _stats                The address of the `Experience` instance.
     * @param _gold                 The address of the `Gold` instance.
     * @param _token                Address of the token to charge for forge upgrades.
     * @param _price                Price of forge upgrades.
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

    /**
     * @dev Disables a forge recipe.
     * @param id  ID of the recipe.
     */
    function disableRecipe(uint256 id) public onlyOwner {
        require(
            id != 0 && id <= _recipes.length,
            "Forge: recipe id doesn't exist."
        );
        recipes[id].available = false;
    }

    /**
     * @dev Adds a new recipe to the forge.
     * @param _materials        Addresses of the raw resources for the creation.
     * @param _amounts          Amounts for each raw resource.
     * @param cooldown          Cooldown in seconds for the recipe.
     * @param level_required    Minimum level required.
     * @param cost              Gold cost of the recipe.
     * @param might_cost        Might stat cost from the stats pool.
     * @param speed_cost        Speed stat cost from the stats pool.
     * @param intellect_cost    Intelect stat cost from the stats pool.
     * @param reward            Address of the reward contract.
     * @param exp_reward            Amount of experience rewarded.
     */
    function addRecipe(
        address[] memory _materials,
        uint256[] memory _amounts,
        uint256 might_cost,
        uint256 speed_cost,
        uint256 intellect_cost,
        uint256 cooldown,
        uint256 level_required,
        uint256 cost,
        uint256 exp_reward,
        address reward
    ) public onlyOwner {
        uint256 id = _recipes.length + 1;
        require(
            _materials.length == _amounts.length,
            "Forge: materials and amounts arrays should be the same length"
        );
        recipes[id] = Recipe(
            id,
            _materials,
            _amounts,
            Stats(might_cost, speed_cost, intellect_cost),
            cooldown,
            level_required,
            reward,
            exp_reward,
            cost,
            true
        );
        _recipes.push(id);
    }

    /**
     * @dev Enables a forge recipe.
     * @param id  ID of the recipe.
     */
    function enableRecipe(uint256 id) public onlyOwner {
        require(
            id != 0 && id <= _recipes.length,
            "Forge: recipe id doesn't exist."
        );
        recipes[id].available = true;
    }

    /**
     * @dev Upgrades character to use another forge slot.
     * @param id  Composed ID of the token.
     */
    function buyUpgrade(bytes memory id) public onlyAllowed(id) {
        require(
            IERC20(token).balanceOf(msg.sender) >= price,
            "Forge: not enough balance of payment tokens to mint tokens."
        );
        require(
            IERC20(token).allowance(msg.sender, address(this)) >= price,
            "Forge: not enough allowance to mint tokens."
        );
        bool canUpgrade = false;
        if (!forges[id].forge_2.available) {
            canUpgrade = true;
        }
        if (!forges[id].forge_3.available) {
            canUpgrade = true;
        }

        require(canUpgrade, "Forge: user doesn't have buyable spots");
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
    ) public onlyAllowed(id) {
        if (_forge == 2) {
            require(
                forges[id].forge_2.available,
                "Forge: forge 2 is not upgraded"
            );
        }
        if (_forge == 3) {
            require(
                forges[id].forge_3.available,
                "Forge: forge 3 is not upgraded"
            );
        }
        require(
            recipe != 0 && recipe <= _recipes.length,
            "Forge: recipe id doesn't exist."
        );
        require(
            _isForgeAvailable(id, _forge),
            "Forge: the forge trying to use is not available for use."
        );

        Recipe memory r = recipes[recipe];
        require(
            r.available,
            "Forge: the recipe trying to forge is not available at the momento."
        );
        require(
            IExperience(experience).getLevel(id) >= r.level_required,
            "Forge: the character doesn't have the level required to forge the material."
        );

        if (r.cost > 0) {
            IBaseFungibleItem(gold).consume(id, r.cost);
        }

        for (uint256 i = 0; i < r.raw_materials.length; i++) {
            IBaseFungibleItem(r.raw_materials[i]).consume(id, r.raw_amounts[i]);
        }

        IStats(stats).consume(
            id,
            r.requirements.might,
            r.requirements.speed,
            r.requirements.intellect
        );

        require(
            _assignRecipeToForge(id, _forge, r),
            "Forge: unable to assign recipe to forge."
        );
    }

    /**
     * @dev Claims a recipe already forged.
     * @param id        Composed ID of the character.
     * @param _forge    Number of the forge to use.
     */
    function claim(bytes memory id, uint256 _forge) public onlyAllowed(id) {
        if (_forge == 2) {
            require(
                forges[id].forge_2.available,
                "Forge: forge 2 is not upgraded"
            );
        }
        if (_forge == 3) {
            require(
                forges[id].forge_3.available,
                "Forge: forge 3 is not upgrades"
            );
        }

        require(
            _isForgeClaimable(id, _forge),
            "Forge: the forge trying to use is not available for claim."
        );

        (bool success, uint256 reward) = _claimForge(id, _forge);
        require(success, "Forge: unable to claim recipe from forge.");
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
        require(valid, "Forge: selected forge is invalid");

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
        require(valid, "Forge: selected forge is invalid");
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
    ) internal returns (bool) {
        Forge memory f = Forge(true, block.timestamp + r.cooldown, r.id, false);
        if (_forge == 1) {
            forges[id].forge_1 = f;
            return true;
        }
        if (_forge == 2) {
            forges[id].forge_2 = f;
            return true;
        }
        if (_forge == 3) {
            forges[id].forge_3 = f;
            return true;
        }

        return false;
    }

    /**
     * @dev Internal function claim a reward from a forge.
     * @param id        Composed ID of the character.
     * @param _forge    Number of the forge to use.
     */
    function _claimForge(bytes memory id, uint256 _forge)
        internal
        returns (bool, uint256)
    {
        Recipe memory r;
        if (_forge == 1) {
            r = recipes[forges[id].forge_1.last_recipe];
            forges[id].forge_1.last_recipe_claimed = true;
        } else if (_forge == 2) {
            r = recipes[forges[id].forge_2.last_recipe];
            forges[id].forge_1.last_recipe_claimed = true;
        } else if (_forge == 3) {
            r = recipes[forges[id].forge_3.last_recipe];
            forges[id].forge_1.last_recipe_claimed = true;
        } else {
            return (false, 0);
        }

        IBaseFungibleItem(r.reward).mintTo(id, 1);

        return (true, r.exp_reward);
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
