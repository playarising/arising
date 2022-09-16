// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @dev `Craft` is the contract to manage item crafting for Arising.
 */

// TODO
contract Craft is Ownable, Pausable {
    // =============================================== Structs ========================================================
    // =============================================== Storage ========================================================
    // =============================================== Modifiers ======================================================

    /** @dev Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @dev Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
    }

    // =============================================== Setters ========================================================
    // =============================================== Getters ========================================================
    // =============================================== Internal =======================================================
}
