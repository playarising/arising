// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @title Wool
 * @notice This contract is an instance of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) to serve as an asset for the ecosystem.
 */
contract Wool is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    Address of the [Civilizations](/docs/core/Civilizations.md) instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Wool",
            "WOOL",
            "https://playarising.com/material/raw/wool.png",
            _civilizations
        )
    {}
}
