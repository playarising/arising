// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

import "../interfaces/IBaseERC721.sol";

/**
 * @title BaseERC721
 * @notice This contract is a `ERC721Enumerable` implementation for the different civilizations.
 * Exposes the mint function to the owner and some check functions.
 *
 * @dev Implementation of the [IBaseERC721](/docs/interfaces/IBaseERC721.md) interface.
 */
contract BaseERC721 is IBaseERC721, Ownable, ERC721Enumerable {
    // =============================================== Storage ========================================================

    /** @notice Constant for the base url of the token metadata. */
    string public baseURI;

    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _name     Name of the `ERC721` token.
     * @param _symbol   Symbol of the `ERC721` token.
     * @param _uri      Base url for the tokens metadata.
     */
    constructor(
        string memory _name,
        string memory _symbol,
        string memory _uri
    ) ERC721(_name, _symbol) {
        baseURI = _uri;
    }

    /**
     * @notice Creates tokens to the address provided.
     *
     * Requirements:
     * @param to    Address that receives the tokens.
     */
    function mint(address to) public onlyOwner {
        _safeMint(to, totalSupply() + 1);
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to check if an address is allowed to access a token.
     *
     * Requirements:
     * @param spender       Address that will access the token.
     * @param id            ID of the token to be accessed.
     *
     * @return _approved    Boolean to return if the address is allowed to access the token.
     */
    function isApprovedOrOwner(address spender, uint256 id)
        public
        view
        virtual
        returns (bool _approved)
    {
        address owner = ownerOf(id);
        return (spender == owner ||
            isApprovedForAll(owner, spender) ||
            getApproved(id) == spender);
    }

    /**
     * @notice External function to check if a token id is minted.
     *
     * Requirements:
     * @param id            ID of the token to be checked.
     *
     * @return _exist       Boolean to check if the token is already minted.
     */
    function exists(uint256 id) public view returns (bool _exist) {
        return _exists(id);
    }

    // =============================================== Internal =======================================================

    /**
     * @notice Internal function that overrides the `ERC721_baseURI` function
     * with an URI specified over the constructor.
     *
     * @return _uri Base URL from the constructor
     */
    function _baseURI()
        internal
        view
        virtual
        override
        returns (string memory _uri)
    {
        return baseURI;
    }
}
