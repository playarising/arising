// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../base/BaseFungibleItem.sol";

/**
 * @dev `Gold` is a fungible item to serve as base currency for the Arising ecosystem.
 */
contract Gold is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _minter    The address of the authorized minter.
     */
    constructor(address _minter)
        BaseFungibleItem("Arising: Gold", "aGOLD", _minter)
    {}
}
