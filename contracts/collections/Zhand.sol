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
     * @param _guard                  The `MintGuard` instance address.
     * @param _cap                    The max supply for the tokens.
     * @param _payments_receiver      The payments receiver address.
     */
    constructor(
        address _guard,
        uint256 _cap,
        address payable _payments_receiver
    )
        BaseERC721(
            "Arising: Zhand",
            "ARISING",
            _guard,
            "https://characters.playarising.com/zhand/",
            _cap,
            _payments_receiver
        )
    {}
}
