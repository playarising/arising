// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `CottonFabric` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract CottonFabric is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Cotton Fabric",
            "COTTON_FABRIC",
            "https://playarising.com/material/basic/cotton_fabric.png",
            _civilizations
        )
    {}
}
