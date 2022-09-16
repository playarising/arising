// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";

import "../interfaces/ICivilizations.sol";
import "../interfaces/IBaseFungibleItem.sol";
import "../interfaces/IBaseERC20Wrapper.sol";

import "./BaseERC20Wrapper.sol";

/**
 * @title BaseERC721
 * @notice This contract an imitation of the ERC20 standard to work around the character context.
 * It tracks balances of characters tokens. This also includes functions to wrap and unwrap to a
 * {BaseERC20Wrapper} instance.
 *
 * @dev Implementation of the {IBaseFungibleItem} interface.
 */
contract BaseFungibleItem is IBaseFungibleItem, Ownable {
    // =============================================== Storage ========================================================

    /** @notice Constant for the name of the item. */
    string public name;

    /** @notice Constant the url pointing to the image of the item. */
    string public image;

    /** @notice Constant for the symbol of the item. */
    string public symbol;

    /** @notice Constant for the address of the {Civilizations} instance. */
    address public civilizations;

    /** @notice Map to track the balances of characters. */
    mapping(bytes => uint256) balances;

    /** @notice Constant for the address of the {BaseERC20Wrapper} instance. */
    address public wrapper;

    // =============================================== Modifiers ======================================================

    /**
     * @notice Checks against the {Civilizations} instance if the {msg.sender} is the owner or
     * has allowance to access a composed ID.
     *
     * Requirements:
     * @param _id    Composed ID of the token.
     */
    modifier onlyAllowed(bytes memory _id) {
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "BaseFungibleItem: require consumer to be owner or have allowance"
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _name             Name of the ERC20 token.
     * @param _symbol           Symbol of the ERC20 token.
     * @param _image            Url of the item image.
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(
        string memory _name,
        string memory _symbol,
        string memory _image,
        address _civilizations
    ) {
        name = _name;
        symbol = _symbol;
        image = _image;
        civilizations = _civilizations;
        wrapper = address(new BaseERC20Wrapper(_name, _symbol));
    }

    /**
     * @notice Creates tokens to the character composed ID provided.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   Amount of tokens to create.
     */
    function mintTo(bytes memory _id, uint256 _amount) public onlyOwner {
        _mint(_id, _amount);
    }

    /**
     * @notice Reduces tokens to the character composed ID provided.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   Amount of tokens to create.
     */
    function consume(bytes memory _id, uint256 _amount)
        public
        onlyAllowed(_id)
    {
        require(
            balances[_id] >= _amount,
            "BaseFungibleItem: not enough balance to consume"
        );
        balances[_id] -= _amount;
    }

    /**
     * @notice Converts the internal item to an ERC20 through the {BaseERC20Wrapper}.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   Amount of tokens to create.
     */
    function wrap(bytes memory _id, uint256 _amount) public onlyAllowed(_id) {
        consume(_id, _amount);
        IBaseERC20Wrapper(wrapper).mint(
            ICivilizations(civilizations).ownerOf(_id),
            _amount
        );
    }

    /**
     * @notice Converts the wrapped ERC20 token to an internal fungible item.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   Amount of tokens to create.
     */
    function unwrap(bytes memory _id, uint256 _amount) public onlyAllowed(_id) {
        ERC20Burnable(wrapper).burn(_amount);
        _mint(_id, _amount);
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to get the balance of the character composed ID provided.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _balance     Amount of tokens of the character from the composed ID.
     */
    function balanceOf(bytes memory _id)
        public
        view
        returns (uint256 _balance)
    {
        return balances[_id];
    }

    // =============================================== Internal ========================================================

    /**
     * @notice Internal function to create tokens to the character composed ID provided without
     * without owner check.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   Amount of tokens to create.
     */
    function _mint(bytes memory _id, uint256 _amount) internal {
        balances[_id] += _amount;
    }
}
