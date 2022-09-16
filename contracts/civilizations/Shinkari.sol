// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @title Shinkari
 * @notice Implementation of the {BaseERC721} contract for the Shinkari civilization.
 */
contract Shinkari is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Shinkari",
            "ARISING",
            "https://characters.playarising.com/shinkari/"
        )
    {}
}
