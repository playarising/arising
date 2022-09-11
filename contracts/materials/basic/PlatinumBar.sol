// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `PlatinumBar` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract PlatinumBar is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Platinum Bar",
            "aPLATINUMBAR",
            "https://playarising.com/material/basic/platinumbar.png",
            _civilizations
        )
    {}
}
