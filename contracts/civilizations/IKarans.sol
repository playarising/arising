// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @dev `IKarans` is the `BaseERC721` instance for the IKarans.
 */
contract IKarans is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: I'Karans",
            "ARISING",
            "https://characters.playarising.com/ikarans/",
            2500
        )
    {}
}
