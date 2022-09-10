// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

interface IBaseFungibleItem {
    function mintTo(bytes memory id, uint256 amount) external;

    function consume(bytes memory id, uint256 amount) external;

    function balanceOf(bytes memory id) external view returns (uint256);
}
