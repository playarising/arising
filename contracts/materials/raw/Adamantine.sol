// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Adamantine` is a fungible item resource for the Arising ecosystem.
 */
contract Adamantine is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Adamantine",
            "aADAMANTINE",
            "https://playarising.com/material/raw/adamantine.png",
            _civilizations
        )
    {}
}
