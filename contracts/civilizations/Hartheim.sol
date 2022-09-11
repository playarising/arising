// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @dev `Hartheim` is the `BaseERC721` instance for the Hartheim.
 */
contract Hartheim is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Hartheim",
            "ARISING",
            "https://characters.playarising.com/hartheim/",
            1000
        )
    {}
}
