// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "../interfaces/ICivilizations.sol";

contract MockMinter {
    address civilizations;

    constructor(address _civilizations) {
        civilizations = _civilizations;
    }

    function mintMock(uint256 _instance) public {
        ICivilizations(civilizations).mint(_instance);
    }
}
