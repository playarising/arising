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
     */
    constructor() BaseFungibleItem("Arising: Iron", "aIRON") {}
}
