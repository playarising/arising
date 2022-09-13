// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `HardenedLeather` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract HardenedLeather is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Hardened Leather",
            "HARDENED_LEATHER",
            "https://playarising.com/material/basic/hardened_leather.png",
            _civilizations
        )
    {}
}
