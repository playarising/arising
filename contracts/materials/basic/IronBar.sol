// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `IronBar` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract IronBar is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Iron Bar",
            "IRON_BAR",
            "https://playarising.com/material/basic/iron_bar.png",
            _civilizations
        )
    {}
}
