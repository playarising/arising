// SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/IMintGuard.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

contract BaseERC721 is ERC721Enumerable, Ownable {
    // Mint guard.
    address guard;

    // Payments receiver.
    address payable payments_receiver;

    // Base URI.
    string baseURI;

    // Max available supply for the collection.
    uint256 cap;

    // Boolean to initialize the mint capabilities.
    bool initialized;

    // Price for each mint (in USD with 8 decimals)
    uint256 price;

    // Price feed oracle
    address internal priceFeed;

    /**
     * @dev Throws if called when the contract is not initialized.
     */
    modifier onlyInitialized() {
        require(initialized, "BaseERC721: contract is not initialized");
        _;
    }

    /**
     * @dev Initializes the contract by setting a `_name`, `_symbol` and define the `_guard`, the `_uri`, the initial `_cap` and the `_payments_receiver` for the token collection.
     */
    constructor(
        string memory _name,
        string memory _symbol,
        address _guard,
        string memory _uri,
        uint256 _cap,
        address payable _payments_receiver
    ) ERC721(_name, _symbol) {
        guard = _guard;
        baseURI = _uri;
        cap = _cap;
        initialized = false;
        payments_receiver = _payments_receiver;
    }

    /**
     * @dev Enabled the collection to be minted
     */
    function setInitialized() public onlyOwner {
        require(price != 0, "BaseERC721: can't initialize when price is 0");
        require(
            priceFeed != address(0),
            "BaseERC721: can't initialize when priceFeed is not set"
        );
        initialized = true;
    }

    /**
     * @dev Set the mint price (in USD with 8 decimals)
     */
    function setPrice(uint256 _price) public onlyOwner {
        price = _price;
    }

    /**
     * @dev Set the price feed oracle
     */
    function setPriceFeed(address _priceFeed) public onlyOwner {
        priceFeed = _priceFeed;
    }

    /**
     * @dev Modifies the max supply for the collection
     */
    function setCap(uint256 _cap) public onlyOwner {
        cap = _cap;
    }

    /**
     * @dev Creates a new token for the msg.sender
     */
    function mint() public payable {
        require(
            totalSupply() <= cap,
            "BaseERC721: Max supply reached, wait for more tokens to be available"
        );
        require(
            !IMintGuard(guard).hasMinted(msg.sender),
            "BaseERC721: Address has already minted"
        );
        int256 cost = getMintCost();
        require(
            msg.value == uint256(cost),
            "BaseERC721: Tx doesn't include enough to pay the mint"
        );
        Address.sendValue(payments_receiver, uint256(cost));
        _safeMint(msg.sender, totalSupply() + 1);
    }

    /**
     * @dev Overrides the ERC721 _baseURI with an URI specified over the constructor.
     */
    function _baseURI() internal view virtual override returns (string memory) {
        return baseURI;
    }

    /**
     * @dev Returns the total mint cost in FTM from the price feed.
     */
    function getMintCost() public view returns (int256) {
        (, int256 _feedPrice, , , ) = AggregatorV3Interface(priceFeed)
            .latestRoundData();
        return (int256(price) / _feedPrice) * 1 ether;
    }
}
