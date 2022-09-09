// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "../interfaces/IMintGuard.sol";
import "../interfaces/IBaseERC721.sol";

contract BaseERC721 is ERC721Enumerable, Ownable, IBaseERC721 {
    // Mint guard.
    address guard;

    // Payments receiver.
    address payable payments_receiver;

    // Base URI.
    string public baseURI;

    // Max available supply for the collection.
    uint256 public cap;

    // Boolean to initialize the mint capabilities.
    bool public initialized;

    // Price for each mint (in MATIC in wei)
    uint256 public price;

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
        initialized = true;
    }

    /**
     * @dev Set the mint price (in USD with 8 decimals)
     */
    function setPrice(uint256 _price) public onlyOwner {
        price = _price;
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
    function mint() public payable onlyInitialized {
        require(
            totalSupply() < cap,
            "BaseERC721: Max supply reached, wait for more tokens to be available"
        );
        require(
            !IMintGuard(guard).hasMinted(msg.sender),
            "BaseERC721: Address has already minted"
        );
        require(
            msg.value == price,
            "BaseERC721: Tx doesn't include enough to pay the mint"
        );
        IMintGuard(guard).setMinter(msg.sender);
        Address.sendValue(payments_receiver, price);
        _safeMint(msg.sender, totalSupply() + 1);
    }

    /**
     * @dev Overrides the ERC721 _baseURI with an URI specified over the constructor.
     */
    function _baseURI() internal view virtual override returns (string memory) {
        return baseURI;
    }
}
