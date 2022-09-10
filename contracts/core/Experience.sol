// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev `Experience` is the contract to manage the storage of experience and missions from all the civilizations.
 */

contract Experience is Ownable {
    // =============================================== Structs ========================================================
    // =============================================== Storage ========================================================

    /** @dev Map to store the experience from composed ID. **/
    mapping(bytes => uint256) experience;

    // =============================================== Events =========================================================
    // ============================================== Modifiers =======================================================
    // =============================================== Setters ========================================================
    // =============================================== Getters ========================================================

    /** @dev Returns the experience points of the token from the civilization sent.
     *  @param id   Composed ID of the token.
     */
    function getExperience(bytes memory id) public view returns (uint256) {
        return experience[id];
    }
}
