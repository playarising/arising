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
     */
    constructor() BaseFungibleItem("Arising: Wood", "aWOOD") {}
}
