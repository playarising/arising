// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Silver` is a fungible item resource for the Arising ecosystem.
 */
contract Silver is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Silver",
            "aSILVER",
            "https://playarising.com/material/raw/silver.png",
            _civilizations
        )
    {}
}
