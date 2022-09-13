// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Cotton` is a fungible item resource for the Arising ecosystem.
 */
contract Cotton is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Cotton",
            "COTTON",
            "https://playarising.com/material/raw/cotton.png",
            _civilizations
        )
    {}
}
