// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

interface IEquipment {
    struct Stats {
        uint256 might;
        uint256 speed;
        uint256 intellect;
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

    struct Item {
        uint256 id;
        uint256 external_id;
        uint256 level_required;
        ItemType item_type;
        StatsModifiers stat_modifiers;
        Attributes attributes;
        bool available;
    }

    struct ItemEquiped {
        uint256 id;
        bool equiped;
    }

    struct CharacterEquipment {
        ItemEquiped helmet;
        ItemEquiped shoulder_guards;
        ItemEquiped arm_guards;
        ItemEquiped hands;
        ItemEquiped rings;
        ItemEquiped necklace;
        ItemEquiped chest;
        ItemEquiped legs;
        ItemEquiped belt;
        ItemEquiped feet;
        ItemEquiped cape;
        ItemEquiped left_hand;
        ItemEquiped right_hand;
    }
}
