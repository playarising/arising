// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `AdamantineBar` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract AdamantineBar is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Adamantine Bar",
            "aADAMANTINEBAR",
            "https://playarising.com/material/basic/adamantinebar.png",
            _civilizations
        )
    {}
}
