// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../base/BaseFungibleItem.sol";

/**
 * @dev `Coal` is a fungible item resource for the Arising ecosystem.
 */
contract Coal is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     */
    constructor() BaseFungibleItem("Arising: Coal", "aCOAL") {}
}
