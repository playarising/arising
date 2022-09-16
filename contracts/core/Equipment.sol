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

/**
 * @dev `Equipment` is the contract to equip gear for Arising characters.
 */

// TODO
contract Equipment is Ownable, ERC1155Holder, IEquipment, Pausable {
    // =============================================== Storage ========================================================

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Address of the `Experience` instance. **/
    address public experience;

    /** @dev Address of the `Items` instance. **/
    address public items;

    /** @dev Map to track the equipment of characters. **/
    mapping(bytes => mapping(EquipmentSlot => ItemEquiped)) character_equipments;

    // =============================================== Modifiers ======================================================

    /**
     * @dev Checks if `msg.sender` is owner or allowed to manipulate a composed ID.
     */
    modifier onlyAllowed(bytes memory id) {
        require(
            ICivilizations(civilizations).exists(id),
            "Equipment: can't get access to a non minted token."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, id),
            "Equipment: msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     * @param _experience       The address of the `Experience` instance.
     * @param _items       The address of the `Items` instance.
     */
    constructor(
        address _civilizations,
        address _experience,
        address _items
    ) {
        civilizations = _civilizations;
        experience = _experience;
        items = _items;
    }

    /** @dev Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @dev Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
    }

    /**
     * @dev Assigns an item to an item slot. If it is already used, it replaces it.
     * @param id            Composed ID of the character.
     * @param item_slot     Slot from the equipment to remove.
     * @param item_id       ID of the item to assign.
     */
    function unequip(
        bytes memory id,
        EquipmentSlot item_slot,
        uint256 item_id
    ) public onlyAllowed(id) {
        IItems.Item memory item_data = IItems(items).getItem(item_id);

        // TODO check item type to match the item slot

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
        onlyAllowed(id)
    {
        require(
            character_equipments[id][item_slot].equiped,
            "Equipment: item slot doesn't have any equiped item"
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
                character_equipments[id][EquipmentSlot.RINGS],
                character_equipments[id][EquipmentSlot.NECKLACE],
                character_equipments[id][EquipmentSlot.CHEST],
                character_equipments[id][EquipmentSlot.LEGS],
                character_equipments[id][EquipmentSlot.BELTS],
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
        returns (IItems.Stats memory, IItems.Stats memory)
    {
        IItems.Stats memory additions = IItems.Stats(0, 0, 0);
        IItems.Stats memory reductions = IItems.Stats(0, 0, 0);

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

    // =============================================== Internal =======================================================
}
