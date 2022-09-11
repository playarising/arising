// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Stone` is a fungible item resource for the Arising ecosystem.
 */
contract Stone is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Stone",
            "aSTONE",
            "https://playarising.com/material/raw/stone.png",
            _civilizations
        )
    {}
}
