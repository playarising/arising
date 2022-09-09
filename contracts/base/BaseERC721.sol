// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/token/ERC721/IERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Address.sol";
import "../interfaces/IMintGuard.sol";
import "../interfaces/IBaseERC721.sol";

/**
 * @dev `BaseERC721` is the base ERC721 token for Arising Civilizations.
 */
contract BaseERC721 is ERC721Enumerable, Ownable, IBaseERC721 {
    // =============================================== Storage ========================================================

    /** @dev The address of the `MintGuard` instance. **/
    address guard;

    /** @dev The receiver address. **/
    address payable payments_receiver;

    /** @dev Base URI of the token metadata. **/
    string public baseURI;

    /** @dev Max token supply. **/
    uint256 public cap;

    /** @dev Boolean to check if the implementation is usable. **/
    bool public initialized;

    /** @dev Price of each mint in MATIC in wei. **/
    uint256 public price;

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
     * @param _name                 The name of the collection.
     * @param _symbol               The symbol of the collection.
     * @param _guard                The `MintGuard` instance address.
     * @param _uri                  The base URI for the tokens metadata.
     * @param _cap                  The max supply of the token.
     * @param _payments_receiver    The address that will receive the payments.
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

    /** @dev Enables the `BaseERC721` implementation. */
    function setInitialized() public onlyOwner {
        require(price != 0, "BaseERC721: can't initialize when price is 0");
        initialized = true;
    }

    /** @dev Sets the minting price.
     *  @param _price   Price in MATIC in wei.
     */
    function setPrice(uint256 _price) public onlyOwner {
        price = _price;
    }

    /** @dev Sets the max supply of the collection.
     *  @param _cap     Max supply.
     */
    function setCap(uint256 _cap) public onlyOwner {
        cap = _cap;
    }

    /** @dev Mints a new token to `msg.sender`. */
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

    // =============================================== Getters ========================================================

    /**
     * @dev Returns whether `spender` is allowed to manage `tokenId`.
     * @param spender   The account checking allowance.
     * @param tokenId   The id of the token.
     */
    function isApprovedOrOwner(address spender, uint256 tokenId)
        public
        view
        virtual
        returns (bool)
    {
        address owner = ownerOf(tokenId);
        return (spender == owner ||
            isApprovedForAll(owner, spender) ||
            getApproved(tokenId) == spender);
    }

    // =============================================== Internal =======================================================

    /** @dev Overrides the ERC721 `_baseURI` function with an URI specified over the constructor. */
    function _baseURI() internal view virtual override returns (string memory) {
        return baseURI;
    }
}
