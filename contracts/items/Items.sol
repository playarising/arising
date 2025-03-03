// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC1155/ERC1155Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "../interfaces/IItems.sol";

/**
 * @title Items
 * @notice This contract is an standard `ERC1155` implementation with internal mappings to store items additional
 * information for the characters usage.
 *
 * @notice Implementation of the [IItems](/docs/interfaces/IItems.md) interface.
 */
contract Items is
    IItems,
    ERC1155Upgradeable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    // =============================================== Storage ========================================================

    /** @notice Map to track the extra items data. */
    mapping(uint256 => Item) items;

    /** @notice Array to track a full list of item IDs. */
    uint256[] _items;

    /** @notice Map to store the list of authorized addresses to mint tokens. */
    mapping(address => bool) authorized;

    // =============================================== Modifiers ======================================================

    /** @notice Checks if the `msg.sender` is authorized to mint items. */
    modifier onlyAuthorized() {
        require(
            authorized[msg.sender],
            "Items: onlyAuthorized() msg.sender not authorized."
        );
        _;
    }

    // =============================================== Events =========================================================

    /**
     * @notice Event emmited when the [addItem](#addItem) function is called.
     *
     * Requirements:
     * @param _item_id    ID of the item added.
     * @param _name         Name of the recipe.
     * @param _description  Recipe description
     */
    event AddItem(uint256 _item_id, string _name, string _description);

    /**
     * @notice Event emmited when the [updateItem](#updateItem) function is called.
     *
     * Requirements:
     * @param _item_id    ID of the item added.
     * @param _name         Name of the recipe.
     * @param _description  Recipe description
     */
    event ItemUpdate(uint256 _item_id, string _name, string _description);

    /**
     * @notice Event emmited when the [enableItem](#enableItem) function is called.
     *
     * Requirements:
     * @param _item_id    ID of the item enabled.
     */
    event EnableItem(uint256 _item_id);

    /**
     * @notice Event emmited when the [disableItem](#disableItem) function is called.
     *
     * Requirements:
     * @param _item_id    ID of the the item disabled.
     */
    event DisableItem(uint256 _item_id);

    // =============================================== Setters ========================================================

    /**
     * @notice Initializer.
     */
    function initialize() public initializer {
        __Ownable_init();
        __UUPSUpgradeable_init();

        __ERC1155_init("https://items.playarising.com/{id}");
        authorized[msg.sender] = true;
    }

    /**
     * @notice Creates tokens to the address provided.
     *
     * Requirements:
     * @param _to           Address that receives the tokens.
     * @param _item_id      ID of the item to be created.
     */
    function mint(address _to, uint256 _item_id) public onlyAuthorized {
        require(
            _item_id != 0 && _item_id <= _items.length,
            "Items: mint() invalid item id."
        );
        _mint(_to, _item_id, 1, "");
    }

    /**
     * @notice Removes tokens from the address provided.
     *
     * Requirements:
     * @param _from         Address that receives the tokens.
     * @param _item_id      ID of the item to be created.
     */
    function burn(address _from, uint256 _item_id) public onlyAuthorized {
        require(
            _item_id != 0 && _item_id <= _items.length,
            "Items: burn() invalid item id."
        );
        _burn(_from, _item_id, 1);
    }

    /**
     * @notice Assigns a new address as an authority to mint items.
     *
     * Requirements:
     * @param _authority    Address to give authority.
     */
    function addAuthority(address _authority) public onlyOwner {
        authorized[_authority] = true;
    }

    /**
     * @notice Removes an authority to mint items.
     *
     * Requirements:
     * @param _authority    Address to give authority.
     */
    function removeAuthority(address _authority) public onlyOwner {
        require(
            authorized[_authority],
            "Items: removeAuthority() address is not authorized."
        );
        authorized[_authority] = false;
    }

    /**
     * @notice Adds the item data to relate with a specific token ID.
     *
     * Requirements:
     * @param _name             The item name.
     * @param _description      The item description.
     * @param _level_required   Minimum level for a character to use the item.
     * @param _item_type        Type of the item defined by the enum [ItemType](/docs/interfaces/IItems.md#itemtype).
     * @param _stats_modifiers  Item modifiers for the character stats.
     * @param _attributes       Specific item attributes.
     */
    function addItem(
        string memory _name,
        string memory _description,
        uint256 _level_required,
        ItemType _item_type,
        StatsModifiers memory _stats_modifiers,
        Attributes memory _attributes
    ) public onlyOwner {
        uint256 _item_id = _items.length + 1;
        items[_item_id] = Item(
            _item_id,
            _name,
            _description,
            _level_required,
            _item_type,
            _stats_modifiers,
            _attributes,
            true
        );
        _items.push(_item_id);
        emit AddItem(_item_id, _name, _description);
    }

    /**
     * @notice Updates a previously added item.
     *
     * Requirements:
     * @param _item   Full information of the item.
     */
    function updateItem(Item memory _item) public onlyOwner {
        require(
            _item.id != 0 && _item.id <= _items.length,
            "Items: updateItem() invalid item id."
        );
        items[_item.id] = _item;
        emit ItemUpdate(_item.id, _item.name, _item.description);
    }

    /**
     * @notice Disables an item from beign equiped.
     *
     * Requirements:
     * @param _item_id   ID of the item.
     */
    function disableItem(uint256 _item_id) public onlyOwner {
        require(
            _item_id != 0 && _item_id <= _items.length,
            "Items: disableItem() invalid item it."
        );
        items[_item_id].available = false;
        emit DisableItem(_item_id);
    }

    /**
     * @notice Enables an item to be equiped.
     *
     * Requirements:
     * @param _item_id   ID of the item.
     */
    function enableItem(uint256 _item_id) public onlyOwner {
        require(
            _item_id != 0 && _item_id <= _items.length,
            "Items: enableItem() invalid item it."
        );
        items[_item_id].available = true;
        emit EnableItem(_item_id);
    }

    // =============================================== Getters ========================================================

    /**
     * @notice Returns the full information of an item.
     *
     * Requirements:
     * @param _item_id       ID of the item.
     *
     * @return _item    Full item information.
     */
    function getItem(uint256 _item_id) public view returns (Item memory _item) {
        require(
            _item_id != 0 && _item_id <= _items.length,
            "Items: getItem() invalid item it."
        );
        return items[_item_id];
    }

    // =============================================== Internal =======================================================

    /** @notice Internal function make sure upgrade proxy caller is the owner. */
    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyOwner {}
}
