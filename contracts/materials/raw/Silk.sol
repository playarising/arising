// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Silk` is a fungible item resource for the Arising ecosystem.
 */
contract Silk is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Silk",
            "SILK",
            "https://playarising.com/material/raw/silk.png",
            _civilizations
        )
    {}
}
