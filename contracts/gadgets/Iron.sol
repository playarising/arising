// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../base/BaseFungibleItem.sol";

/**
 * @dev `Iron` is a fungible item resource for the Arising ecosystem.
 */
contract Iron is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _minter    The address of the authorized minter.
     */
    constructor(address _minter)
        BaseFungibleItem("Arising: Iron", "aIRON", _minter)
    {}
}
