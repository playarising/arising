// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Ironstone` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract Ironstone is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Ironstone",
            "IRONSTONE",
            "https://playarising.com/material/basic/ironstone.png",
            _civilizations
        )
    {}
}
