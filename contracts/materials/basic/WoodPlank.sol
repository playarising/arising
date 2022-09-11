// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `WoodPlank` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract WoodPlank is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Wood Plank",
            "aWOODPLANK",
            "https://playarising.com/material/basic/woodplank.png",
            _civilizations
        )
    {}
}
