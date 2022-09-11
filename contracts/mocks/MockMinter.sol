// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "../interfaces/ICivilizations.sol";

contract MockMinter {
    address civilizations;

    constructor(address _civilizations) {
        civilizations = _civilizations;
    }

    function mintMock(address _instance) public payable {
        ICivilizations(civilizations).mint{value: msg.value}(_instance);
    }
}
