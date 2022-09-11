// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseFungibleItem.sol";

/**
 * @dev `Coal` is a fungible item resource for the Arising ecosystem.
 */
contract Coal is BaseFungibleItem {
    // =============================================== Setters ========================================================
    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _civilizations)
        BaseFungibleItem("Arising: Coal", "aCOAL", _civilizations)
    {}
}
