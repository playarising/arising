// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "../interfaces/IBaseERC1155.sol";

/**
 * @dev `BaseERC1155` is the base ERC1155 token for Arising civilizations gear and equipable items.
 */

contract BaseERC1155 is Ownable, IBaseERC1155, ERC1155 {
    // =============================================== Storage ========================================================

    /** @dev Map to track all the available items. **/
    mapping(uint256 => Item) items;

    /** @dev Array to track all the available items. **/
    uint256[] _items;

    /** @dev Boolean to check if the implementation is usable. **/
    bool public initialized;

    // ============================================== Modifiers =======================================================

    /**
     * @dev Checks if `initialized` is enabled.
     */
    modifier onlyInitialized() {
        require(initialized, "Civilizations: contract is not initialized");
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     */
    constructor() ERC1155("https://items.playarising.com/{id}/") {
        initialized = false;
    }

    /**
     * @dev Enables or disables the `BaseERC1155` implementation.
     * @param _init       Enable or disable the instance
     */
    function setInitialized(bool _init) public onlyOwner {
        initialized = _init;
    }

    /**
     * @dev Creates a new item.
     * @param to       The address of the receiver
     * @param id       The token and item ID
     */
    function mint(address to, uint256 id) external onlyInitialized onlyOwner {
        _mint(to, id, 1, "");
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

    /** @dev Returns the information of an item.
     *  @param id  ID of the item.
     */
    function getItem(uint256 id) public view returns (Item memory) {
        return items[id];
    }
    // =============================================== Internal =======================================================
}
