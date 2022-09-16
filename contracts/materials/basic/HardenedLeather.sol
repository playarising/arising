// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title HardenedLeather
 * @notice This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.
 */
contract HardenedLeather is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Hardened Leather",
            "HARDENED_LEATHER",
            "https://playarising.com/material/basic/hardened_leather.png",
            _civilizations
        )
    {}
}
