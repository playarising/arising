// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @dev `Ard` is the `BaseERC721` instance for the people of Ard.
 */
contract Ard is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Ard",
            "ARISING",
            "https://characters.playarising.com/ard/"
        )
    {}
}
