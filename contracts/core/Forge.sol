// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev `Forge` is a contract to convert the raw material to craftable pieces.
 */

contract Forge is Ownable {
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

    // =============================================== Getters ========================================================
    // =============================================== Internal =======================================================
}
