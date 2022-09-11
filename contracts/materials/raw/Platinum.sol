// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Platinum` is a fungible item resource for the Arising ecosystem.
 */
contract Platinum is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Platinum",
            "aPLATINUM",
            "https://playarising.com/material/raw/platinum.png",
            _civilizations
        )
    {}
}
