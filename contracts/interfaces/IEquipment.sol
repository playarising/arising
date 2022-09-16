// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title IEquipment
 * @notice Interface for the {Equipment} contract.
 */
interface IEquipment {
    struct ItemEquiped {
        uint256 id;
        bool equiped;
    }

    enum EquipmentSlot {
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
        LEFT_HAND,
        RIGHT_HAND
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
