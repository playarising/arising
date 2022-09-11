// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `GoldBar` is a fungible item to serve as a craftable resource for the Arising ecosystem.
 */
contract GoldBar is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Gold Bar",
            "aGOLDBAR",
            "https://playarising.com/material/basic/goldbar.png",
            _civilizations
        )
    {}
}
