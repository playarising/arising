// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/IBaseERC721.sol";

/**
 * @dev `BaseFungibleItems` is a base contract to imitate the ERC20 functionality in the context of characters.
 */
contract BaseFungibleItems is Ownable {
    // =============================================== Storage ========================================================

    /** @dev Name of the item. **/
    string name;

    /** @dev Symbol of the item. **/
    string symbol;

    /** @dev Balances. **/
    mapping(uint256 => mapping(uint256 => uint256)) balances;

    /** @dev Map to store the civilizations implemented. **/
    mapping(uint256 => address) _civilizations;

    /** @dev Array to store the civilizations implemented. **/
    uint256[] civilizations;

    /** @dev Address of the allowed minter. **/
    address minter;

    // ============================================== Modifiers =======================================================
    /**
     * @dev Checks if `msg.sender` is a minter.
     */
    modifier onlyMinter() {
        require(
            msg.sender == minter,
            "BaseFungibleItems: msg.sender is not a minter"
        );
        _;
    }

    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _name      The name of fungible token.
     * @param _symbol    The symbol of the fungible token.
     * @param _minter    The address of the authorized minter.
     */
    constructor(
        string memory _name,
        string memory _symbol,
        address _minter
    ) {
        name = _name;
        symbol = _symbol;
        minter = _minter;
    }

    /** @dev Adds a civilization to the fungible item.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function addCivilization(address _instance) public onlyOwner {
        require(
            _instance != address(0),
            "BaseFungibleItems: instance address is null"
        );
        uint256 newId = civilizations.length + 1;
        _civilizations[newId] = _instance;
        civilizations.push(newId);
    }

    /** @dev Sets the `minter` address.
     *  @param _minter   address of the authorized minter.
     */
    function setMinter(address _minter) public onlyOwner {
        minter = _minter;
    }

    /** @dev Mints an specific amount of items to a character balance.
     *  @param civilization     ID of the civilization.
     *  @param id               ID of the token.
     *  @param amount           Amount to be minted.
     */
    function mintTo(
        uint256 civilization,
        uint256 id,
        uint256 amount
    ) public onlyMinter {
        require(
            _civilizations[civilization] != address(0),
            "BaseFungibleItems: civilization doesn't exist"
        );
        balances[civilization][id] += amount;
    }

    /** @dev Mints an specific amount of items to a character balance.
     *  @param civilization     ID of the civilization.
     *  @param id               ID of the token.
     *  @param amount           Amount to be consumed.
     */
    function consume(
        uint256 civilization,
        uint256 id,
        uint256 amount
    ) public {
        require(
            _civilizations[civilization] != address(0),
            "BaseFungibleItems: civilization doesn't exist"
        );
        require(
            IBaseERC721(_civilizations[civilization]).isApprovedOrOwner(
                msg.sender,
                id
            ),
            "StatsManager: interaction is not from owner or allowed"
        );
        balances[civilization][id] -= amount;
    }

    // =============================================== Getters ========================================================
    /** @dev Returns the balance of the item owned by a character.
     *  @param civilization     ID of the civilization.
     *  @param id               ID of the token.
     */
    function balanceOf(uint256 civilization, uint256 id)
        public
        view
        returns (uint256)
    {
        require(
            _civilizations[civilization] != address(0),
            "BaseFungibleItems: civilization doesn't exist"
        );
        return balances[civilization][id];
    }
}
