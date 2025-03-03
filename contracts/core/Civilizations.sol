// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import '@openzeppelin/contracts/utils/Address.sol';

import "../interfaces/IBaseERC721.sol";
import "../interfaces/ICivilizations.sol";

/**
 * @title Civilizations
 * @notice This contract stores all the [BaseERC721](/docs/base/BaseERC721.md) instances usable on the environmne. The contract
 * is in charge of token ownership verifications and generating/storing composable IDs for each character.
 *
 * @notice Implementation of the [ICivilizations](/docs/interfaces/ICivilizations.md) interface.
 */

contract Civilizations is
    ICivilizations,
    Initializable,
    PausableUpgradeable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    using Address for address;

    // =============================================== Storage ========================================================
    /** @notice Map to track the supported [BaseERC721](/docs/base/BaseERC721.md) instances. */
    mapping(uint256 => address) civilizations;

    /** @notice Map to track the IDs of the supported [BaseERC721](/docs/base/BaseERC721.md) instances. */
    mapping(address => uint256) civilizations_id;

    /** @notice Array to track the [BaseERC721](/docs/base/BaseERC721.md) IDs. */
    uint256[] private _civilizations;

    /** @notice Map to track the count of address mints. */
    mapping(address => uint256) private _minters;

    /** @notice Constant for address of the `ERC20` token used to purchase. */
    address public token;

    /** @notice Map to track the character upgrades. */
    mapping(bytes => mapping(uint256 => bool)) private character_upgrades;

    /** @notice Map to track the upgrades information. */
    mapping(uint256 => Upgrade) public upgrades;

    /** @notice Map to the price to mint characters. */
    uint256 public price;

    // =============================================== Modifiers ======================================================

    /**  @notice Checks if the `msg.sender` is a civilization contract to emit a Transfer event. */
    modifier onlyCivilization() {
        require(
            civilizations_id[msg.sender] != 0,
            "Civilizations: onlyCivilization() msg.sender is not a valid civilization."
        );
        _;
    }

    // =============================================== Events =========================================================

    /**
     * @notice Event emmited when the [mint](#mint) function is called.
     *
     * Requirements:
     * @param _owner    Owner of the minted token.
     * @param _id       Composed ID of the character.
     */
    event Summoned(address indexed _owner, bytes _id);

    /**
     * @notice Event emmited when the [buyUpgrade](#buyUpgrade) function is called.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     * @param _upgrade_id   ID of the upgrade purchased.
     */
    event UpgradePurchased(bytes _id, uint256 _upgrade_id);

    /**
     * @notice Event emmited when the [transfer](#transfer) function is called.
     *
     * Requirements:
     * @param _from     Address of the character transfering the character.
     * @param _to       New owner of the character.
     * @param _id       Composed ID of the character
     */
    event Transfer(address _from, address _to, bytes _id);

    // =============================================== Setters ========================================================

    /**
     * @notice Initialize.
     *
     * Requirements:
     * @param _token    Address of the token used to purchase upgrades.
     */
    function initialize(address _token) public initializer {
        __Ownable_init();
        __Pausable_init();
        __UUPSUpgradeable_init();
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
     * @notice Emits a transfer event to track the user of the character.
     *
     * Requirements:
     * @param _from         Address of the character transfering the character.
     * @param _to           New owner of the character.
     * @param _token_id     ID of the character in the context of a [BaseERC721](/docs/base/BaseERC721.md) instance.
     */
    function transfer(
        address _from,
        address _to,
        uint256 _token_id
    ) public onlyCivilization {
        uint256 _civilization_id = civilizations_id[msg.sender];
        emit Transfer(_from, _to, getTokenID(_civilization_id, _token_id));
    }

    /**
     * @notice Activates or deactivates an upgrade purchase.
     *
     * Requirements:
     * @param _upgrade_id   ID of the upgrade to change.
     * @param _available     Boolean to activate/deactivate.
     */
    function setInitializeUpgrade(
        uint256 _upgrade_id,
        bool _available
    ) public onlyOwner {
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
    function setUpgradePrice(
        uint256 _upgrade_id,
        uint256 _price
    ) public onlyOwner {
        require(
            _upgrade_id > 0 && _upgrade_id <= 3,
            "Civilizations: setUpgradePrice() invalid upgrade id."
        );
        upgrades[_upgrade_id].price = _price;
    }

    /**
     * @notice Sets the price to mint a character.
     *
     * Requirements:
     * @param _price     Amount of tokens to pay for the upgrade.
     */
    function setMintPrice(uint256 _price) public onlyOwner {
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
        uint256 _civilization_id = _civilizations.length + 1;
        civilizations[_civilization_id] = _civilization;
        civilizations_id[_civilization] = _civilization_id;
        _civilizations.push(_civilization_id);
    }

    /**
     * @notice Creates a new token of the valid civilizations list to the `msg.sender`.
     *
     * Requirements:
     * @param _civilization_id     ID of the civilization.
     */
    function mint(uint256 _civilization_id) public whenNotPaused {
        require(
            _civilization_id != 0 && _civilization_id <= _civilizations.length,
            "Civilizations: mint() invalid civilization id."
        );
        require(
            civilizations[_civilization_id] != address(0),
            "Civilizations: mint() invalid civilization address."
        );
        require(
            _canMint(msg.sender),
            "Civilizations: mint() address already minted."
        );
        IERC20(token).transferFrom(msg.sender, owner(), price);
        _addMint(msg.sender);
        emit Summoned(
            msg.sender,
            getTokenID(
                _civilization_id,
                IBaseERC721(civilizations[_civilization_id]).mint(msg.sender)
            )
        );
    }

    /**
     * @notice Purchase an upgrade and marks it as available for the provided composed ID.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _upgrade_id   ID of the upgrade to purchase.
     */
    function buyUpgrade(
        bytes memory _id,
        uint256 _upgrade_id
    ) public whenNotPaused {
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
        IERC20(token).transferFrom(msg.sender, owner(), _price);
        character_upgrades[_id][_upgrade_id] = true;
        emit UpgradePurchased(_id, _upgrade_id);
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to return the upgrades for a composed ID.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _upgrades    Array of booleans for each upgrade.
     */
    function getCharacterUpgrades(
        bytes memory _id
    ) public view returns (bool[3] memory _upgrades) {
        return (
            [
                character_upgrades[_id][1],
                character_upgrades[_id][2],
                character_upgrades[_id][3]
            ]
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
    function getUpgradeInformation(
        uint256 _upgrade_id
    ) public view returns (Upgrade memory _upgrade) {
        require(
            _upgrade_id > 0 && _upgrade_id <= 3,
            "Civilizations: getUpgradeInformation() invalid upgrade id."
        );
        return upgrades[_upgrade_id];
    }

    /**
     * @notice Returns the composed ID of a token from a valid civilization.
     *
     * Requirements:
     * @param _civilization_id  ID of the civilization.
     * @param _token_id         ID of the token to get the composed ID.
     *
     * @return _id              Composed ID of the character.
     */
    function getTokenID(
        uint256 _civilization_id,
        uint256 _token_id
    ) public view returns (bytes memory _id) {
        require(
            _civilization_id != 0 && _civilization_id <= _civilizations.length,
            "Civilizations: getTokenID() invalid civilization id."
        );
        require(
            IBaseERC721(civilizations[_civilization_id]).exists(_token_id),
            "Civilizations: getTokenID() token not minted."
        );
        return abi.encode(_civilization_id, _token_id);
    }

    /**
     * @notice External function to check if the `msg.sender` can access a token.
     *
     * Requirements:
     * @param _spender      Address to check ownership or allowance.
     * @param _id           Composed ID of the character.
     *
     * @return _allowed     Boolean to check if access is valid.
     */
    function isAllowed(
        address _spender,
        bytes memory _id
    ) public view returns (bool _allowed) {
        (uint256 _civilization_id, uint256 _token_id) = _decodeID(_id);
        require(
            _civilization_id != 0 && _civilization_id <= _civilizations.length,
            "Civilizations: isAllowed() invalid civilization id."
        );
        address _civilization = civilizations[_civilization_id];
        require(
            _civilization != address(0),
            "Civilizations: isAllowed() address of the civilization not found."
        );
        return
            IBaseERC721(_civilization).isApprovedOrOwner(_spender, _token_id);
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
            _civilization_id != 0 && _civilization_id <= _civilizations.length,
            "Civilizations: exists() invalid civilization id."
        );
        address _civilization = civilizations[_civilization_id];
        require(
            _civilization != address(0),
            "Civilizations: isAllowed() address of the civilization not found."
        );
        return IBaseERC721(_civilization).exists(_token_id);
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
            _civilization_id != 0 && _civilization_id <= _civilizations.length,
            "Civilizations: ownerOf() invalid civilization id."
        );
        address _civilization = civilizations[_civilization_id];
        require(
            _civilization != address(0),
            "Civilizations: isAllowed() address of the civilization not found."
        );
        return IERC721(_civilization).ownerOf(_token_id);
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
            !_minter.isContract(),
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
    function _decodeID(
        bytes memory _id
    ) internal pure returns (uint256 _civilization, uint256 _token_id) {
        return abi.decode(_id, (uint256, uint256));
    }

    /** @notice Internal function make sure upgrade proxy caller is the owner. */
    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyOwner {}
}
