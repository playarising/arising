// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "../interfaces/IStats.sol";
import "../interfaces/IItems.sol";

/**
 * @title IEquipment
 * @notice Interface for the [Equipment](/docs/core/Equipment.md) contract.
 */
interface IEquipment {
    /**
     * @notice Internal struct to store the information of an equipment slot.
     *
     * Requirements:
     * @param id        ID of the item equiped.
     * @param equiped   Boolean to determine if the slot is being used.
     */
    struct ItemEquiped {
        uint256 id;
        bool equiped;
    }

    /** @notice Enum to define the different slots that can be equiped */
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

    /**
     * @notice Struct to expose the information of a character equipment.
     *
     * Requirements:
     * @param helmet            Slot for the HELMET slot.
     * @param shoulder_guards   Slot for the SHOULDER_GUARDS slot.
     * @param arm_guards        Slot for the ARM_GUARDS slot.
     * @param hands             Slot for the HANDS slot.
     * @param rings             Slot for the RING slot.
     * @param necklace          Slot for the NECKLACE slot.
     * @param ichestd           Slot for the CHEST slot.
     * @param legs              Slot for the LEGS slot.
     * @param belt              Slot for the BELT slot.
     * @param feet              Slot for the FEET slot.
     * @param cape              Slot for the CAPE slot.
     * @param left_hand         Slot for the LEFT_HAND slot.
     * @param right_hand        Slot for the RIGHT_HAND slot.
     */
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

    /** @notice See [Equipment#pause](/docs/core/Equipment.md#pause) */
    function pause() external;

    /** @notice See [Equipment#unpause](/docs/core/Equipment.md#unpause) */
    function unpause() external;

    /** @notice See [Equipment#equip](/docs/core/Equipment.md#equip) */
    function equip(
        bytes memory _id,
        EquipmentSlot _slot,
        uint256 _item_id
    ) external;

    /** @notice See [Equipment#unequip](/docs/core/Equipment.md#unequip) */
    function unequip(bytes memory _id, EquipmentSlot _slot) external;

    /** @notice See [Equipment#getCharacterEquipment](/docs/core/Equipment.md#getCharacterEquipment) */
    function getCharacterEquipment(
        bytes memory _id
    ) external view returns (CharacterEquipment memory);

    /** @notice See [Equipment#getCharacterTotalStatsModifiers](/docs/core/Equipment.md#getCharacterTotalStatsModifiers) */
    function getCharacterTotalStatsModifiers(
        bytes memory _id
    ) external view returns (IStats.BasicStats memory _modifiers);

    /** @notice See [Equipment#getCharacterTotalAttributes](/docs/core/Equipment.md#getCharacterTotalAttributes) */
    function getCharacterTotalAttributes(
        bytes memory _id
    ) external view returns (IItems.BaseAttributes memory _modifiers);
}
