// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title SilkFabric
 * @notice This contract is an instance of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) to serve as an asset for the ecosystem.
 */
contract SilkFabric is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the [Civilizations](/docs/core/Civilizations.md) instance.
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
