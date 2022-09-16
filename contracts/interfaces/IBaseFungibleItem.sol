// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title IBaseFungibleItem
 * @notice Interface for the {BaseFungibleItem} contract.
 */
interface IBaseFungibleItem {
    /** @notice See {BaseFungibleItem.mintTo} */
    function mintTo(bytes memory _id, uint256 _amount) external;

    /** @notice See {BaseFungibleItem.consume} */
    function consume(bytes memory _id, uint256 _amount) external;

    /** @notice See {BaseFungibleItem.wrap} */
    function wrap(bytes memory _id, uint256 _amount) external;

    /** @notice See {BaseFungibleItem.unwrap} */
    function unwrap(bytes memory _id, uint256 _amount) external;

    /** @notice See {BaseFungibleItem.balanceOf} */
    function balanceOf(bytes memory _id) external view returns (uint256);
}
