// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../../base/BaseFungibleItem.sol";

/**
 * @dev `CobaltBar` is a fungible item to serve as a craftable resource for the Arising ecosystem.
 */
contract CobaltBar is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem(
            "Arising: Cobalt Bar",
            "aCOBALTBAR",
            "https://playarising.com/material/basic/cobaltbar.png",
            _civilizations
        )
    {}
}
