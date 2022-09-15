// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface IForge {
    struct Stats {
        uint256 might;
        uint256 speed;
        uint256 intellect;
    }

    struct Recipe {
        uint256 id;
        address[] raw_materials;
        uint256[] raw_amounts;
        Stats requirements;
        uint256 cooldown;
        uint256 level_required;
        address reward;
        uint256 exp_reward;
        uint256 cost;
        bool available;
    }

    struct Forge {
        bool available;
        uint256 cooldown;
        uint256 last_recipe;
        bool last_recipe_claimed;
    }

    struct Forges {
        Forge forge_1;
        Forge forge_2;
        Forge forge_3;
    }

    function disableRecipe(uint256 id) external;

    function enableRecipe(uint256 id) external;

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
    ) external;

    function buyUpgrade(bytes memory id) external;

    function forge(
        bytes memory id,
        uint256 recipe,
        uint256 _forge
    ) external;

    function claim(bytes memory id, uint256 _forge) external;

    function withdraw() external;

    function getRecipe(uint256 id) external view returns (Recipe memory);

    function getCharacterForge(bytes memory id, uint256 _forge)
        external
        view
        returns (Forge memory);

    function getCharacterForgesUpgrades(bytes memory id)
        external
        view
        returns (bool[3] memory);

    function getCharacterForgesAvailability(bytes memory id)
        external
        view
        returns (bool[3] memory);
}
