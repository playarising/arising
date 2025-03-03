// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

/**
 * @title IItems
 * @notice Interface for the [Items](/docs/items/Items.md) contract.
 */
interface IItems {
    /**
     * @notice Internal struct to store the item properties.
     *
     * Requirements:
     * @param id                The id of the item.
     * @param name              The name of the item.
     * @param description       The description of the item
     * @param level_required    The minimum level required to equip the item.
     * @param item_type         The item type to use for the equipment slot.
     * @param stat_modifiers    The modifiers that add or removes from the character pool.
     * @param attributes        The base item attributes.
     * @param available         Boolean to check if the item is available to be equiped.
     */
    struct Item {
        uint256 id;
        string name;
        string description;
        uint256 level_required;
        ItemType item_type;
        StatsModifiers stats_modifiers;
        Attributes attributes;
        bool available;
    }

    /** @notice Enum to define different item types. */
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

    /**
     * @notice Internal struct to store the item [IStats.BasicStats](/docs/interfaces/IStats.md#BasicStats) modifiers.
     *
     * Requirements:
     * @param might             The amount of might points added to the stats pool.
     * @param might_reducer     The amount of might points reduced to the stats pool.
     * @param speed             The amount of speed points added to the stats pool.
     * @param speed_reducer     The amount of speed points reduced to the stats pool.
     * @param intellect         The amount of intellect points added to the stats pool.
     * @param intellect_reducer The amount of intellect points reduced to the stats pool.
     */
    struct StatsModifiers {
        uint256 might;
        uint256 might_reducer;
        uint256 speed;
        uint256 speed_reducer;
        uint256 intellect;
        uint256 intellect_reducer;
    }

    /**
     * @notice Internal struct to store the base combat attributes of an item.
     *
     * Requirements:
     * @param atk       The amount of attack points of the item.
     * @param def       The amount of defence points of the item.
     * @param range     The amount of range points of the item.
     * @param mag_atk   The amount of magical attack points of the item.
     * @param mag_def   The amount of magical defence points of the item.
     * @param rate      The rate speed points of the item.
     */
    struct BaseAttributes {
        uint256 atk;
        uint256 def;
        uint256 range;
        uint256 mag_atk;
        uint256 mag_def;
        uint256 rate;
    }

    /**
     * @notice Internal struct to store the base combat attributes with reducers of an item.
     *
     * Requirements:
     * @param atk               The amount of attack points the item adds to the total combat points.
     * @param atk_reducer       The amount of attack points the item reduces to the total combat points.
     * @param def               The amount of defence points the item adds to the total combat points.
     * @param def_reducer       The amount of defence points the item reduces to the total combat points.
     * @param range             The amount of range points the item adds to the total combat points.
     * @param range_reducer     The amount of range points the item reduces to the total combat points.
     * @param mag_atk           The amount of magical attack points the item adds to the total combat points.
     * @param mag_atk_reducer   The amount of magical attack points the item reduces to the total combat points.
     * @param mag_def           The amount of magical defence points the item adds to the total combat points.
     * @param mag_def_reducer   The amount of magical defence points the item reduces to the total combat points.
     * @param rate              The amount of rate speed points the item adds to the total combat points.
     * @param rate_reducer      The amount of rate speed points the item reduces to the total combat points.
     */
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

    /** @notice See [Items#mint](/docs/items/Items.md#mint) */
    function mint(address _to, uint256 _item_id) external;

    /** @notice See [Items#burn](/docs/items/Items.md#burn) */
    function burn(address _from, uint256 _item_id) external;

    /** @notice See [Items#addAuthority](/docs/items/Items.md#addAuthority) */
    function addAuthority(address _authority) external;

    /** @notice See [Items#removeAuthority](/docs/items/Items.md#removeAuthority) */
    function removeAuthority(address _authority) external;

    /** @notice See [Items#addItem](/docs/items/Items.md#addItem) */
    function addItem(
        string memory _name,
        string memory _description,
        uint256 _level_required,
        ItemType _item_type,
        StatsModifiers memory _stats_modifiers,
        Attributes memory _attributes
    ) external;

    /** @notice See [Items#updateItem](/docs/items/Items.md#updateItem) */
    function updateItem(Item memory _item) external;

    /** @notice See [Items#disableItem](/docs/items/Items.md#disableItem) */
    function disableItem(uint256 _item_id) external;

    /** @notice See [Items#enableItem](/docs/items/Items.md#enableItem) */
    function enableItem(uint256 _item_id) external;

    /** @notice See [Items#getItem](/docs/items/Items.md#getItem) */
    function getItem(
        uint256 _item_id
    ) external view returns (Item memory _item);
}
