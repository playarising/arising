// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseGadgetToken.sol";

/**
 * @dev `Vitalizer` is a contract serve as a self-served gadget to perform base stats increase for `Stats`.
 */
contract Vitalizer is BaseGadgetToken {
    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _token   Address of the token to charge.
     * @param _price   Amount of tokens to charge.
     */
    constructor(address _token, uint256 _price)
        BaseGadgetToken("Arising: Vitalizer Token", "VITALIZER", _token, _price)
    {}
}
