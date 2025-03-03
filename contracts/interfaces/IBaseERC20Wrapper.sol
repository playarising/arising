// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

/**
 * @title IBaseERC20Wrapper
 * @notice Interface for the [BaseERC20Wrapper](/docs/base/BaseERC20Wrapper.md) contract.
 */
interface IBaseERC20Wrapper {
    /** @notice See [BaseERC20Wrapper#mint](/docs/base/BaseERC20Wrapper.md#mint) */
    function mint(address _to, uint256 _amount) external;
}
