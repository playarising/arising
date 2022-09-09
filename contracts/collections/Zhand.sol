// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../base/BaseERC721.sol";

/*
 * ZhandCharacters the Zhand collection token for Arising.
 */
contract ZhandCharacters is BaseERC721 {
    /**
     * @dev Initializes the contract by setting a `_guard`, `_cap` and the `_payments_receiver` for the token collection.
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
