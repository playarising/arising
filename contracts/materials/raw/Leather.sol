// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Leather` is a fungible item resource for the Arising ecosystem.
 */
contract Leather is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Leather",
            "LEATHER",
            "https://playarising.com/material/raw/leather.png",
            _civilizations
        )
    {}
}
