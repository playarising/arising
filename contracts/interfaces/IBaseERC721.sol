// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

interface IBaseERC721 {
    function mint(address to) external;

    function isApprovedOrOwner(address spender, uint256 tokenId)
        external
        view
        returns (bool);

    function exists(uint256 tokenId) external view returns (bool);
}
