// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../base/BaseFungibleItem.sol";

/**
 * @dev `Stone` is a fungible item resource for the Arising ecosystem.
 */
contract Stone is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     */
    constructor(address _minter) BaseFungibleItem("Arising: Stone", "aSTONE") {}
}
