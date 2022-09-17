// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title IBaseFungibleItem
 * @notice Interface for the [BaseFungibleItem](/docs/base/BaseFungibleItem.md) contract.
 */
interface IBaseFungibleItem {
    /** @notice See [BaseFungibleItem](/docs/base/BaseFungibleItem.md#mintTo) */
    function mintTo(bytes memory _id, uint256 _amount) external;

    /** @notice See [BaseFungibleItem](/docs/base/BaseFungibleItem.md#consume) */
    function consume(bytes memory _id, uint256 _amount) external;

    /** @notice See [BaseFungibleItem](/docs/base/BaseFungibleItem.md#wrap) */
    function wrap(bytes memory _id, uint256 _amount) external;

    /** @notice See [BaseFungibleItem](/docs/base/BaseFungibleItem.md#unwrap) */
    function unwrap(bytes memory _id, uint256 _amount) external;

    /** @notice See [BaseFungibleItem](/docs/base/BaseFungibleItem.md#balanceOf) */
    function balanceOf(bytes memory _id)
        external
        view
        returns (uint256 _balance);
}
