// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

/**
 * @title IBaseGadgetToken
 * @notice Interface for the [BaseGadgetToken](/docs/base/BaseGadgetToken.md) contract.
 */
interface IBaseGadgetToken {
    /** @notice See [BaseGadgetToken#pause](/docs/base/BaseGadgetToken.md#pause) */
    function pause() external;

    /** @notice See [BaseGadgetToken#unpause](/docs/base/BaseGadgetToken.md#unpause)*/
    function unpause() external;

    /** @notice See [BaseGadgetToken#setPrice](/docs/base/BaseGadgetToken.md#setPrice) */
    function setPrice(uint256 _price) external;

    /** @notice See [BaseGadgetToken#setToken](/docs/base/BaseGadgetToken.md#setToken) */
    function setToken(address _token) external;

    /** @notice See [BaseGadgetToken#mint](/docs/base/BaseGadgetToken.md#mint) */
    function mint(uint256 _amount) external;

    /** @notice See [BaseGadgetToken#mintFree](/docs/base/BaseGadgetToken.md#mintFree) */
    function mintFree(address _receiver, uint256 _amount) external;

    /** @notice See [BaseGadgetToken#getTotalCost](/docs/base/BaseGadgetToken.md#getTotalCost) */
    function getTotalCost(uint256 _amount) external returns (uint256 _cost);
}
