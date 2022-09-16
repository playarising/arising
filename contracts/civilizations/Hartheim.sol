// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @title Hartheim
 * @notice Implementation of the {BaseERC721} contract for the Hartheim civilization.
 */
contract Hartheim is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Hartheim",
            "ARISING",
            "https://characters.playarising.com/hartheim/"
        )
    {}
}
