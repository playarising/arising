// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `SilkFabric` is a fungible item to serve as a usable resource for the Arising ecosystem.
 */
contract SilkFabric is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations  The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Silk Fabric",
            "SILK_FABRIC",
            "https://playarising.com/material/basic/silk_fabric.png",
            _civilizations
        )
    {}
}
