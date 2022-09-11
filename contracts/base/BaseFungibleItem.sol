// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/ICivilizations.sol";
import "../interfaces/IBaseFungibleItem.sol";

/**
 * @dev `BaseFungibleItem` is a base contract to imitate the ERC20 functionality in the context of characters.
 */
contract BaseFungibleItem is Ownable, IBaseFungibleItem {
    // =============================================== Storage ========================================================

    /** @dev Name of the item. **/
    string public name;

    /** @dev Symbol of the item. **/
    string public symbol;

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Balances. **/
    mapping(bytes => uint256) balances;

    // =============================================== Modifiers ======================================================

    /**
     * @dev Checks if `msg.sender` is owner or allowed to manipulate a composed ID.
     */
    modifier onlyAllowed(bytes memory id) {
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, id),
            "BaseFungibleItem: require consumer to be owner or have allowance"
        );
        _;
    }

    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _name             The name of fungible token.
     * @param _symbol           The symbol of the fungible token.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(
        string memory _name,
        string memory _symbol,
        address _civilizations
    ) {
        name = _name;
        symbol = _symbol;
        civilizations = _civilizations;
    }

    /** @dev Mints an specific amount of items to a character balance.
     *  @param id       Composed ID of the token.
     *  @param amount   Amount to be minted.
     */
    function mintTo(bytes memory id, uint256 amount) public onlyOwner {
        balances[id] += amount;
    }

    /** @dev Mints an specific amount of items to a character balance.
     *  @param id       Composed ID of the token.
     *  @param amount   Amount to be consumed.
     */
    function consume(bytes memory id, uint256 amount) public onlyAllowed(id) {
        balances[id] -= amount;
    }

    // =============================================== Getters ========================================================
    /** @dev Returns the balance of the item owned by a character.
     *  @param id   Composed ID of the token.
     */
    function balanceOf(bytes memory id) public view returns (uint256) {
        return balances[id];
    }
}
