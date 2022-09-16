// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface ICraft {
    struct Stats {
        uint256 might;
        uint256 speed;
        uint256 intellect;
    }

    struct Recipe {
        uint256 id;
        address[] materials;
        uint256[] amounts;
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
}
