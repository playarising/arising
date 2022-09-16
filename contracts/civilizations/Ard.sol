// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../base/BaseERC721.sol";

/**
 * @title Ard
 * @notice Implementation of the {BaseERC721} contract for the Ard civilization.
 */
contract Ard is BaseERC721 {
    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     */
    constructor()
        BaseERC721(
            "Arising: Ard",
            "ARISING",
            "https://characters.playarising.com/ard/"
        )
    {}
}
