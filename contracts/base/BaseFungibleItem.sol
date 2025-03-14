// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

import "../interfaces/ICivilizations.sol";
import "../interfaces/IBaseFungibleItem.sol";
import "../interfaces/IBaseERC20Wrapper.sol";

import "./BaseERC20Wrapper.sol";

/**
 * @title BaseERC721
 * @notice This contract an imitation of the `ERC20` standard to work around the character context.
 * It tracks balances of characters tokens. This also includes functions to wrap and unwrap to a
 * [BaseERC20Wrapper](/docs/base/BaseERC20Wrapper.md) instance.
 *
 * @notice Implementation of the [IBaseFungibleItem](/docs/interfaces/IBaseFungibleItem.md) interface.
 */
contract BaseFungibleItem is
    IBaseFungibleItem,
    Initializable,
    OwnableUpgradeable
{
    // =============================================== Storage ========================================================

    /** @notice Constant for the name of the item. */
    string public name;

    /** @notice Constant for the symbol of the item. */
    string public symbol;

    /** @notice Constant for the address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;

    /** @notice Map to track the balances of characters. */
    mapping(bytes => uint256) balances;

    /** @notice Constant for the address of the [BaseERC20Wrapper](/docs/base/BaseERC20Wrapper.md) instance. */
    address public wrapper;

    /** @notice Constant to enable/disable the token wrap. */
    bool public enable_wrap;

    /** @notice Map to store the list of authorized addresses to mint items. */
    mapping(address => bool) authorized;

    // =============================================== Modifiers ======================================================

    /**
     * @notice Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
     * has allowance to access a composed ID.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    modifier onlyAllowed(bytes memory _id) {
        require(
            ICivilizations(civilizations).exists(_id),
            "BaseFungibleItem: onlyAllowed() token not minted."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "BaseFungibleItem: onlyAllowed() msg.sender is not allowed to access this token."
        );
        _;
    }

    /** @notice Checks if the wrap functionality is enabled. */
    modifier onlyEnabled() {
        require(
            enable_wrap,
            "BaseFungibleItem: onlyEnabled() wrap is not enabled."
        );
        _;
    }

    /** @notice Checks if the `msg.sender` is authorized to mint items. */
    modifier onlyAuthorized() {
        require(
            authorized[msg.sender],
            "BaseFungibleItem: onlyAuthorized() msg.sender not authorized."
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @notice Initialize.
     *
     * Requirements:
     * @param _name             Name of the `ERC20` token.
     * @param _symbol           Symbol of the `ERC20` token.
     * @param _civilizations    Address of the [Civilizations](/docs/core/Civilizations.md) instance.
     */
    function initialize(
        string memory _name,
        string memory _symbol,
        address _civilizations
    ) public initializer {
        __Ownable_init(_msgSender());
        name = _name;
        symbol = _symbol;
        civilizations = _civilizations;
        wrapper = address(new BaseERC20Wrapper(_name, _symbol));
        enable_wrap = false;
        authorized[_msgSender()] = true;
    }

    /**
     * @notice Assigns a new address as an authority to mint items.
     *
     * Requirements:
     * @param _authority    Address to give authority.
     */
    function addAuthority(address _authority) public onlyOwner {
        authorized[_authority] = true;
    }

    /**
     * @notice Removes an authority to mint items.
     *
     * Requirements:
     * @param _authority    Address to give authority.
     */
    function removeAuthority(address _authority) public onlyOwner {
        require(
            authorized[_authority],
            "BaseFungibleItem: removeAuthority() address is not authorized."
        );
        authorized[_authority] = false;
    }

    /**
     * @notice Enables or disables the wrap function for the token.
     *
     * Requirements:
     * @param _enabled    Enable or diable wrap function.
     */
    function setWrapFunction(bool _enabled) public onlyOwner {
        enable_wrap = _enabled;
    }

    /**
     * @notice Creates tokens to the character composed ID provided.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   Amount of tokens to create.
     */
    function mintTo(bytes memory _id, uint256 _amount) public onlyAuthorized {
        _mint(_id, _amount);
    }

    /**
     * @notice Reduces tokens to the character composed ID provided.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   Amount of tokens to create.
     */
    function consume(
        bytes memory _id,
        uint256 _amount
    ) public onlyAllowed(_id) {
        require(
            balances[_id] >= _amount,
            "BaseFungibleItem: consume() not enough balance."
        );
        balances[_id] -= _amount;
    }

    /**
     * @notice Converts the internal item to an `ERC20` through the [BaseERC20Wrapper](/docs/base/BaseERC20Wrapper.md).
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   Amount of tokens to create.
     */
    function wrap(
        bytes memory _id,
        uint256 _amount
    ) public onlyAllowed(_id) onlyEnabled {
        consume(_id, _amount);
        IBaseERC20Wrapper(wrapper).mint(
            ICivilizations(civilizations).ownerOf(_id),
            _amount * 1 ether
        );
    }

    /**
     * @notice Converts the wrapped `ERC20` token to an internal fungible item.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   Amount of tokens to create.
     */
    function unwrap(
        bytes memory _id,
        uint256 _amount
    ) public onlyAllowed(_id) onlyEnabled {
        ERC20Burnable(wrapper).burnFrom(msg.sender, _amount * 1 ether);
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
    function balanceOf(
        bytes memory _id
    ) public view returns (uint256 _balance) {
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
