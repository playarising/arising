// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `WoolFabric` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract WoolFabric is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Wool Fabric",
            "WOOL_FABRIC",
            "https://playarising.com/material/basic/wool_fabric.png",
            _civilizations
        )
    {}
}
