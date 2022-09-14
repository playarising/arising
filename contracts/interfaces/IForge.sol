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
        uint256 cost;
        bool available;
    }

    struct Forge {
        bool available;
        uint256 cooldown;
    }

    struct Forges {
        Forge forge_1;
        Forge forge_2;
        Forge forge_3;
    }
}
