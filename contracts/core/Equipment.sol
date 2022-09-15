// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/IEquipment.sol";

/**
 * @dev `Equipment` is the contract to equip gear for Arising characters.
 */

// TODO
contract Equipment is Ownable, IEquipment {
    // =============================================== Storage ========================================================

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Address of the `Experience` instance. **/
    address public experience;

    /** @dev Map to track the equipment of characters. **/
    mapping(bytes => CharacterEquipment) character_equipments;

    /** @dev Map to track all the available items. **/
    mapping(uint256 => Item) items;

    /** @dev Array to track all the available items. **/
    uint256[] _items;

    // =============================================== Modifiers ======================================================

    /**
     * @dev Checks if `msg.sender` is owner or allowed to manipulate a composed ID.
     */
    modifier onlyAllowed(bytes memory id) {
        require(
            ICivilizations(civilizations).exists(id),
            "Stats: can't get access to a non minted token."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, id),
            "Stats: msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     * @param _experience       The address of the `Experience` instance.
     */
    constructor(address _civilizations, address _experience) {
        civilizations = _civilizations;
        experience = _experience;
    }

    /**
     * @dev Adds a new recipe to the forge.
     * @param external_id       The ID of the item on the ERC1155 token.
     * @param level_required    Minimum level required to equip.
     * @param item_type         Enum number of the item type.
     * @param stat_modifiers    Stats modifiers with reducers.
     * @param attributes        Item attributes with reducers.
     */
    function addItem(
        uint256 external_id,
        uint256 level_required,
        ItemType item_type,
        StatsModifiers memory stat_modifiers,
        Attributes memory attributes
    ) public onlyOwner {
        uint256 id = _items.length + 1;
        items[id] = Item(
            id,
            external_id,
            level_required,
            item_type,
            stat_modifiers,
            attributes,
            true
        );
        _items.push(id);
    }

    /**
     * @dev Disables an item to be equiped recipe.
     * @param id  ID of the item.
     */
    function disableItem(uint256 id) public onlyOwner {
        require(
            id != 0 && id <= _items.length,
            "Equipment: item id doesn't exist."
        );
        items[id].available = false;
    }

    /**
     * @dev Enables an item to be equiped recipe.
     * @param id  ID of the item.
     */
    function enableItem(uint256 id) public onlyOwner {
        require(
            id != 0 && id <= _items.length,
            "Equipment: item id doesn't exist."
        );
        items[id].available = true;
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
        return character_equipments[id];
    }

    /** @dev Returns the information of an item.
     *  @param id  ID of the item.
     */
    function getItem(uint256 id) public view returns (Item memory) {
        return items[id];
    }

    /** @dev Returns the total modifiers from the equipment.
     *  @param id  Composed ID of the token.
     */
    function getCharacterTotalStatsModifiers(bytes memory id)
        public
        view
        returns (Stats memory, Stats memory)
    {
        CharacterEquipment memory e = character_equipments[id];
        Stats memory additions = Stats(0, 0, 0);
        Stats memory reductions = Stats(0, 0, 0);
        if (e.helmet.equiped) {
            additions.might += items[e.helmet.id].stat_modifiers.might;
            additions.speed += items[e.helmet.id].stat_modifiers.speed;
            additions.intellect += items[e.helmet.id].stat_modifiers.intellect;

            reductions.might += items[e.helmet.id].stat_modifiers.might_reducer;
            reductions.speed += items[e.helmet.id].stat_modifiers.speed_reducer;
            reductions.intellect += items[e.helmet.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.shoulder_guards.equiped) {
            additions.might += items[e.shoulder_guards.id].stat_modifiers.might;
            additions.speed += items[e.shoulder_guards.id].stat_modifiers.speed;
            additions.intellect += items[e.shoulder_guards.id]
                .stat_modifiers
                .intellect;

            reductions.might += items[e.shoulder_guards.id]
                .stat_modifiers
                .might_reducer;
            reductions.speed += items[e.shoulder_guards.id]
                .stat_modifiers
                .speed_reducer;
            reductions.intellect += items[e.shoulder_guards.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.arm_guards.equiped) {
            additions.might += items[e.arm_guards.id].stat_modifiers.might;
            additions.speed += items[e.arm_guards.id].stat_modifiers.speed;
            additions.intellect += items[e.arm_guards.id]
                .stat_modifiers
                .intellect;

            reductions.might += items[e.arm_guards.id]
                .stat_modifiers
                .might_reducer;
            reductions.speed += items[e.arm_guards.id]
                .stat_modifiers
                .speed_reducer;
            reductions.intellect += items[e.arm_guards.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.hands.equiped) {
            additions.might += items[e.hands.id].stat_modifiers.might;
            additions.speed += items[e.hands.id].stat_modifiers.speed;
            additions.intellect += items[e.hands.id].stat_modifiers.intellect;

            reductions.might += items[e.hands.id].stat_modifiers.might_reducer;
            reductions.speed += items[e.hands.id].stat_modifiers.speed_reducer;
            reductions.intellect += items[e.hands.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.rings.equiped) {
            additions.might += items[e.rings.id].stat_modifiers.might;
            additions.speed += items[e.rings.id].stat_modifiers.speed;
            additions.intellect += items[e.rings.id].stat_modifiers.intellect;

            reductions.might += items[e.rings.id].stat_modifiers.might_reducer;
            reductions.speed += items[e.rings.id].stat_modifiers.speed_reducer;
            reductions.intellect += items[e.rings.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.necklace.equiped) {
            additions.might += items[e.necklace.id].stat_modifiers.might;
            additions.speed += items[e.necklace.id].stat_modifiers.speed;
            additions.intellect += items[e.necklace.id]
                .stat_modifiers
                .intellect;

            reductions.might += items[e.necklace.id]
                .stat_modifiers
                .might_reducer;
            reductions.speed += items[e.necklace.id]
                .stat_modifiers
                .speed_reducer;
            reductions.intellect += items[e.necklace.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.chest.equiped) {
            additions.might += items[e.chest.id].stat_modifiers.might;
            additions.speed += items[e.chest.id].stat_modifiers.speed;
            additions.intellect += items[e.chest.id].stat_modifiers.intellect;

            reductions.might += items[e.chest.id].stat_modifiers.might_reducer;
            reductions.speed += items[e.chest.id].stat_modifiers.speed_reducer;
            reductions.intellect += items[e.chest.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.legs.equiped) {
            additions.might += items[e.legs.id].stat_modifiers.might;
            additions.speed += items[e.legs.id].stat_modifiers.speed;
            additions.intellect += items[e.legs.id].stat_modifiers.intellect;

            reductions.might += items[e.legs.id].stat_modifiers.might_reducer;
            reductions.speed += items[e.legs.id].stat_modifiers.speed_reducer;
            reductions.intellect += items[e.legs.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.belt.equiped) {
            additions.might += items[e.belt.id].stat_modifiers.might;
            additions.speed += items[e.belt.id].stat_modifiers.speed;
            additions.intellect += items[e.belt.id].stat_modifiers.intellect;

            reductions.might += items[e.belt.id].stat_modifiers.might_reducer;
            reductions.speed += items[e.belt.id].stat_modifiers.speed_reducer;
            reductions.intellect += items[e.belt.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.feet.equiped) {
            additions.might += items[e.feet.id].stat_modifiers.might;
            additions.speed += items[e.feet.id].stat_modifiers.speed;
            additions.intellect += items[e.feet.id].stat_modifiers.intellect;

            reductions.might += items[e.feet.id].stat_modifiers.might_reducer;
            reductions.speed += items[e.feet.id].stat_modifiers.speed_reducer;
            reductions.intellect += items[e.feet.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.cape.equiped) {
            additions.might += items[e.cape.id].stat_modifiers.might;
            additions.speed += items[e.cape.id].stat_modifiers.speed;
            additions.intellect += items[e.cape.id].stat_modifiers.intellect;

            reductions.might += items[e.cape.id].stat_modifiers.might_reducer;
            reductions.speed += items[e.cape.id].stat_modifiers.speed_reducer;
            reductions.intellect += items[e.cape.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.left_hand.equiped) {
            additions.might += items[e.left_hand.id].stat_modifiers.might;
            additions.speed += items[e.left_hand.id].stat_modifiers.speed;
            additions.intellect += items[e.left_hand.id]
                .stat_modifiers
                .intellect;

            reductions.might += items[e.left_hand.id]
                .stat_modifiers
                .might_reducer;
            reductions.speed += items[e.left_hand.id]
                .stat_modifiers
                .speed_reducer;
            reductions.intellect += items[e.left_hand.id]
                .stat_modifiers
                .intellect_reducer;
        }

        if (e.right_hand.equiped) {
            additions.might += items[e.right_hand.id].stat_modifiers.might;
            additions.speed += items[e.right_hand.id].stat_modifiers.speed;
            additions.intellect += items[e.right_hand.id]
                .stat_modifiers
                .intellect;

            reductions.might += items[e.right_hand.id]
                .stat_modifiers
                .might_reducer;
            reductions.speed += items[e.right_hand.id]
                .stat_modifiers
                .speed_reducer;
            reductions.intellect += items[e.right_hand.id]
                .stat_modifiers
                .intellect_reducer;
        }

        return (additions, reductions);
    }

    /** @dev Returns the total attributes from the equipment.
     *  @param id  Composed ID of the token.
     */
    function getCharacterTotalAttributes(bytes memory id)
        public
        view
        returns (BaseAttributes memory, BaseAttributes memory)
    {
        CharacterEquipment memory e = character_equipments[id];
        BaseAttributes memory additions = BaseAttributes(0, 0, 0, 0, 0, 0);
        BaseAttributes memory reductions = BaseAttributes(0, 0, 0, 0, 0, 0);

        if (e.helmet.equiped) {
            additions.atk += items[e.helmet.id].attributes.atk;
            additions.def += items[e.helmet.id].attributes.def;
            additions.range += items[e.helmet.id].attributes.range;
            additions.mag_atk += items[e.helmet.id].attributes.mag_atk;
            additions.mag_def += items[e.helmet.id].attributes.mag_def;
            additions.rate += items[e.helmet.id].attributes.rate;

            reductions.atk += items[e.helmet.id].attributes.atk_reducer;
            reductions.def += items[e.helmet.id].attributes.def_reducer;
            reductions.range += items[e.helmet.id].attributes.range_reducer;
            reductions.mag_atk += items[e.helmet.id].attributes.mag_atk_reducer;
            reductions.mag_def += items[e.helmet.id].attributes.mag_def_reducer;
            reductions.rate += items[e.helmet.id].attributes.rate_reducer;
        }

        if (e.shoulder_guards.equiped) {
            additions.atk += items[e.shoulder_guards.id].attributes.atk;
            additions.def += items[e.shoulder_guards.id].attributes.def;
            additions.range += items[e.shoulder_guards.id].attributes.range;
            additions.mag_atk += items[e.shoulder_guards.id].attributes.mag_atk;
            additions.mag_def += items[e.shoulder_guards.id].attributes.mag_def;
            additions.rate += items[e.shoulder_guards.id].attributes.rate;

            reductions.atk += items[e.shoulder_guards.id]
                .attributes
                .atk_reducer;
            reductions.def += items[e.shoulder_guards.id]
                .attributes
                .def_reducer;
            reductions.range += items[e.shoulder_guards.id]
                .attributes
                .range_reducer;
            reductions.mag_atk += items[e.shoulder_guards.id]
                .attributes
                .mag_atk_reducer;
            reductions.mag_def += items[e.shoulder_guards.id]
                .attributes
                .mag_def_reducer;
            reductions.rate += items[e.shoulder_guards.id]
                .attributes
                .rate_reducer;
        }

        if (e.arm_guards.equiped) {
            additions.atk += items[e.arm_guards.id].attributes.atk;
            additions.def += items[e.arm_guards.id].attributes.def;
            additions.range += items[e.arm_guards.id].attributes.range;
            additions.mag_atk += items[e.arm_guards.id].attributes.mag_atk;
            additions.mag_def += items[e.arm_guards.id].attributes.mag_def;
            additions.rate += items[e.arm_guards.id].attributes.rate;

            reductions.atk += items[e.arm_guards.id].attributes.atk_reducer;
            reductions.def += items[e.arm_guards.id].attributes.def_reducer;
            reductions.range += items[e.arm_guards.id].attributes.range_reducer;
            reductions.mag_atk += items[e.arm_guards.id]
                .attributes
                .mag_atk_reducer;
            reductions.mag_def += items[e.arm_guards.id]
                .attributes
                .mag_def_reducer;
            reductions.rate += items[e.arm_guards.id].attributes.rate_reducer;
        }

        if (e.hands.equiped) {
            additions.atk += items[e.hands.id].attributes.atk;
            additions.def += items[e.hands.id].attributes.def;
            additions.range += items[e.hands.id].attributes.range;
            additions.mag_atk += items[e.hands.id].attributes.mag_atk;
            additions.mag_def += items[e.hands.id].attributes.mag_def;
            additions.rate += items[e.hands.id].attributes.rate;

            reductions.atk += items[e.hands.id].attributes.atk_reducer;
            reductions.def += items[e.hands.id].attributes.def_reducer;
            reductions.range += items[e.hands.id].attributes.range_reducer;
            reductions.mag_atk += items[e.hands.id].attributes.mag_atk_reducer;
            reductions.mag_def += items[e.hands.id].attributes.mag_def_reducer;
            reductions.rate += items[e.hands.id].attributes.rate_reducer;
        }

        if (e.rings.equiped) {
            additions.atk += items[e.rings.id].attributes.atk;
            additions.def += items[e.rings.id].attributes.def;
            additions.range += items[e.rings.id].attributes.range;
            additions.mag_atk += items[e.rings.id].attributes.mag_atk;
            additions.mag_def += items[e.rings.id].attributes.mag_def;
            additions.rate += items[e.rings.id].attributes.rate;

            reductions.atk += items[e.rings.id].attributes.atk_reducer;
            reductions.def += items[e.rings.id].attributes.def_reducer;
            reductions.range += items[e.rings.id].attributes.range_reducer;
            reductions.mag_atk += items[e.rings.id].attributes.mag_atk_reducer;
            reductions.mag_def += items[e.rings.id].attributes.mag_def_reducer;
            reductions.rate += items[e.rings.id].attributes.rate_reducer;
        }

        if (e.necklace.equiped) {
            additions.atk += items[e.necklace.id].attributes.atk;
            additions.def += items[e.necklace.id].attributes.def;
            additions.range += items[e.necklace.id].attributes.range;
            additions.mag_atk += items[e.necklace.id].attributes.mag_atk;
            additions.mag_def += items[e.necklace.id].attributes.mag_def;
            additions.rate += items[e.necklace.id].attributes.rate;

            reductions.atk += items[e.necklace.id].attributes.atk_reducer;
            reductions.def += items[e.necklace.id].attributes.def_reducer;
            reductions.range += items[e.necklace.id].attributes.range_reducer;
            reductions.mag_atk += items[e.necklace.id]
                .attributes
                .mag_atk_reducer;
            reductions.mag_def += items[e.necklace.id]
                .attributes
                .mag_def_reducer;
            reductions.rate += items[e.necklace.id].attributes.rate_reducer;
        }

        if (e.chest.equiped) {
            additions.atk += items[e.chest.id].attributes.atk;
            additions.def += items[e.chest.id].attributes.def;
            additions.range += items[e.chest.id].attributes.range;
            additions.mag_atk += items[e.chest.id].attributes.mag_atk;
            additions.mag_def += items[e.chest.id].attributes.mag_def;
            additions.rate += items[e.chest.id].attributes.rate;

            reductions.atk += items[e.chest.id].attributes.atk_reducer;
            reductions.def += items[e.chest.id].attributes.def_reducer;
            reductions.range += items[e.chest.id].attributes.range_reducer;
            reductions.mag_atk += items[e.chest.id].attributes.mag_atk_reducer;
            reductions.mag_def += items[e.chest.id].attributes.mag_def_reducer;
            reductions.rate += items[e.chest.id].attributes.rate_reducer;
        }

        if (e.legs.equiped) {
            additions.atk += items[e.legs.id].attributes.atk;
            additions.def += items[e.legs.id].attributes.def;
            additions.range += items[e.legs.id].attributes.range;
            additions.mag_atk += items[e.legs.id].attributes.mag_atk;
            additions.mag_def += items[e.legs.id].attributes.mag_def;
            additions.rate += items[e.legs.id].attributes.rate;

            reductions.atk += items[e.legs.id].attributes.atk_reducer;
            reductions.def += items[e.legs.id].attributes.def_reducer;
            reductions.range += items[e.legs.id].attributes.range_reducer;
            reductions.mag_atk += items[e.legs.id].attributes.mag_atk_reducer;
            reductions.mag_def += items[e.legs.id].attributes.mag_def_reducer;
            reductions.rate += items[e.legs.id].attributes.rate_reducer;
        }

        if (e.belt.equiped) {
            additions.atk += items[e.belt.id].attributes.atk;
            additions.def += items[e.belt.id].attributes.def;
            additions.range += items[e.belt.id].attributes.range;
            additions.mag_atk += items[e.belt.id].attributes.mag_atk;
            additions.mag_def += items[e.belt.id].attributes.mag_def;
            additions.rate += items[e.belt.id].attributes.rate;

            reductions.atk += items[e.belt.id].attributes.atk_reducer;
            reductions.def += items[e.belt.id].attributes.def_reducer;
            reductions.range += items[e.belt.id].attributes.range_reducer;
            reductions.mag_atk += items[e.belt.id].attributes.mag_atk_reducer;
            reductions.mag_def += items[e.belt.id].attributes.mag_def_reducer;
            reductions.rate += items[e.belt.id].attributes.rate_reducer;
        }

        if (e.feet.equiped) {
            additions.atk += items[e.feet.id].attributes.atk;
            additions.def += items[e.feet.id].attributes.def;
            additions.range += items[e.feet.id].attributes.range;
            additions.mag_atk += items[e.feet.id].attributes.mag_atk;
            additions.mag_def += items[e.feet.id].attributes.mag_def;
            additions.rate += items[e.feet.id].attributes.rate;

            reductions.atk += items[e.feet.id].attributes.atk_reducer;
            reductions.def += items[e.feet.id].attributes.def_reducer;
            reductions.range += items[e.feet.id].attributes.range_reducer;
            reductions.mag_atk += items[e.feet.id].attributes.mag_atk_reducer;
            reductions.mag_def += items[e.feet.id].attributes.mag_def_reducer;
            reductions.rate += items[e.feet.id].attributes.rate_reducer;
        }

        if (e.cape.equiped) {
            additions.atk += items[e.cape.id].attributes.atk;
            additions.def += items[e.cape.id].attributes.def;
            additions.range += items[e.cape.id].attributes.range;
            additions.mag_atk += items[e.cape.id].attributes.mag_atk;
            additions.mag_def += items[e.cape.id].attributes.mag_def;
            additions.rate += items[e.cape.id].attributes.rate;

            reductions.atk += items[e.cape.id].attributes.atk_reducer;
            reductions.def += items[e.cape.id].attributes.def_reducer;
            reductions.range += items[e.cape.id].attributes.range_reducer;
            reductions.mag_atk += items[e.cape.id].attributes.mag_atk_reducer;
            reductions.mag_def += items[e.cape.id].attributes.mag_def_reducer;
            reductions.rate += items[e.cape.id].attributes.rate_reducer;
        }

        if (e.left_hand.equiped) {
            additions.atk += items[e.left_hand.id].attributes.atk;
            additions.def += items[e.left_hand.id].attributes.def;
            additions.range += items[e.left_hand.id].attributes.range;
            additions.mag_atk += items[e.left_hand.id].attributes.mag_atk;
            additions.mag_def += items[e.left_hand.id].attributes.mag_def;
            additions.rate += items[e.left_hand.id].attributes.rate;

            reductions.atk += items[e.left_hand.id].attributes.atk_reducer;
            reductions.def += items[e.left_hand.id].attributes.def_reducer;
            reductions.range += items[e.left_hand.id].attributes.range_reducer;
            reductions.mag_atk += items[e.left_hand.id]
                .attributes
                .mag_atk_reducer;
            reductions.mag_def += items[e.left_hand.id]
                .attributes
                .mag_def_reducer;
            reductions.rate += items[e.left_hand.id].attributes.rate_reducer;
        }

        if (e.right_hand.equiped) {
            additions.atk += items[e.right_hand.id].attributes.atk;
            additions.def += items[e.right_hand.id].attributes.def;
            additions.range += items[e.right_hand.id].attributes.range;
            additions.mag_atk += items[e.right_hand.id].attributes.mag_atk;
            additions.mag_def += items[e.right_hand.id].attributes.mag_def;
            additions.rate += items[e.right_hand.id].attributes.rate;

            reductions.atk += items[e.right_hand.id].attributes.atk_reducer;
            reductions.def += items[e.right_hand.id].attributes.def_reducer;
            reductions.range += items[e.right_hand.id].attributes.range_reducer;
            reductions.mag_atk += items[e.right_hand.id]
                .attributes
                .mag_atk_reducer;
            reductions.mag_def += items[e.right_hand.id]
                .attributes
                .mag_def_reducer;
            reductions.rate += items[e.right_hand.id].attributes.rate_reducer;
        }

        return (additions, reductions);
    }

    // =============================================== Internal =======================================================
}
