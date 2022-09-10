// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @dev `ExperienceManager` is the contract to manage the storage of experience and missions from all the civilizations.
 */

contract ExperienceManager is Ownable {
    // =============================================== Structs ========================================================
    // =============================================== Storage ========================================================

    /** @dev Boolean to check if the implementation is usable. **/
    bool public initialized;

    /** @dev Map to store the civilizations implemented. **/
    mapping(uint256 => address) _civilizations;

    /** @dev Array to store the civilizations implemented. **/
    uint256[] civilizations;

    /** @dev Map to store the maps for each civilization and token id experience points. **/
    mapping(uint256 => mapping(uint256 => uint256)) experience;

    // =============================================== Events =========================================================
    // ============================================== Modifiers =======================================================

    /**
     * @dev Checks if `initialized` is enabled.
     */
    modifier onlyInitialized() {
        require(initialized, "ExperienceManager: contract is not initialized");
        _;
    }

    // =============================================== Setters ========================================================

    /** @dev Enables the `ExperienceManager` implementation. */
    function setInitialized() public onlyOwner {
        initialized = true;
    }

    /** @dev Adds a civilization to the experience manager.
     *  @param _instance  Address of the `BaseERC721` instance.
     */
    function addCivilization(address _instance)
        public
        onlyOwner
        onlyInitialized
    {
        require(
            _instance != address(0),
            "ExperienceManager: instance address is null"
        );
        uint256 newId = civilizations.length + 1;
        _civilizations[newId] = _instance;
        civilizations.push(newId);
    }

    // =============================================== Getters ========================================================

    /** @dev Returns the experience points of the token from the civilization sent.
     *  @param id               ID of the token.
     *  @param civilization     ID of the civilization.
     */
    function getExperience(uint256 id, uint256 civilization)
        public
        view
        returns (uint256)
    {
        require(
            _civilizations[civilization] != address(0),
            "ExperienceManager: civilization doesn't exist"
        );
        return experience[civilization][id];
    }
}
