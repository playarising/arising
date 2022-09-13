// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "../interfaces/IBaseERC721.sol";
import "../interfaces/ICivilizations.sol";

/**
 * @dev `Civilizations` is the contract that stores all the usable civilizations.
 */
contract Civilizations is Ownable, ICivilizations {
    using Address for address;

    // =============================================== Storage ========================================================
    /** @dev Map to store the civilizations implemented. **/
    mapping(address => uint256) _civilizations;

    /** @dev Array to store the civilizations implemented. **/
    address[] civilizations;

    /** @dev Map to track the amount of tokens minted by address. **/
    mapping(address => uint256) private _minters;

    /** @dev Boolean to check if the implementation is usable. **/
    bool public initialized;

    /** @dev Address of the token used to charge the mint. **/
    address public token;

    /** @dev Map to track the character upgrades. **/
    UpgradedCharacters private _upgrades;

    /** @dev Map to track upgrades information. **/
    mapping(uint256 => Upgrade) public upgrades;

    // ============================================== Modifiers =======================================================

    /**
     * @dev Checks if `initialized` is enabled.
     */
    modifier onlyInitialized() {
        require(initialized, "Civilizations: contract is not initialized");
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _token   Address of the token to charge.
     */
    constructor(address _token) {
        token = _token;
        initialized = false;
        upgrades[1] = Upgrade(0, false);
        upgrades[2] = Upgrade(0, false);
        upgrades[3] = Upgrade(0, false);
    }

    /** @dev Enables the `Civilizations` implementation. */
    function setInitialized() public onlyOwner {
        initialized = true;
    }

    /** @dev Adds a civilization to the contract.
     *  @param upgrade      The ID of the upgrade.
     *  @param available    The availability of the upgrade.
     */
    function setInitializeUpgrade(uint256 upgrade, bool available)
        public
        onlyOwner
    {
        require(
            upgrade > 0 && upgrade <= 3,
            "Civilizations: upgrade id doesn't exist."
        );
        require(
            upgrades[upgrade].price != 0,
            "Civilizations: to initialize an upgrade set the price first."
        );
        upgrades[upgrade].available = available;
    }

    /** @dev Adds a civilization to the contract.
     *  @param upgrade  The ID of the upgrade.
     *  @param price    The amount of tokens to charge for the upgrade.
     */
    function setUpgradePrice(uint256 upgrade, uint256 price) public onlyOwner {
        require(
            upgrade > 0 && upgrade <= 3,
            "Civilizations: upgrade id doesn't exist."
        );
        upgrades[upgrade].price = price;
    }

    /** @dev Sets the `token` address.
     *  @param _token   address of the token to use for charge.
     */
    function setToken(address _token) public onlyOwner {
        token = _token;
    }

    /** @dev Adds a civilization to the contract.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function addCivilization(address _instance) public onlyOwner {
        require(
            _instance != address(0),
            "Civilizations: instance address is null."
        );
        require(
            msg.sender == Ownable(_instance).owner(),
            "Civilizations: msg.sender must be the owner of the instance added."
        );
        uint256 newId = civilizations.length + 1;
        _civilizations[_instance] = newId;
        civilizations.push(_instance);
    }

    /** @dev Mints a token.
     *  @param _instance  Address of the `BaseERC721` instance to mint.
     */
    function mint(address _instance) public onlyInitialized {
        require(
            _instance != address(0),
            "Civilizations: instance address is null."
        );
        require(
            _civilizations[_instance] != 0,
            "Civilizations: instance is not an Arising civilization."
        );
        require(
            _canMint(msg.sender),
            "Civilizations: address used already minted 5 characters."
        );
        IBaseERC721(_instance).mint(msg.sender);
        _addMinted(msg.sender);
    }

    /**
     * @dev Marks a character upgrade as purchased.
     * @param id         Composed ID of the token.
     * @param upgrade    Upgrade id.
     */
    function buyUpgrade(bytes memory id, uint256 upgrade) public {
        require(
            upgrade > 0 && upgrade <= 3,
            "Civilizations: upgrade id doesn't exist."
        );
        require(
            isAllowed(msg.sender, id),
            "Civilizations: can't purchase for a non owned token."
        );
        require(
            upgrades[upgrade].available,
            "Civilizations: can't purchase an upgrade not initialized."
        );
        uint256 price = upgrades[upgrade].price;
        require(
            IERC20(token).balanceOf(msg.sender) >= price,
            "Civilizations: not enough balance of payment tokens to mint tokens."
        );
        require(
            IERC20(token).allowance(msg.sender, address(this)) >=
                upgrades[upgrade].price,
            "Civilizations: not enough allowance to mint tokens."
        );
        IERC20(token).transferFrom(msg.sender, address(this), price);
        if (upgrade == 1) {
            _upgrades.upgrade_1[id] = true;
        }
        if (upgrade == 2) {
            _upgrades.upgrade_2[id] = true;
        }
        if (upgrade == 3) {
            _upgrades.upgrade_3[id] = true;
        }
    }

    /**
     * @dev Transfers the total amount of `token` stored in the contract to `owner`.
     */
    function withdraw() public onlyOwner {
        uint256 balance = IERC20(token).balanceOf(address(this));
        IERC20(token).transfer(owner(), balance);
    }

    // =============================================== Getters ========================================================

    /** @dev Returns the internal ID for a civilization.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function getID(address _instance) public view returns (uint256) {
        require(
            _instance != address(0),
            "Civilizations: instance address is null."
        );
        require(
            _civilizations[_instance] != 0,
            "Civilizations: instance is not an Arising civilization."
        );
        return _civilizations[_instance];
    }

    /** @dev Returns the upgrades for a composed ID.
     *  @param id Composed token id.
     */
    function getTokenUpgrades(bytes memory id)
        public
        view
        returns (TokenUpgrades memory)
    {
        return
            TokenUpgrades(
                _upgrades.upgrade_1[id],
                _upgrades.upgrade_2[id],
                _upgrades.upgrade_3[id]
            );
    }

    /** @dev Returns the upgrades for a composed ID.
     *  @param upgrade ID of the upgrade.
     */
    function getUpgradeInformation(uint256 upgrade)
        public
        view
        returns (Upgrade memory)
    {
        require(
            upgrade > 0 && upgrade <= 3,
            "Civilizations: upgrade id doesn't exist."
        );
        return upgrades[upgrade];
    }

    /** @dev Returns the list of civilizations.
     */
    function getCivilizations() public view returns (address[] memory) {
        return civilizations;
    }

    /** @dev Returns a composed ID from a collection.
     *  @param _instance  Address of the `BaseERC721` instance.
     *  @param _id        The ID of the token inside the collection.
     */
    function getTokenID(address _instance, uint256 _id)
        public
        view
        returns (bytes memory)
    {
        require(
            _instance != address(0),
            "Civilizations: instance address is null."
        );
        uint256 civilizationID = _civilizations[_instance];
        require(
            civilizationID != 0,
            "Civilizations: instance is not an Arising civilization."
        );
        require(
            IBaseERC721(_instance).exists(_id),
            "Civilizations: token id is not minted."
        );
        return abi.encode(civilizationID, _id);
    }

    /** @dev Function to check if an address has allowance to a composed ID.
     *  @param spender   Address to check ownership or allowance.
     *  @param _id       Composed token id.
     */
    function isAllowed(address spender, bytes memory _id)
        public
        view
        returns (bool)
    {
        (uint256 civilizationID, uint256 tokenID) = _decomposeTokenID(_id);
        require(
            civilizationID <= civilizations.length,
            "Civilizations: id of the civilization is not valid."
        );
        address instance = civilizations[civilizationID - 1];
        return IBaseERC721(instance).isApprovedOrOwner(spender, tokenID);
    }

    /** @dev Function to check if a composed ID is already minted.
     *  @param _id Composed token id.
     */
    function exists(bytes memory _id) public view returns (bool) {
        (uint256 civilizationID, uint256 tokenID) = _decomposeTokenID(_id);
        require(
            civilizationID <= civilizations.length,
            "Civilizations: id of the civilization is not valid."
        );
        address instance = civilizations[civilizationID - 1];
        return IBaseERC721(instance).exists(tokenID);
    }

    // =============================================== Internal ========================================================

    /**
     * @dev Adds a mint to the mint counter for the address.
     * @param _minter The minter address.
     */
    function _addMinted(address _minter) internal {
        _minters[_minter] += 1;
    }

    /**
     * @dev Checks if an address is able to mint more tokens.
     * @param _minter The minter address.
     */
    function _canMint(address _minter) internal view returns (bool) {
        require(
            !Address.isContract(_minter),
            "Civilizations: cannot mint from a contract"
        );
        return _minters[_minter] < 5;
    }

    /**
     * @dev Decompose a byte encoded token ID.
     * @param id The composed token id.
     */
    function _decomposeTokenID(bytes memory id)
        internal
        pure
        returns (uint256, uint256)
    {
        return abi.decode(id, (uint256, uint256));
    }
}
