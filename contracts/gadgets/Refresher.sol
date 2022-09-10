// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/IRefresher.sol";

/**
 * @dev `Refresher` is a contract serve as a self-served gadget to perform refreshes for `Stats` without cooldown.
 */
contract Refresher is ERC20Burnable, Ownable, IRefresher {
    // =============================================== Storage ========================================================
    /** @dev Address of the token used to charge the mint. **/
    address public token;

    /** @dev Amount of tokens required to mint one Refresh Token (in wei). **/
    uint256 public price;

    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _token   Address of the token to charge.
     * @param _price   Amount of tokens to charge.
     */
    constructor(address _token, uint256 _price)
        ERC20("Arising: Refresh Token", "ARISING")
    {
        token = _token;
        price = _price;
    }

    /** @dev Sets the minting price.
     *  @param _price   Amount of tokens to charge (in wei).
     */
    function setPrice(uint256 _price) public onlyOwner {
        price = _price;
    }

    /** @dev Sets the `token` address.
     *  @param _token   address of the token to use for charge.
     */
    function setToken(address _token) public onlyOwner {
        token = _token;
    }

    /**
     * @dev Mints any amount of tokens to the `msg.sender` by paying the price in `token`.
     * @param amount    Amount of tokens to mint.
     */
    function mint(uint256 amount) public {
        IERC20(token).transferFrom(msg.sender, address(this), (amount * price));
        _mint(msg.sender, amount);
    }

    /**
     * @dev Mints any amount of tokens for free.
     * @param receiver      Address that will receive the tokens.
     * @param amount        Amount of tokens to mint.
     */
    function mintFree(address receiver, uint256 amount) public onlyOwner {
        _mint(receiver, amount);
    }

    /**
     * @dev Transfers the total amount of `token` stored in the contract to `owner`.
     */
    function withdraw() public onlyOwner {
        uint256 balance = IERC20(token).balanceOf(address(this));
        IERC20(token).transfer(owner(), balance);
    }

    // =============================================== Getters ========================================================

    /** @dev Overrides the ERC20 `decimals` function with an URI specified over the constructor. */
    function decimals() public view virtual override returns (uint8) {
        return 0;
    }
}
