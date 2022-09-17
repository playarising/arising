// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title Ironstone
 * @notice This contract is an instance of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) to serve as an asset for the ecosystem.
 */
contract Ironstone is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the {Civilizations} instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Ironstone",
            "IRONSTONE",
            "https://playarising.com/material/basic/ironstone.png",
            _civilizations
        )
    {}
}
