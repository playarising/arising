// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `Cobalt` is a fungible item resource for the Arising ecosystem.
 */
contract Cobalt is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Cobalt",
            "COBALT",
            "https://playarising.com/material/raw/cobalt.png",
            _civilizations
        )
    {}
}
