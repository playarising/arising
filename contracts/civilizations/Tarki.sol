// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @title Tarki
 * @notice Implementation of the {BaseERC721} contract for the Tark'i civilization.
 */
contract Tarki is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Tark'i",
            "ARISING",
            "https://characters.playarising.com/tarki/"
        )
    {}
}
