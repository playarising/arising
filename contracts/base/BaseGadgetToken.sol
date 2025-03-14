// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import "@openzeppelin/contracts-upgradeable/token/ERC20/extensions/ERC20BurnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";

import "../interfaces/IBaseGadgetToken.sol";

/**
 * @title BaseGadgetToken
 * @notice This contract implements an `ERC20BurnableUpgradeable` token to serve as utility tokens that
 * can be purchased by themselves.
 *
 * @notice Implementation of the [IBaseGadgetToken](/docs/interfaces/IBaseGadgetToken.md) interface.
 */
contract BaseGadgetToken is
    IBaseGadgetToken,
    OwnableUpgradeable,
    ERC20BurnableUpgradeable,
    PausableUpgradeable
{
    // =============================================== Storage ========================================================
    /** @notice Constant for address of the `ERC20` token used to purchase. */
    address public token;

    /** @notice Constant for the price of each token (in wei). */
    uint256 public price;

    // =============================================== Setters ========================================================

    /**
     * @notice Initialize.
     *
     * Requirements:
     * @param _name     Name of the `ERC20` token.
     * @param _symbol   Symbol of the `ERC20` token.
     * @param _token    Address of the token used to purchase.
     * @param _price    Price for each token.
     */
    function initialize(
        string memory _name,
        string memory _symbol,
        address _token,
        uint256 _price
    ) public initializer {
        __ERC20_init(_name, _symbol);
        __ERC20Burnable_init();
        __Ownable_init(_msgSender());
        __Pausable_init();
        token = _token;
        price = _price;
    }

    /** @notice Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @notice Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
    }

    /**
     * @notice Changes the base price for each token.
     *
     * Requirements:
     * @param _price    Amount of tokens to charge for each token purchase (in wei).
     */
    function setPrice(uint256 _price) public onlyOwner {
        price = _price;
    }

    /**
     * @notice Changes the token address to charge.
     *
     * Requirements:
     * @param _token    Address of the new token to charge.
     */
    function setToken(address _token) public onlyOwner {
        token = _token;
    }

    /**
     * @notice Creates tokens to the `msg.sender` by charging the total amount of tokens.
     *
     * Requirements:
     * @param _amount   Amount of tokens to purchase.
     */
    function mint(uint256 _amount) public whenNotPaused {
        require(
            IERC20(token).balanceOf(msg.sender) >= getTotalCost(_amount),
            "BaseGadgetToken: mint() not enough balance to mint tokens."
        );
        require(
            IERC20(token).allowance(msg.sender, address(this)) >=
                getTotalCost(_amount),
            "BaseGadgetToken: mint() not enough allowance to mint tokens."
        );
        IERC20(token).transferFrom(msg.sender, owner(), (_amount * price));
        _mint(msg.sender, _amount);
    }

    /**
     * @notice Creates tokens to the address provided.
     *
     * Requirements:
     * @param _receiver     Address that receives the tokens.
     * @param _amount       Amount of tokens to create.
     */
    function mintFree(
        address _receiver,
        uint256 _amount
    ) public whenNotPaused onlyOwner {
        _mint(_receiver, _amount);
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to get the total amount of tokens required to purchase an amount of tokens.
     *
     * Requirements:
     * @param _amount   Amount of tokens to purchase.
     *
     * @return _cost    Total cost to purchase tokens.
     */
    function getTotalCost(uint256 _amount) public view returns (uint256 _cost) {
        return _amount * price;
    }

    /**
     * @notice Overrides the `ERC20.decimals` function to return 0 decimals.
     *
     * @return _decimals    Amount of decimals of the token.
     */
    function decimals() public view virtual override returns (uint8 _decimals) {
        return 0;
    }
}
