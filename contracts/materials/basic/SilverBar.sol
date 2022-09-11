// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `SilverBar` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract SilverBar is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Silver Bar",
            "aSILVERBAR",
            "https://playarising.com/material/basic/silverbar.png",
            _civilizations
        )
    {}
}
