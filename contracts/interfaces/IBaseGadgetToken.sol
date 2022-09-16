// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title IBaseGadgetToken
 * @notice Interface for the {BaseGadgetToken} contract.
 */
interface IBaseGadgetToken {
    /** @notice See {BaseGadgetToken.pause} */
    function pause() external;

    /** @notice See {BaseGadgetToken.unpause} */
    function unpause() external;

    /** @notice See {BaseGadgetToken.setPrice} */
    function setPrice(uint256 _price) external;

    /** @notice See {BaseGadgetToken.setToken} */
    function setToken(address _token) external;

    /** @notice See {BaseGadgetToken.mint} */
    function mint(uint256 _amount) external;

    /** @notice See {BaseGadgetToken.mintFree} */
    function mintFree(address _receiver, uint256 _amount) external;

    /** @notice See {BaseGadgetToken.withdraw} */
    function withdraw() external;

    /** @notice See {BaseGadgetToken.getTotalCost} */
    function getTotalCost(uint256 _amount) external returns (uint256);
}
