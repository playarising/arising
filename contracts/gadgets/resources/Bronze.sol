// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Bronze` is a fungible item resource for the Arising ecosystem.
 */
contract Bronze is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Bronze",
            "aBRONZE",
            "https://playarising.com/gadgets/raw/bronze.png",
            _civilizations
        )
    {}
}
