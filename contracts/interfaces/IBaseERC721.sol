// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

interface IBaseERC721 {
    function setInitialized() external;

    function setPrice(uint256 _price) external;

    function setCap(uint256 _cap) external;

    function mint() external payable;

    function isApprovedOrOwner(address spender, uint256 tokenId)
        external
        view
        returns (bool);
}
