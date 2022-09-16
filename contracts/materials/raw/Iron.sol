// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title Iron
 * @notice This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.
 */
contract Iron is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Iron",
            "IRON",
            "https://playarising.com/material/raw/iron.png",
            _civilizations
        )
    {}
}
