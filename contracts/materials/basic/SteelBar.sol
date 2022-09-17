// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title SteelBar
 * @notice This contract is an instance of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) to serve as an asset for the ecosystem.
 */
contract SteelBar is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Steel Bar",
            "STEEL_BAR",
            "https://playarising.com/material/basic/steel_bar.png",
            _civilizations
        )
    {}
}
