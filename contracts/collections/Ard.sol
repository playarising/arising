// SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

import "../base/BaseERC721.sol";

contract ArdCharacters is BaseERC721 {
    /**
     * @dev Initializes the contract by setting a `_guard`, `_cap` and the `_payments_receiver` for the token collection.
     */
    constructor(
        address _guard,
        uint256 _cap,
        address payable _payments_receiver
    )
        BaseERC721(
            "Arising: Ard",
            "ARISING",
            _guard,
            "https://characters.playarising.com/ard/",
            _cap,
            _payments_receiver
        )
    {}
}