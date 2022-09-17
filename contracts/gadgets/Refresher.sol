// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseGadgetToken.sol";

/**
 * @title Refresher
 * @notice This contract is an instance of {BaseGadgetToken} to perform paid refreshes for the [Stats](/docs/core/Stats.md) contract.
 */
contract Refresher is BaseGadgetToken {
    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     *
     * Requirements:
     * @param _token   Address of the token used to purchase.
     * @param _price   Price for each token.
     */
    constructor(address _token, uint256 _price)
        BaseGadgetToken("Arising: Refresh Token", "REFRESHER", _token, _price)
    {}
}
