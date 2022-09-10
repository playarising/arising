// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/IBaseERC721.sol";

/**
 * @dev `BaseFungibleItem` is a base contract to imitate the ERC20 functionality in the context of characters.
 */
contract BaseFungibleItem is Ownable {
    // =============================================== Storage ========================================================

    /** @dev Name of the item. **/
    string name;

    /** @dev Symbol of the item. **/
    string symbol;

    /** @dev Balances. **/
    mapping(string => uint256) balances;

    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _name      The name of fungible token.
     * @param _symbol    The symbol of the fungible token.
     */
    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;
    }

    /** @dev Mints an specific amount of items to a character balance.
     *  @param id       Composed ID of the token.
     *  @param amount   Amount to be minted.
     */
    function mintTo(string memory id, uint256 amount) public onlyOwner {
        balances[id] += amount;
    }

    /** @dev Mints an specific amount of items to a character balance.
     *  @param id       Composed ID of the token.
     *  @param amount   Amount to be consumed.
     */
    function consume(string memory id, uint256 amount) public {
        // TODO check ownership
        balances[id] -= amount;
    }

    // =============================================== Getters ========================================================
    /** @dev Returns the balance of the item owned by a character.
     *  @param id   Composed ID of the token.
     */
    function balanceOf(string memory id) public view returns (uint256) {
        return balances[id];
    }
}
