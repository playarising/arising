// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "../interfaces/IBaseERC721.sol";

/**
 * @dev `Civilizations` is the contract that stores all the usable civilizations.
 */
contract Civilizations is Ownable {
    using Address for address;

    // =============================================== Storage ========================================================
    /** @dev Map to store the civilizations implemented. **/
    mapping(address => uint256) _civilizations;

    /** @dev Array to store the civilizations implemented. **/
    address[] civilizations;

    /** @dev Map to track the amount of tokens minted by address. **/
    mapping(address => uint256) private _minters;

    /** @dev The receiver address. **/
    address payable payments_receiver;

    /** @dev Price of each mint in MATIC in wei. **/
    uint256 public price;

    /** @dev Boolean to check if the implementation is usable. **/
    bool public initialized;

    // ============================================== Modifiers =======================================================

    /**
     * @dev Checks if `initialized` is enabled.
     */
    modifier onlyInitialized() {
        require(initialized, "BaseERC721: contract is not initialized");
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _price                The price for each token minted.
     * @param _payments_receiver    The address that will receive the payments.
     */
    constructor(uint256 _price, address payable _payments_receiver) {
        price = _price;
        payments_receiver = _payments_receiver;
        initialized = false;
    }

    /** @dev Enables the `Civilizations` implementation. */
    function setInitialized() public onlyOwner {
        require(price != 0, "Civilizations: can't initialize when price is 0");
        initialized = true;
    }

    /** @dev Sets the minting price.
     *  @param _price   Price in MATIC in wei.
     */
    function setPrice(uint256 _price) public onlyOwner {
        price = _price;
    }

    /** @dev Sets payment receive address.
     *  @param _payments_receiver   Address to receive the payments.
     */
    function setPaymentsReceiver(address payable _payments_receiver)
        public
        onlyOwner
    {
        payments_receiver = _payments_receiver;
    }

    /** @dev Adds a civilization to the contract.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function addCivilization(address _instance) public onlyOwner {
        require(
            _instance != address(0),
            "Civilizations: instance address is null"
        );
        uint256 newId = civilizations.length + 1;
        _civilizations[_instance] = newId;
        civilizations.push(_instance);
    }

    /** @dev Mints a token from an .
     *  @param _instance  Address of the `BaseERC721` instance to mint.
     */
    function mint(address _instance) public payable {
        require(
            _instance != address(0),
            "Civilizations: instance address is null"
        );
        require(
            _civilizations[_instance] != 0,
            "Civilizations: instance is not an Arising civilization."
        );
        require(
            msg.value == price,
            "Civilizations: Tx doesn't include enough to pay the mint"
        );
        require(
            _canMint(msg.sender),
            "Civilizations: address used cannot mint"
        );
        Address.sendValue(payments_receiver, price);
        IBaseERC721(_instance).mint(msg.sender);
        _addMinted(msg.sender);
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

    /** @dev Returns the list of civilizations.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function getCivilizations(address _instance)
        public
        view
        returns (address[] memory)
    {
        require(
            _instance != address(0),
            "Civilizations: instance address is null"
        );
        return civilizations;
    }

    /** @dev Returns a unique token ID from a collection.
     *  @param _instance  Address of the `BaseERC721` instance.
     *  @param _id        The ID of the token inside the collection.
     */
    function getTokenID(address _instance, uint256 _id)
        public
        view
        returns (string memory)
    {
        require(
            _instance != address(0),
            "Civilizations: instance address is null"
        );
        uint256 id = _civilizations[_instance];
        require(
            id != 0,
            "Civilizations: instance is not an Arising civilization."
        );
        require(
            IBaseERC721(_instance).exists(_id),
            "Civilizations: token id is not minted."
        );
        return
            string(
                abi.encodePacked(
                    Strings.toString(id),
                    "-",
                    Strings.toString(_id)
                )
            );
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
}