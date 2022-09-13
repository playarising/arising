// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Iron` is a fungible item resource for the Arising ecosystem.
 */
contract Iron is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Iron",
            "IRON",
            "https://playarising.com/material/raw/iron.png",
            _civilizations
        )
    {}
}
