// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

import "../interfaces/IBaseERC20Wrapper.sol";

/**
 * @title BaseERC20Wrapper
 * @notice This contract is a standard `ERC20` implementation with burnable and mintable
 * functions exposed to the contract owner. This contract is a wrapper for the [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instance to convert
 * an internal fungible token to the `ERC20` standard.
 *
 * @notice Implementation of the [IBaseERC20Wrapper](/docs/interfaces/IBaseERC20Wrapper.md) interface.
 */
contract BaseERC20Wrapper is IBaseERC20Wrapper, Ownable, ERC20Burnable {
    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _name     Name of the `ERC20` token.
     * @param _symbol   Symbol of the `ERC20` token.
     */
    constructor(
        string memory _name,
        string memory _symbol
    ) ERC20(_name, _symbol) {}

    /**
     * @notice Creates tokens to the address provided.
     *
     * Requirements:
     * @param _to        Address that receives the tokens.
     * @param _amount    Amount to be minted.
     */
    function mint(address _to, uint256 _amount) public onlyOwner {
        _mint(_to, _amount);
    }
}
