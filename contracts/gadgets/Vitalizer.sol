// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseGadgetToken.sol";

/**
 * @title Vitalizer
 * @notice This contract is an instance of {BaseGadgetToken} to reclaim sacrificed points on the {Stats} contract.
 */
contract Vitalizer is BaseGadgetToken {
    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     *
     * Requirements:
     * @param _token   Address of the token used to purchase.
     * @param _price   Price for each token.
     */
    constructor(address _token, uint256 _price)
        BaseGadgetToken("Arising: Vitalizer Token", "VITALIZER", _token, _price)
    {}
}
