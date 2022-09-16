// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title IBaseERC721
 * @notice Interface for the {BaseERC721} contract.
 */
interface IBaseERC721 {
    /** @notice See {BaseERC721.mint} */
    function mint(address _to) external;

    /** @notice See {BaseERC721.isApprovedOrOwner} */
    function isApprovedOrOwner(address _spender, uint256 _id)
        external
        view
        returns (bool);

    /** @notice See {BaseERC721.exists} */
    function exists(uint256 _id) external view returns (bool);
}
