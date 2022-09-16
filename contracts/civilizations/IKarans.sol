// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @title IKarans
 * @notice Implementation of the {BaseERC721} contract for the I'Karans civilization.
 */
contract IKarans is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: I'Karans",
            "ARISING",
            "https://characters.playarising.com/ikarans/"
        )
    {}
}
