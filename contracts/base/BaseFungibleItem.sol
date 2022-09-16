// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "../interfaces/ICivilizations.sol";
import "../interfaces/IBaseFungibleItem.sol";
import "../interfaces/IBaseERC20Wrapper.sol";
import "./BaseERC20Wrapper.sol";

/**
 * @dev `BaseFungibleItem` is a base contract to imitate the ERC20 functionality in the context of characters.
 */
contract BaseFungibleItem is Ownable, IBaseFungibleItem {
    // =============================================== Storage ========================================================

    /** @dev Name of the item. **/
    string public name;

    /** @dev Url pointing an image of the item. **/
    string public image;

    /** @dev Symbol of the item. **/
    string public symbol;

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Balances. **/
    mapping(bytes => uint256) balances;

    /** @dev ERC20 token for the fungible item. **/
    address public wrapper;

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
     * @param _image            Url of the token image.
     * @param _civilizations    The address of the `Civilizations` instance.
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

    /** @dev Mints an specific amount of items to a character balance.
     *  @param id       Composed ID of the token.
     *  @param amount   Amount to be minted.
     */
    function mintTo(bytes memory id, uint256 amount) public onlyOwner {
        _mint(id, amount);
    }

    /** @dev Mints an specific amount of items to a character balance.
     *  @param id       Composed ID of the token.
     *  @param amount   Amount to be consumed.
     */
    function consume(bytes memory id, uint256 amount) public onlyAllowed(id) {
        require(
            balances[id] >= amount,
            "BaseFungibleItem: not enough balance to consume"
        );
        balances[id] -= amount;
    }

    /** @dev Converts the internal fungible item to an ERC20 standard token.
     *  @param id       Composed ID of the token.
     *  @param amount   Amount to be wrapped.
     */
    function wrap(bytes memory id, uint256 amount) public onlyAllowed(id) {
        consume(id, amount);
        IBaseERC20Wrapper(wrapper).mint(
            ICivilizations(civilizations).ownerOf(id),
            amount
        );
    }

    /** @dev Converts the standard ERC20 token to an internable fungible item.
     *  @param id       Composed ID of the token to transfer the item.
     *  @param amount   Amount to be unwrapped.
     */
    function unwrap(bytes memory id, uint256 amount) public onlyAllowed(id) {
        ERC20Burnable(wrapper).burn(amount);
        _mint(id, amount);
    }

    // =============================================== Getters ========================================================
    /** @dev Returns the balance of the item owned by a character.
     *  @param id   Composed ID of the token.
     */
    function balanceOf(bytes memory id) public view returns (uint256) {
        return balances[id];
    }

    // =============================================== Internal ========================================================
    /** @dev Internal function to mint fungible tokens without owner check.
     *  @param id       Composed ID of the token.
     *  @param amount   Amount to be minted.
     */
    function _mint(bytes memory id, uint256 amount) internal {
        balances[id] += amount;
    }
}
