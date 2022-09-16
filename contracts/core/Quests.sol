// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

/**
 * @dev `Quests` is a contract to manage the different missions characters can do.
 */

contract Quests is Ownable, Pausable {
    // =============================================== Structs ========================================================
    // =============================================== Storage ========================================================

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Address of the `Experience` instance. **/
    address public experience;

    // =============================================== Modifiers ======================================================

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _civilizations    The address of the `Civilizations` instance.
     * @param _experience       The address of the `Experience` instance.
     */
    constructor(address _civilizations, address _experience) {
        civilizations = _civilizations;
        experience = _experience;
    }

    /** @dev Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @dev Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
    }

    // =============================================== Getters ========================================================
    // =============================================== Internal =======================================================
}
