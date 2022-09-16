// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "../interfaces/IBaseERC20Wrapper.sol";

/**
 * @dev `BaseERC20Wrapper` is a a ERC20 token implementation to wrap around `BaseFungibleItem` instances.
 */
contract BaseERC20Wrapper is Ownable, ERC20Burnable, IBaseERC20Wrapper {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _name     The name of the token.
     * @param _symbol   The symbol of the token.
     */
    constructor(string memory _name, string memory _symbol)
        ERC20(_name, _symbol)
    {}

    /** @dev Mints an specific amount of tokens to an address.
     *  @param to       Address that receives the tokens.
     *  @param amount   Amount to be minted.
     */
    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }
}
