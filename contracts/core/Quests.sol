// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

import "../interfaces/IQuests.sol";

/**
 * @title Quests
 * @notice This contracts stores multiple quests and enables all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance
 * to obtain rewards and experience from them.
 *
 * @notice Implementation of the [IQuests](/docs/interfaces/IQuests.md) interface.
 */
contract Quests is IQuests, Ownable, Pausable {
    // =============================================== Structs ========================================================
    // =============================================== Storage ========================================================

    /** @dev Address of the [Civilizations](/docs/core/Civilizations.md) instance. **/
    address public civilizations;

    /** @dev Address of the [Experience](/docs/core/Experience.md) instance. **/
    address public experience;

    // =============================================== Modifiers ======================================================

    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     */
    constructor(address _civilizations, address _experience) {
        civilizations = _civilizations;
        experience = _experience;
    }

    /** @notice Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @notice Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
    }

    // =============================================== Getters ========================================================
    // =============================================== Internal =======================================================
}
