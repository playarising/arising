// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @title Zhand
 * @notice Implementation of the {BaseERC721} contract for the Zhand civilization.
 */
contract Zhand is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Zhand",
            "ARISING",
            "https://characters.playarising.com/zhand/"
        )
    {}
}
