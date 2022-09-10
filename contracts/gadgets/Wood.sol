// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../base/BaseFungibleItem.sol";

/**
 * @dev `Wood` is a fungible item resource for the Arising ecosystem.
 */
contract Wood is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _minter    The address of the authorized minter.
     */
    constructor(address _minter)
        BaseFungibleItem("Arising: Wood", "aWOOD", _minter)
    {}
}
