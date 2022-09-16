// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseFungibleItem.sol";

/**
 * @title Gold
 * @notice This contract is an instance of {BaseFungibleItem} to serve as the main currency of the whole internal ecosystem.
 */
contract Gold is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Gold",
            "GOLD",
            "https://playarising.com/gadgets/gold.png",
            _civilizations
        )
    {}
}
