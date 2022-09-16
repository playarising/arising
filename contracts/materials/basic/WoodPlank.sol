// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title WoodPlank
 * @notice This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.
 */
contract WoodPlank is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Wood Plank",
            "WOOD_PLANK",
            "https://playarising.com/material/basic/wood_plank.png",
            _civilizations
        )
    {}
}
