// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";

import "../interfaces/ILevels.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/ICivilizations.sol";

/**
 * @dev `Experience` is the contract to manage the storage of experience and missions from all the civilizations.
 */

contract Experience is Ownable, IExperience {
    // =============================================== Storage ========================================================

    /** @dev Map to store the experience from composed ID. **/
    mapping(bytes => uint256) experience;

    /** @dev Address of the `Levels` implementation. **/
    address public levels;

    /** @dev Address of the `Civilizations` implementation. **/
    address public civilizations;

    /** @dev Map to store the list of authorized addresses to assign experience. **/
    mapping(address => bool) authorized;

    // ============================================== Modifiers =======================================================

    /**
     * @dev Checks if `msg.sender` is authorized to assign experience.
     */
    modifier onlyAuthorized() {
        require(
            authorized[msg.sender],
            "Experience: msg.sender is not authorized to assign experience"
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _levels           The address of the `Levels` instance.
     * @param _civilizations    The address of the `Civilizations` instance.
     */
    constructor(address _levels, address _civilizations) {
        levels = _levels;
        civilizations = _civilizations;
        authorized[msg.sender] = true;
    }

    /** @dev Adds experience to the character from a composed ID.
     *  @param id   Composed ID of the token.
     */
    function assignExperience(bytes memory id, uint256 amount)
        public
        onlyAuthorized
    {
        require(
            ICivilizations(civilizations).exists(id),
            "Experience: can't assign experience to non minted token."
        );
        experience[id] += amount;
    }

    /** @dev Adds an authority to assign experience.
     *  @param authority   Address of the authority to assign.
     */
    function addAuthority(address authority) public onlyOwner {
        authorized[authority] = true;
    }

    /** @dev Removes an authority to assign experience.
     *  @param authority   Address of the authority to remove.
     */
    function removeAuthority(address authority) public onlyOwner {
        authorized[authority] = false;
    }

    // =============================================== Getters ========================================================

    /** @dev Returns the experience points of the token from a composed ID.
     *  @param id   Composed ID of the token.
     */
    function getExperience(bytes memory id) public view returns (uint256) {
        return experience[id];
    }

    /** @dev Returns the level of a token from a composed ID.
     *  @param id   Composed ID of the token.
     */
    function getLevel(bytes memory id) public view returns (uint256) {
        return ILevels(levels).getLevel(experience[id]);
    }

    /** @dev Returns the amount of experience required to reach the next level.
     *  @param id   Composed ID of the token.
     */
    function getExperienceForNextLevel(bytes memory id)
        public
        view
        returns (uint256)
    {
        return ILevels(levels).getExperience(getLevel(id)) - experience[id];
    }
}
