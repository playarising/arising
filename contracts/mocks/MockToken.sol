// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";

contract MockToken is ERC20 {
    constructor(uint256 supply) ERC20("MockToken", "MOCK") {
        _mint(msg.sender, supply);
    }
}
