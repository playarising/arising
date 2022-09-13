// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Wool` is a fungible item resource for the Arising ecosystem.
 */
contract Wool is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
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
