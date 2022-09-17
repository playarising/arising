// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title Bronze
 * @notice This contract is an instance of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) to serve as an asset for the ecosystem.
 */
contract Bronze is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Bronze",
            "BRONZE",
            "https://playarising.com/material/raw/bronze.png",
            _civilizations
        )
    {}
}
