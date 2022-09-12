// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";
import "../interfaces/IBaseERC721.sol";

/**
 * @dev `BaseERC721` is the base ERC721 token for Arising Civilizations.
 */
contract BaseERC721 is ERC721Enumerable, Ownable, IBaseERC721 {
    // =============================================== Storage ========================================================

    /** @dev Base URI of the token metadata. **/
    string public baseURI;

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _name                 The name of the collection.
     * @param _symbol               The symbol of the collection.
     * @param _uri                  The base URI for the tokens metadata.
     */
    constructor(
        string memory _name,
        string memory _symbol,
        string memory _uri
    ) ERC721(_name, _symbol) {
        baseURI = _uri;
    }

    /** @dev Mints a new token to `msg.sender`. */
    function mint(address to) public onlyOwner {
        _safeMint(to, totalSupply() + 1);
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

    /**
     * @dev Returns a bool if a tokenId is already minted in the collection.
     * @param tokenId   The id of the token.
     */
    function exists(uint256 tokenId) public view returns (bool) {
        return _exists(tokenId);
    }

    // =============================================== Internal =======================================================

    /** @dev Overrides the ERC721 `_baseURI` function with an URI specified over the constructor. */
    function _baseURI() internal view virtual override returns (string memory) {
        return baseURI;
    }
}
