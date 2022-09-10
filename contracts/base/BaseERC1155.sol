// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev `BaseERC1155` is the base ERC1155 token for Arising Civilizations.
 */
contract BaseERC1155 is ERC1155 {
    // =============================================== Storage ========================================================
    // =============================================== Setters ========================================================
    // =============================================== Modifiers ======================================================

    /**
     * @dev Constructor.
     */
    constructor(string memory _uri) ERC1155(_uri) {}

    // =============================================== Getters ========================================================
    // =============================================== Internal =======================================================
}
