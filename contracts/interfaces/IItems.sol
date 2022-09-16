// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title IItems
 * @notice Interface for the {Items} contract.
 */
interface IItems {
    /**
     * @notice Internal struct to store the item properties.
     *
     * Requirements:
     * @param min   The minimum amount of experience to achieve the level.
     * @param max   The maximum amount of experience for this level (non inclusive).
     */
    struct Item {
        uint256 id;
        uint256 level_required;
        ItemType item_type;
        StatsModifiers stat_modifiers;
        Attributes attributes;
        bool available;
    }

    enum ItemType {
        HELMET,
        SHOULDER_GUARDS,
        ARM_GUARDS,
        HANDS,
        RING,
        NECKLACE,
        CHEST,
        LEGS,
        BELT,
        FEET,
        CAPE,
        ONE_HANDED,
        TWO_HANDED
    }

    struct StatsModifiers {
        uint256 might;
        uint256 might_reducer;
        uint256 speed;
        uint256 speed_reducer;
        uint256 intellect;
        uint256 intellect_reducer;
    }

    struct BaseAttributes {
        uint256 atk;
        uint256 def;
        uint256 range;
        uint256 mag_atk;
        uint256 mag_def;
        uint256 rate;
    }

    struct Attributes {
        uint256 atk;
        uint256 atk_reducer;
        uint256 def;
        uint256 def_reducer;
        uint256 range;
        uint256 range_reducer;
        uint256 mag_atk;
        uint256 mag_atk_reducer;
        uint256 mag_def;
        uint256 mag_def_reducer;
        uint256 rate;
        uint256 rate_reducer;
    }

    function addItem(
        uint256 level_required,
        ItemType item_type,
        StatsModifiers memory stat_modifiers,
        Attributes memory attributes
    ) external;

    function disableItem(uint256 id) external;

    function enableItem(uint256 id) external;

    function getItem(uint256 id) external view returns (Item memory);

    function mint(address to, uint256 id) external;
}
