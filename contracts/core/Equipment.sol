// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC1155/utils/ERC1155Holder.sol";
import "@openzeppelin/contracts/token/ERC1155/IERC1155.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

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
contract Equipment is
    IEquipment,
    ERC1155Holder,
    Initializable,
    PausableUpgradeable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    // =============================================== Storage ========================================================

    /** @notice Address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;

    /** @notice Address of the [Experience](/docs/core/Experience.md) instance. */
    address public experience;

    /** @notice Address of the [Items](/docs/items/Items.md) instance. */
    address public items;

    /** @notice Map to track the equipment of characters. */
    mapping(bytes => mapping(EquipmentSlot => ItemEquiped)) character_equipments;

    /** @notice Map to track the equipment slots and its attachable items. */
    mapping(EquipmentSlot => mapping(IItems.ItemType => bool)) slots_types;

    // =============================================== Modifiers ======================================================

    /**
     * @notice Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
     * has allowance to access a composed ID.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    modifier onlyAllowed(bytes memory _id) {
        require(
            ICivilizations(civilizations).exists(_id),
            "Equipment: onlyAllowed() token not minted."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "Equipment: onlyAllowed() msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Events =========================================================

    /**
     * @notice Event emmited when the [equip](#equip) function is called.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _slot     Slot of the item equiped.
     * @param _item_id  ID of the item equipped.
     */
    event Equipped(bytes _id, EquipmentSlot _slot, uint256 _item_id);

    /**
     * @notice Event emmited when the [unequip](#unequip) function is called.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _slot     Slot of the item unequipped.
     */
    event Unequipped(bytes _id, EquipmentSlot _slot);

    // =============================================== Setters ========================================================

    /**
     * @notice Initializer.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     * @param _items            The address of the [Items](/docs/items/Items.md) instance.
     */
    function initialize(
        address _civilizations,
        address _experience,
        address _items
    ) public initializer {
        __Ownable_init();
        __Pausable_init();
        __UUPSUpgradeable_init();

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
     * @notice Assigns an item to a character equipment slot. If the slot already has an equiped item
     * it is replaced by the item being equiped.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _slot     Slot to equip the item.
     * @param _item_id  ID of the item to equip.
     */
    function equip(
        bytes memory _id,
        EquipmentSlot _slot,
        uint256 _item_id
    ) public whenNotPaused onlyAllowed(_id) {
        IItems.Item memory item_data = IItems(items).getItem(_item_id);

        require(
            slots_types[_slot][item_data.item_type],
            "Equipment: equip() item type not for this slot."
        );

        require(
            IExperience(experience).getLevel(_id) >= item_data.level_required,
            "Equipment: equip() not enough level to equip item."
        );

        if (item_data.item_type == IItems.ItemType.TWO_HANDED) {
            if (character_equipments[_id][EquipmentSlot.RIGHT_HAND].equiped) {
                unequip(_id, EquipmentSlot.RIGHT_HAND);
            }
        }

        if (character_equipments[_id][_slot].equiped) {
            unequip(_id, _slot);
        }

        IERC1155(items).safeTransferFrom(
            msg.sender,
            address(this),
            _item_id,
            1,
            ""
        );

        character_equipments[_id][_slot].equiped = true;
        character_equipments[_id][_slot].id = _item_id;
        emit Equipped(_id, _slot, _item_id);
    }

    /**
     * @notice Removes an item from a character equipment slot.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     * @param _slot Slot to equip the item.
     */
    function unequip(
        bytes memory _id,
        EquipmentSlot _slot
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            character_equipments[_id][_slot].equiped,
            "Equipment: unequip() item slot not equiped."
        );

        uint256 item_id = character_equipments[_id][_slot].id;
        character_equipments[_id][_slot].equiped = false;
        character_equipments[_id][_slot].id = 0;

        IERC1155(items).safeTransferFrom(
            address(this),
            ICivilizations(civilizations).ownerOf(_id),
            item_id,
            1,
            ""
        );
        emit Unequipped(_id, _slot);
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to return the character slots and items attached.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    function getCharacterEquipment(
        bytes memory _id
    ) public view returns (CharacterEquipment memory) {
        return
            CharacterEquipment(
                character_equipments[_id][EquipmentSlot.HELMET],
                character_equipments[_id][EquipmentSlot.SHOULDER_GUARDS],
                character_equipments[_id][EquipmentSlot.ARM_GUARDS],
                character_equipments[_id][EquipmentSlot.HANDS],
                character_equipments[_id][EquipmentSlot.RING],
                character_equipments[_id][EquipmentSlot.NECKLACE],
                character_equipments[_id][EquipmentSlot.CHEST],
                character_equipments[_id][EquipmentSlot.LEGS],
                character_equipments[_id][EquipmentSlot.BELT],
                character_equipments[_id][EquipmentSlot.FEET],
                character_equipments[_id][EquipmentSlot.CAPE],
                character_equipments[_id][EquipmentSlot.LEFT_HAND],
                character_equipments[_id][EquipmentSlot.RIGHT_HAND]
            );
    }

    /**
     * @notice External function to return the character stats modifiers.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _modifiers   The total modifiers.
     */
    function getCharacterTotalStatsModifiers(
        bytes memory _id
    ) public view returns (IStats.BasicStats memory _modifiers) {
        IStats.BasicStats memory _additions;
        IStats.BasicStats memory _reductions;

        for (uint256 i = 0; i < 13; i++) {
            ItemEquiped memory _slot = character_equipments[_id][
                EquipmentSlot(i)
            ];
            if (_slot.equiped) {
                IItems.Item memory _item = IItems(items).getItem(_slot.id);

                _additions.might += _item.stats_modifiers.might;
                _additions.speed += _item.stats_modifiers.speed;
                _additions.intellect += _item.stats_modifiers.intellect;
                _reductions.might += _item.stats_modifiers.might_reducer;
                _reductions.speed += _item.stats_modifiers.speed_reducer;
                _reductions.intellect += _item
                    .stats_modifiers
                    .intellect_reducer;
            }
        }

        if (_reductions.might <= _additions.might) {
            _modifiers.might = _additions.might - _reductions.might;
        }
        if (_reductions.speed <= _additions.speed) {
            _modifiers.speed = _additions.speed - _reductions.speed;
        }
        if (_reductions.intellect <= _additions.intellect) {
            _modifiers.intellect = _additions.intellect - _reductions.intellect;
        }
    }

    /**
     * @notice External function to return the character total attributes modifiers.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _modifiers   The amount of modifiers.
     */
    function getCharacterTotalAttributes(
        bytes memory _id
    ) public view returns (IItems.BaseAttributes memory _modifiers) {
        IItems.BaseAttributes memory _additions;
        IItems.BaseAttributes memory _reductions;
        for (uint256 i = 0; i < 13; i++) {
            ItemEquiped memory _slot = character_equipments[_id][
                EquipmentSlot(i)
            ];

            if (_slot.equiped) {
                IItems.Item memory _item = IItems(items).getItem(_slot.id);

                _additions.atk += _item.attributes.atk;
                _additions.def += _item.attributes.def;
                _additions.range += _item.attributes.range;
                _additions.mag_atk += _item.attributes.mag_atk;
                _additions.mag_def += _item.attributes.mag_def;
                _additions.rate += _item.attributes.rate;

                _reductions.atk += _item.attributes.atk_reducer;
                _reductions.def += _item.attributes.def_reducer;
                _reductions.range += _item.attributes.range_reducer;
                _reductions.mag_atk += _item.attributes.mag_atk_reducer;
                _reductions.mag_def += _item.attributes.mag_def_reducer;
                _reductions.rate += _item.attributes.rate_reducer;
            }
        }

        if (_reductions.atk <= _additions.atk) {
            _modifiers.atk = _additions.atk - _reductions.atk;
        }

        if (_reductions.def <= _additions.def) {
            _modifiers.def = _additions.def - _reductions.def;
        }

        if (_reductions.range <= _additions.range) {
            _modifiers.range = _additions.range - _reductions.range;
        }

        if (_reductions.mag_atk <= _additions.mag_atk) {
            _modifiers.mag_atk = _additions.mag_atk - _reductions.mag_atk;
        }

        if (_reductions.mag_def <= _additions.mag_def) {
            _modifiers.mag_def = _additions.mag_def - _reductions.mag_def;
        }

        if (_reductions.rate <= _additions.rate) {
            _modifiers.rate = _additions.rate - _reductions.rate;
        }
    }

    // =============================================== Internal =======================================================

    /** @notice Internal function make sure upgrade proxy caller is the owner. */
    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyOwner {}
}
