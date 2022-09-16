// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/IBaseGadgetToken.sol";

/**
 * @dev `BaseGadgetToken` is a contract serve as a self-served tokens to perform upgrades for `Stats`.
 */
contract BaseGadgetToken is ERC20Burnable, Ownable, IBaseGadgetToken, Pausable {
    // =============================================== Storage ========================================================
    /** @dev Address of the token used to charge the mint. **/
    address public token;

    /** @dev Amount of tokens required to mint one Refresh Token (in wei). **/
    uint256 public price;

    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param name   Name of the token.
     * @param symbol   Symbol of the token.
     * @param _token   Address of the token to charge.
     * @param _price   Amount of tokens to charge.
     */
    constructor(
        string memory name,
        string memory symbol,
        address _token,
        uint256 _price
    ) ERC20(name, symbol) {
        token = _token;
        price = _price;
    }

    /** @dev Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @dev Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
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
        require(
            IERC20(token).balanceOf(msg.sender) >= totalCost(amount),
            "BaseGadgetToken: not enough balance of payment tokens to mint tokens."
        );
        require(
            IERC20(token).allowance(msg.sender, address(this)) >=
                totalCost(amount),
            "BaseGadgetToken: not enough allowance to mint tokens."
        );
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

    /**
     * @dev Returns the total cost to mint a number of tokens.
     * @param amount    Amount of tokens to mint.
     */
    function totalCost(uint256 amount) public view returns (uint256) {
        return amount * price;
    }

    /** @dev Overrides the ERC20 `decimals` function with an URI specified over the constructor. */
    function decimals() public view virtual override returns (uint8) {
        return 0;
    }
}
