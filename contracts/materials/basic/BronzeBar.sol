// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `BronzeBar` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract BronzeBar is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Bronze Bar",
            "aBRONZEBAR",
            "https://playarising.com/material/basic/bronzebar.png",
            _civilizations
        )
    {}
}
