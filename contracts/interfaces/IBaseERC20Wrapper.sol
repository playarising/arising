// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title IBaseERC20Wrapper
 * @notice Interface for the {BaseERC20Wrapper} contract.
 */
interface IBaseERC20Wrapper {
    /** @notice See {BaseERC20Wrapper.mint} */
    function mint(address _to, uint256 _amount) external;
}
