// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title Platinum
 * @notice This contract is an instance of {BaseFungibleItem} to serve as an asset for the ecosystem.
 */
contract Platinum is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Platinum",
            "PLATINUM",
            "https://playarising.com/material/raw/platinum.png",
            _civilizations
        )
    {}
}
