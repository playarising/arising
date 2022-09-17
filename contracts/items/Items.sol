// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";

import "../interfaces/IItems.sol";

/**
 * @title Items
 * @notice This contract is an standard `ERC1155` implementation with internal mappings to store items additional
 * information for the characters usage.
 *
 * @dev Implementation of the [IItems](/docs/interfaces/IItems.md) interface.
 */
contract Items is IItems, Ownable, ERC1155 {
    // =============================================== Storage ========================================================

    /** @notice Map to track the extra items data. */
    mapping(uint256 => Item) items;

    /** @notice Array to track a full list of item IDs. */
    uint256[] _items;

    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     */
    constructor() ERC1155("https://items.playarising.com/{id}/") {}

    /**
     * @notice Creates tokens to the address provided.
     *
     * Requirements:
     * @param _to    Address that receives the tokens.
     * @param _id    ID of the item to be created.
     */
    function mint(address _to, uint256 _id) public onlyOwner {
        _mint(_to, _id, 1, "");
    }

    /**
     * @notice Adds the item data to relate with a specific token ID.
     *
     * Requirements:
     * @param _level_required   Minimum level for a character to use the item.
     * @param _item_type        Type of the item defined by the enum [ItemType](/docs/interfaces/IItems.md#itemtype).
     * @param _stats_modifiers  Item modifiers for the character stats.
     * @param _attributes       Specific item attributes.
     */
    function addItem(
        uint256 _level_required,
        ItemType _item_type,
        StatsModifiers memory _stats_modifiers,
        Attributes memory _attributes
    ) public onlyOwner {
        uint256 new_id = _items.length + 1;
        items[new_id] = Item(
            new_id,
            _level_required,
            _item_type,
            _stats_modifiers,
            _attributes,
            true
        );
        _items.push(new_id);
    }

    /**
     * @notice Disables an item from beign equiped.
     *
     * Requirements:
     * @param _id   ID of the item.
     */
    function disableItem(uint256 _id) public onlyOwner {
        require(
            _id != 0 && _id <= _items.length,
            "Items: item id doesn't exist."
        );
        items[_id].available = false;
    }

    /**
     * @notice Enables an item to be equiped.
     *
     * Requirements:
     * @param _id   ID of the item.
     */
    function enableItem(uint256 _id) public onlyOwner {
        require(
            _id != 0 && _id <= _items.length,
            "Equipment: item id doesn't exist."
        );
        items[_id].available = true;
    }

    // =============================================== Getters ========================================================

    /**
     * @notice Returns the full information of an item.
     *
     * Requirements:
     * @param _id       ID of the item.
     *
     * @return _item    Full item information.
     */
    function getItem(uint256 _id) public view returns (Item memory _item) {
        return items[_id];
    }
}
