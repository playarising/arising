// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Wood` is a fungible item resource for the Arising ecosystem.
 */
contract Wood is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Wood",
            "aWOOD",
            "https://playarising.com/material/raw/wood.png",
            _civilizations
        )
    {}
}
