// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../interfaces/IGuard.sol";
import "../interfaces/IBaseERC721.sol";

contract MockMinter {
    address guard;
    address baseErc721;

    constructor(address _guard, address _base) {
        guard = _guard;
        baseErc721 = _base;
    }

    function mintMock() public {
        IGuard(guard).setMinter(msg.sender);
    }

    function testMint() public {
        IBaseERC721(baseErc721).mint();
    }
}
