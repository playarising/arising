// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract RefresherToken is ERC20Burnable, Ownable {
    // Token used to mint the refresher token
    address usd;

    // Price to mint each refresher token
    uint256 price;

    constructor(address _usd, uint256 _price)
        ERC20("Arising: Refresher", "ARISING")
    {
        usd = _usd;
        price = _price;
    }

    /**
     * @dev Mint any amount of tokens by paying the usd token at the price rate.
     */
    function mint(uint256 amount) public {
        IERC20(usd).transferFrom(
            msg.sender,
            address(this),
            (amount * price) * (1 ether)
        );
        _mint(msg.sender, amount);
    }

    /**
     * @dev Overrides the ERC20 decimals with 0.
     */
    function decimals() public view virtual override returns (uint8) {
        return 0;
    }

    /**
     * @dev Withdraw the USD tokens to the owner.
     */
    function withdraw() public onlyOwner {
        uint256 balance = IERC20(usd).balanceOf(address(this));
        IERC20(usd).transfer(owner(), balance);
    }
}
