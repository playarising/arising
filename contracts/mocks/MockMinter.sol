// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../interfaces/IMintGuard.sol";
import "../interfaces/IBaseERC721.sol";

contract MockMinter {
    // MintGuard
    address guard;

    // BaseErc721
    address baseErc721;

    constructor(address _guard, address _base) {
        guard = _guard;
        baseErc721 = _base;
    }

    /**
     * @dev This function will imitate writing to the MintGuard contract
     */
    function mintMock() public {
        IMintGuard(guard).setMinter(msg.sender);
    }

    /**
     * @dev This function will try to mint a character from a contract
     */
    function testMint() public {
        IBaseERC721(baseErc721).mint();
    }
}
