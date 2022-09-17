// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC1155/utils/ERC1155Holder.sol";
import "@openzeppelin/contracts/token/ERC1155/IERC1155.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/IEquipment.sol";
import "../interfaces/IItems.sol";
import "../interfaces/IStats.sol";

/**
 * @title Equipment
 * @notice This contract enables characters to equip/unequip `ERC1155` tokens stored through the [Items](/docs/items/Items.md) implementation.
 *
 * @notice Implementation of the [IEquipment](/docs/interfaces/IEquipment.md) interface.
 */
contract Equipment is IEquipment, Ownable, ERC1155Holder, Pausable {
    // =============================================== Storage ========================================================

    /** @dev Address of the [Civilizations](/docs/core/Civilizations.md) instance. **/
    address public civilizations;

    /** @dev Address of the [Experience](/docs/core/Experience.md) instance. **/
    address public experience;

    /** @dev Address of the [Items](/docs/items/Items.md) instance. **/
    address public items;

    /** @dev Map to track the equipment of characters. **/
    mapping(bytes => mapping(EquipmentSlot => ItemEquiped)) character_equipments;

    /** @dev Map to track the slots that can be used by an item type. **/
    mapping(EquipmentSlot => mapping(IItems.ItemType => bool)) slots_types;

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
            "Equipment: onlyAllowed() msg.sender is not allowed to access this token."
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
     * @param _items            The address of the [Items](/docs/items/Items.md) instance.
     */
    constructor(
        address _civilizations,
        address _experience,
        address _items
    ) {
        civilizations = _civilizations;
        experience = _experience;
        items = _items;

        slots_types[EquipmentSlot.HELMET][IItems.ItemType.HELMET] = true;
        slots_types[EquipmentSlot.SHOULDER_GUARDS][
            IItems.ItemType.SHOULDER_GUARDS
        ] = true;
        slots_types[EquipmentSlot.ARM_GUARDS][
            IItems.ItemType.ARM_GUARDS
        ] = true;
        slots_types[EquipmentSlot.HANDS][IItems.ItemType.HANDS] = true;
        slots_types[EquipmentSlot.RING][IItems.ItemType.RING] = true;
        slots_types[EquipmentSlot.NECKLACE][IItems.ItemType.NECKLACE] = true;
        slots_types[EquipmentSlot.CHEST][IItems.ItemType.CHEST] = true;
        slots_types[EquipmentSlot.LEGS][IItems.ItemType.LEGS] = true;
        slots_types[EquipmentSlot.BELT][IItems.ItemType.BELT] = true;
        slots_types[EquipmentSlot.FEET][IItems.ItemType.FEET] = true;
        slots_types[EquipmentSlot.CAPE][IItems.ItemType.CAPE] = true;
        slots_types[EquipmentSlot.LEFT_HAND][IItems.ItemType.ONE_HANDED] = true;
        slots_types[EquipmentSlot.LEFT_HAND][IItems.ItemType.TWO_HANDED] = true;
        slots_types[EquipmentSlot.RIGHT_HAND][
            IItems.ItemType.ONE_HANDED
        ] = true;
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
     * @dev Assigns an item to an item slot. If it is already used, it replaces it.
     * @param id            Composed ID of the character.
     * @param item_slot     Slot from the equipment to remove.
     * @param item_id       ID of the item to assign.
     */
    function equip(
        bytes memory id,
        EquipmentSlot item_slot,
        uint256 item_id
    ) public whenNotPaused onlyAllowed(id) {
        IItems.Item memory item_data = IItems(items).getItem(item_id);

        require(
            slots_types[item_slot][item_data.item_type],
            "Equipment: equip() item type not for this slot."
        );

        if (item_data.item_type == IItems.ItemType.TWO_HANDED) {
            if (character_equipments[id][EquipmentSlot.RIGHT_HAND].equiped) {
                unequip(id, EquipmentSlot.RIGHT_HAND);
            }
        }

        if (character_equipments[id][item_slot].equiped) {
            unequip(id, item_slot);
        }

        IERC1155(items).safeTransferFrom(
            msg.sender,
            address(this),
            item_id,
            1,
            ""
        );

        character_equipments[id][item_slot].equiped = true;
        character_equipments[id][item_slot].id = item_id;
    }

    /**
     * @dev Removes an item from the character equipment and transfers de ERC1155 token.
     * @param id            Composed ID of the character.
     * @param item_slot     Slot from the equipment to remove.
     */
    function unequip(bytes memory id, EquipmentSlot item_slot)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        require(
            character_equipments[id][item_slot].equiped,
            "Equipment: unequip() item slot not equiped."
        );

        uint256 item_id = character_equipments[id][item_slot].id;
        character_equipments[id][item_slot].equiped = false;
        character_equipments[id][item_slot].id = 0;

        IERC1155(items).safeTransferFrom(
            address(this),
            ICivilizations(civilizations).ownerOf(id),
            item_id,
            1,
            ""
        );
    }

    // =============================================== Getters ========================================================

    /** @dev Returns the full requipment of a character.
     *  @param id  Composed ID of the token.
     */
    function getCharacterEquipment(bytes memory id)
        public
        view
        returns (CharacterEquipment memory)
    {
        return
            CharacterEquipment(
                character_equipments[id][EquipmentSlot.HELMET],
                character_equipments[id][EquipmentSlot.SHOULDER_GUARDS],
                character_equipments[id][EquipmentSlot.ARM_GUARDS],
                character_equipments[id][EquipmentSlot.HANDS],
                character_equipments[id][EquipmentSlot.RING],
                character_equipments[id][EquipmentSlot.NECKLACE],
                character_equipments[id][EquipmentSlot.CHEST],
                character_equipments[id][EquipmentSlot.LEGS],
                character_equipments[id][EquipmentSlot.BELT],
                character_equipments[id][EquipmentSlot.FEET],
                character_equipments[id][EquipmentSlot.CAPE],
                character_equipments[id][EquipmentSlot.LEFT_HAND],
                character_equipments[id][EquipmentSlot.RIGHT_HAND]
            );
    }

    /** @dev Returns the total modifiers from the equipment.
     *  @param id  Composed ID of the token.
     */
    function getCharacterTotalStatsModifiers(bytes memory id)
        public
        view
        returns (IStats.BasicStats memory, IStats.BasicStats memory)
    {
        IStats.BasicStats memory additions = IStats.BasicStats(0, 0, 0);
        IStats.BasicStats memory reductions = IStats.BasicStats(0, 0, 0);

        for (uint256 i = 0; i < 13; i++) {
            ItemEquiped memory e = character_equipments[id][EquipmentSlot(i)];
            if (e.equiped) {
                IItems.Item memory item = IItems(items).getItem(e.id);
                additions.might += item.stat_modifiers.might;
                additions.speed += item.stat_modifiers.speed;
                additions.intellect += item.stat_modifiers.intellect;
                reductions.might += item.stat_modifiers.might_reducer;
                reductions.speed += item.stat_modifiers.speed_reducer;
                reductions.intellect += item.stat_modifiers.intellect_reducer;
            }
        }

        return (additions, reductions);
    }

    /** @dev Returns the total attributes from the equipment.
     *  @param id  Composed ID of the token.
     */
    function getCharacterTotalAttributes(bytes memory id)
        public
        view
        returns (IItems.BaseAttributes memory, IItems.BaseAttributes memory)
    {
        IItems.BaseAttributes memory additions = IItems.BaseAttributes(
            0,
            0,
            0,
            0,
            0,
            0
        );
        IItems.BaseAttributes memory reductions = IItems.BaseAttributes(
            0,
            0,
            0,
            0,
            0,
            0
        );

        for (uint256 i = 0; i < 13; i++) {
            ItemEquiped memory e = character_equipments[id][EquipmentSlot(i)];
            if (e.equiped) {
                IItems.Item memory item = IItems(items).getItem(e.id);
                additions.atk += item.attributes.atk;
                additions.def += item.attributes.def;
                additions.range += item.attributes.range;
                additions.mag_atk += item.attributes.mag_atk;
                additions.mag_def += item.attributes.mag_def;
                additions.rate += item.attributes.rate;

                reductions.atk += item.attributes.atk_reducer;
                reductions.def += item.attributes.def_reducer;
                reductions.range += item.attributes.range_reducer;
                reductions.mag_atk += item.attributes.mag_atk_reducer;
                reductions.mag_def += item.attributes.mag_def_reducer;
                reductions.rate += item.attributes.rate_reducer;
            }
        }

        return (additions, reductions);
    }
}
