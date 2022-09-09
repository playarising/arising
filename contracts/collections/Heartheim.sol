// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../base/BaseERC721.sol";

/*
 * HeartheimCharacters the Hartheim collection token for Arising.
 */
contract HeartheimCharacters is BaseERC721 {
    /**
     * @dev Initializes the contract by setting a `_guard`, `_cap` and the `_payments_receiver` for the token collection.
     */
    constructor(
        address _guard,
        uint256 _cap,
        address payable _payments_receiver
    )
        BaseERC721(
            "Arising: Heartheim",
            "ARISING",
            _guard,
            "https://characters.playarising.com/heartheim/",
            _cap,
            _payments_receiver
        )
    {}
}
