// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @dev `Tarki` is the `BaseERC721` instance for the Tarki.
 */
contract Tarki is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Tark'i",
            "ARISING",
            "https://characters.playarising.com/tarki/"
        )
    {}
}
