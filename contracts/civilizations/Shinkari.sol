// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../base/BaseERC721.sol";

/**
 * @dev `Shinkari` is the `BaseERC721` instance for the Shinkari.
 */
contract Shinkari is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Shinkari",
            "ARISING",
            "https://characters.playarising.com/shinkari/",
            4500
        )
    {}
}
