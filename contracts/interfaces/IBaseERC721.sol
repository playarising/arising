// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

/**
 * @title IBaseERC721
 * @notice Interface for the [BaseERC721](/docs/base/BaseERC721.md) contract.
 */
interface IBaseERC721 {
    /** @notice See [BaseERC721#addAuthority](/docs/base/BaseERC721.md#addAuthority) */
    function addAuthority(address _authority) external;

    /** @notice See [BaseERC721#removeAuthority](/docs/base/BaseERC721.md#removeAuthority) */
    function removeAuthority(address _authority) external;

    /** @notice See [BaseERC721#mint](/docs/base/BaseERC721.md#mint) */
    function mint(address _to) external returns (uint256 _token_id);

    /** @notice See [BaseERC721#isApprovedOrOwner](/docs/base/BaseERC721.md#isApprovedOrOwner) */
    function isApprovedOrOwner(
        address _spender,
        uint256 _token_id
    ) external view returns (bool _approved);

    /** @notice See [BaseERC721#exists](/docs/base/BaseERC721.md#exists) */
    function exists(uint256 _token_id) external view returns (bool _exist);
}
