// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title WoolFabric
 * @notice This contract is an instance of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) to serve as an asset for the ecosystem.
 */
contract WoolFabric is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
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
