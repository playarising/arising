// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";

import "../interfaces/IBaseERC721.sol";
import "../interfaces/ICivilizations.sol";

/**
 * @title Civilizations
 * @notice This contract stores all the [BaseERC721](/docs/base/BaseERC721.md) instances usable on the environmne. The contract
 * is in charge of token ownership verifications and generating/storing composable IDs for each character.
 *
 * @notice Implementation of the [ICivilizations](/docs/interfaces/ICivilizations.md) interface.
 */

contract Civilizations is ICivilizations, Ownable, Pausable {
    using Address for address;

    // =============================================== Storage ========================================================
    /** @notice Map to track the supported [BaseERC721](/docs/base/BaseERC721.md) instances. */
    mapping(address => uint256) civilizations;

    /** @notice Array to track the [BaseERC721](/docs/base/BaseERC721.md) IDs. */
    address[] _civilizations;

    /** @notice Map to track the count of address mints. */
    mapping(address => uint256) private _minters;

    /** @notice Constant for address of the `ERC20` token used to purchase. */
    address public token;

    /** @notice Map to track the character upgrades. */
    UpgradedCharacters private character_upgrades;

    /** @notice Map to track the upgrades information. */
    mapping(uint256 => Upgrade) public upgrades;

    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _token    Address of the token used to purchase upgrades.
     */
    constructor(address _token) {
        token = _token;
        upgrades[1] = Upgrade(0, false);
        upgrades[2] = Upgrade(0, false);
        upgrades[3] = Upgrade(0, false);
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
     * @notice Activates or deactivates an upgrade purchase.
     *
     * Requirements:
     * @param _upgrade_id   ID of the upgrade to change.
     * @param _available     Boolean to activate/deactivate.
     */
    function setInitializeUpgrade(uint256 _upgrade_id, bool _available)
        public
        onlyOwner
    {
        require(
            _upgrade_id > 0 && _upgrade_id <= 3,
            "Civilizations: setInitializeUpgrade() invalid upgrade id."
        );
        require(
            upgrades[_upgrade_id].price != 0,
            "Civilizations: setInitializeUpgrade() no price set."
        );
        upgrades[_upgrade_id].available = _available;
    }

    /**
     * @notice Sets the price to purchase an upgrade.
     *
     * Requirements:
     * @param _upgrade_id   ID of the upgrade to change.
     * @param _price     Amount of tokens to pay for the upgrade.
     */
    function setUpgradePrice(uint256 _upgrade_id, uint256 _price)
        public
        onlyOwner
    {
        require(
            _upgrade_id > 0 && _upgrade_id <= 3,
            "Civilizations: setUpgradePrice() invalid upgrade id."
        );
        upgrades[_upgrade_id].price = _price;
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
     * @notice Adds a new [BaseERC721](/docs/base/BaseERC721.md) instance to the valid civilizations.
     *
     * Requirements:
     * @param _civilization     Address of the [BaseERC721](/docs/base/BaseERC721.md) instance to add.
     */
    function addCivilization(address _civilization) public onlyOwner {
        require(
            _civilization != address(0),
            "Civilizations: addCivilization() civilization address is empty."
        );
        require(
            msg.sender == Ownable(_civilization).owner(),
            "Civilizations: addCivilization() missing civilization ownership."
        );
        uint256 _civilization_id = _civilizations.length + 1;
        civilizations[_civilization] = _civilization_id;
        _civilizations.push(_civilization);
    }

    /**
     * @notice Creates a new token of the valid civilizations list to the `msg.sender`.
     *
     * Requirements:
     * @param _civilization     Address of the [BaseERC721](/docs/base/BaseERC721.md) instance to add.
     */
    function mint(address _civilization) public whenNotPaused {
        require(
            _civilization != address(0),
            "Civilizations: mint() civilization address is empty."
        );
        require(
            civilizations[_civilization] != 0,
            "Civilizations: mint() invalid civilization address."
        );
        require(
            _canMint(msg.sender),
            "Civilizations: mint() address already minted."
        );
        IBaseERC721(_civilization).mint(msg.sender);
        _addMint(msg.sender);
    }

    /**
     * @notice Purchase an upgrade and marks it as available for the provided composed ID.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _upgrade_id   ID of the upgrade to purchase.
     */
    function buyUpgrade(bytes memory _id, uint256 _upgrade_id)
        public
        whenNotPaused
    {
        require(
            _upgrade_id > 0 && _upgrade_id <= 3,
            "Civilizations: buyUpgrade() invalid upgrade id."
        );
        require(
            isAllowed(msg.sender, _id),
            "Civilizations: buyUpgrade() msg.sender is not allowed to access this token."
        );
        require(
            upgrades[_upgrade_id].available,
            "Civilizations: buyUpgrade() upgrade is not initialized."
        );
        uint256 _price = upgrades[_upgrade_id].price;
        require(
            IERC20(token).balanceOf(msg.sender) >= _price,
            "Civilizations: buyUpgrade() not enough balance to mint tokens."
        );
        require(
            IERC20(token).allowance(msg.sender, address(this)) >=
                upgrades[_upgrade_id].price,
            "Civilizations: buyUpgrade() not enough allowance to mint tokens."
        );
        IERC20(token).transferFrom(msg.sender, address(this), _price);
        if (_upgrade_id == 1) {
            character_upgrades.upgrade_1[_id] = true;
        }
        if (_upgrade_id == 2) {
            character_upgrades.upgrade_2[_id] = true;
        }
        if (_upgrade_id == 3) {
            character_upgrades.upgrade_3[_id] = true;
        }
    }

    /** @notice Transfers the total amount of tokens stored in the contract to the owner .*/
    function withdraw() public onlyOwner {
        uint256 balance = IERC20(token).balanceOf(address(this));
        IERC20(token).transfer(owner(), balance);
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to return the internal ID of a valid civilization.
     *
     * Requirements:
     * @param _civilization     Address of the [BaseERC721](/docs/base/BaseERC721.md) instance to add.
     * 
     * @return _civilization_id Internal ID of the civilization.
]     */
    function getID(address _civilization)
        public
        view
        returns (uint256 _civilization_id)
    {
        require(
            _civilization != address(0),
            "Civilizations: getID() instance address is empty."
        );
        require(
            civilizations[_civilization] != 0,
            "Civilizations: getID() invalid instance address."
        );
        return civilizations[_civilization];
    }

    /**
     * @notice External function to return the upgrades for a composed ID.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _upgrades    Struct of the character upgrades.
     */
    function getTokenUpgrades(bytes memory _id)
        public
        view
        returns (CharacterUpgrades memory _upgrades)
    {
        return
            CharacterUpgrades(
                character_upgrades.upgrade_1[_id],
                character_upgrades.upgrade_2[_id],
                character_upgrades.upgrade_3[_id]
            );
    }

    /**
     * @notice External function to return global status of an upgrade.
     *
     * Requirements:
     * @param _upgrade_id   ID of the upgrade.
     *
     * @return _upgrade     Upgrade information.
     */
    function getUpgradeInformation(uint256 _upgrade_id)
        public
        view
        returns (Upgrade memory _upgrade)
    {
        require(
            _upgrade_id > 0 && _upgrade_id <= 3,
            "Civilizations: getUpgradeInformation() invalid upgrade id."
        );
        return upgrades[_upgrade_id];
    }

    /**
     * @notice Returns the list of valid civilizations instances.
     *
     * @return _valid_civilizations     Array of valid civilizations intances.
     */
    function getCivilizations()
        public
        view
        returns (address[] memory _valid_civilizations)
    {
        return _civilizations;
    }

    /**
     * @notice Returns the composed ID of a token from a valid civilization.
     *
     * Requirements:
     * @param _civilization     Address of the [BaseERC721](/docs/base/BaseERC721.md) instance to add.
     * @param _token_id         ID of the token to get the composed ID.
     *
     * @return _id              Composed ID of the token.
     */
    function getTokenID(address _civilization, uint256 _token_id)
        public
        view
        returns (bytes memory _id)
    {
        require(
            _civilization != address(0),
            "Civilizations: getTokenID() civilization address is empty."
        );
        uint256 _civilization_id = civilizations[_civilization];
        require(
            _civilization_id != 0,
            "Civilizations: getTokenID() invalid civilization address."
        );
        require(
            IBaseERC721(_civilization).exists(_token_id),
            "Civilizations: getTokenID() token not minted."
        );
        return abi.encode(_civilization_id, _id);
    }

    /**
     * @notice External function to check if the `msg.sender` can access a token.
     *
     * Requirements:
     *  @param _spender     Address to check ownership or allowance.
     * @param _id           Composed ID of the character.
     *
     * @return _allowed     Boolean to check if access is valid.
     */
    function isAllowed(address _spender, bytes memory _id)
        public
        view
        returns (bool _allowed)
    {
        (uint256 _civilization_id, uint256 _token_id) = _decodeID(_id);
        require(
            _civilization_id <= _civilizations.length,
            "Civilizations: isAllowed() id of the civilization is not valid."
        );
        address instance = _civilizations[_civilization_id - 1];
        return IBaseERC721(instance).isApprovedOrOwner(_spender, _token_id);
    }

    /**
     * @notice External function to check a composed ID is already minted.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _exist       Boolean to check if the token is minted.
     */
    function exists(bytes memory _id) public view returns (bool _exist) {
        (uint256 _civilization_id, uint256 _token_id) = _decodeID(_id);
        require(
            _civilization_id <= _civilizations.length,
            "Civilizations: exists() id of the civilization is not valid."
        );
        address instance = _civilizations[_civilization_id - 1];
        return IBaseERC721(instance).exists(_token_id);
    }

    /**
     * @notice External function to return the owner a composed ID.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _owner       Address of the owner of the token.
     */
    function ownerOf(bytes memory _id) public view returns (address _owner) {
        (uint256 _civilization_id, uint256 _token_id) = _decodeID(_id);
        require(
            _civilization_id <= _civilizations.length,
            "Civilizations: ownerOf() id of the civilization is not valid."
        );
        address instance = _civilizations[_civilization_id - 1];
        return IERC721(instance).ownerOf(_token_id);
    }

    // =============================================== Internal ========================================================

    /**
     * @notice Internal function to add a mint count for the `msg.sender`.
     *
     * Requirements:
     * @param _minter       Address of the minter.
     */
    function _addMint(address _minter) internal {
        _minters[_minter] += 1;
    }

    /**
     * @notice Internal function check if the `msg.sender` can mint a token.
     *
     * Requirements:
     * @param _minter       Address of the minter.
     */
    function _canMint(address _minter) internal view returns (bool) {
        require(
            !Address.isContract(_minter),
            "Civilizations: _canMint() contract cannot mint."
        );
        return _minters[_minter] < 5;
    }

    /**
     * @notice Internal function to decode a composed ID to a civilization instance and token ID.
     *
     * Requirements:
     * @param _id           Composed ID.
     *
     * @return _civilization    The internal ID of the civilization.
     * @return _token_id        The token id of the composed ID.
     */
    function _decodeID(bytes memory _id)
        internal
        pure
        returns (uint256 _civilization, uint256 _token_id)
    {
        return abi.decode(_id, (uint256, uint256));
    }
}
