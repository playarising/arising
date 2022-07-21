// SPDX-License-Identifier: MIT
pragma solidity 0.8.9;

import "../interfaces/IMintGuard.sol";

contract MockMinter {
    // MintGuard
    address guard;

    constructor(address _guard) {
        guard = _guard;
    }

    /**
     * @dev This function will imitate writing to the MintGuard contract
     */
    function mintMock() public {
        IMintGuard(guard).setMinter(msg.sender);
    }
}
