// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

import "../interfaces/IBaseERC721.sol";
import "../interfaces/ICivilizations.sol";

/**
 * @title BaseERC721
 * @notice This contract is a `ERC721Enumerable` implementation for the different civilizations.
 * Exposes the mint function to the owner and some check functions.
 *
 * @notice Implementation of the [IBaseERC721](/docs/interfaces/IBaseERC721.md) interface.
 */
contract BaseERC721 is IBaseERC721, ERC721Enumerable, Ownable {
    // =============================================== Storage ========================================================

    /** @notice Constant for the base url of the token metadata. */
    string public baseURI;

    /** @notice Address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;

    /** @notice Map to store the list of authorized addresses to mint tokens. */
    mapping(address => bool) authorized;

    // ============================================== Modifiers =======================================================

    /** @notice Checks against if the `msg.sender` is authorized to mint tokens. */
    modifier onlyAuthorized() {
        require(
            authorized[msg.sender],
            "BaseERC721: onlyAuthorized() msg.sender not authorized."
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _name             Name of the `ERC721` token.
     * @param _symbol           Symbol of the `ERC721` token.
     * @param _uri              Base url for the tokens metadata.
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     */
    constructor(
        string memory _name,
        string memory _symbol,
        string memory _uri,
        address _civilizations
    ) {
        baseURI = _uri;
        civilizations = _civilizations;
        authorized[msg.sender] = true;
    }

    /**
     * @notice Assigns a new address as an authority to mint tokens.
     *
     * Requirements:
     * @param _authority    Address to give authority.
     */
    function addAuthority(address _authority) public onlyOwner {
        authorized[_authority] = true;
    }

    /**
     * @notice Removes an authority to mint tokens.
     *
     * Requirements:
     * @param _authority    Address to give authority.
     */
    function removeAuthority(address _authority) public onlyOwner {
        require(
            authorized[_authority],
            "Experience: removeAuthority() address is not authorized."
        );
        authorized[_authority] = false;
    }

    /**
     * @notice Creates tokens to the address provided.
     *
     * Requirements:
     * @param _to           Address that receives the tokens.
     *
     * @return _token_id    The ID of the new token.
     */
    function mint(address _to) public onlyAuthorized returns (uint256) {
        uint256 _token_id = totalSupply() + 1;
        _safeMint(_to, _token_id);
        return _token_id;
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to check if an address is allowed to access a token.
     *
     * Requirements:
     * @param _spender      Address that will access the token.
     * @param _token_id     ID of the token to be accessed.
     *
     * @return _approved    Boolean to return if the address is allowed to access the token.
     */
    function isApprovedOrOwner(
        address _spender,
        uint256 _token_id
    ) public view virtual returns (bool _approved) {
        address _owner = ownerOf(_token_id);
        return (_spender == _owner ||
            isApprovedForAll(_owner, _spender) ||
            getApproved(_token_id) == _spender);
    }

    /**
     * @notice External function to check if a token id is minted.
     *
     * Requirements:
     * @param _token_id     ID of the token to be checked.
     *
     * @return _exist       Boolean to check if the token is already minted.
     */
    function exists(uint256 _token_id) public view returns (bool _exist) {
        return _ownerOf(_token_id) != address(0);
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

    /** @notice Internal function that overrides the `ERC721_afterTokenTransfer` function to emit the transfer event.  */
    function _afterTokenTransfer(
        address from,
        address to,
        uint256 tokenId
    ) internal virtual override {
        super._afterTokenTransfer(from, to, tokenId);
        ICivilizations(civilizations).transfer(from, to, tokenId);
    }
}
