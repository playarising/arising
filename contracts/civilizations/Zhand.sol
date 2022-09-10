// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../base/BaseERC721.sol";

/**
 * @dev `Zhand` is the `BaseERC721` instance for the Zhand.
 */
contract Zhand is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Zhand",
            "ARISING",
            "https://characters.playarising.com/zhand/",
            1000
        )
    {}
}
