// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title Stone
 * @notice This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.
 */
contract Stone is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Stone",
            "STONE",
            "https://playarising.com/material/raw/stone.png",
            _civilizations
        )
    {}
}
