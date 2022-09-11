// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `SteelBar` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract SteelBar is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Steel Bar",
            "aSTEELBAR",
            "https://playarising.com/material/basic/steelbar.png",
            _civilizations
        )
    {}
}
