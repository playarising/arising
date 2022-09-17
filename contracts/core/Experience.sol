// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";

import "../interfaces/ILevels.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/ICivilizations.sol";

/**
 * @title Experience
 * @notice This contract tracks and assigns experience of all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance.
 *
 * @notice Implementation of the [IExperience](/docs/interfaces/IExperience.md) interface.
 */
contract Experience is IExperience, Ownable {
    // =============================================== Storage ========================================================

    /** @notice Address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;

    /** @notice Address of the [Levels](/docs/codex/Levels.md) instance. */
    address public levels;

    /** @notice Map to store the list of authorized addresses to assign experience. */
    mapping(address => bool) authorized;

    /** @notice Map to track the experience of composed IDs. */
    mapping(bytes => uint256) public experience;

    // ============================================== Modifiers =======================================================

    /** @notice Checks against if the `msg.sender` is authorized to assign experience. */
    modifier onlyAuthorized() {
        require(
            authorized[msg.sender],
            "Experience: onlyAuthorized() msg.sender not authorized."
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _levels           The address of the [Levels](/docs/codex/Levels.md) instance.
     */
    constructor(address _levels, address _civilizations) {
        levels = _levels;
        civilizations = _civilizations;
        authorized[msg.sender] = true;
    }

    /**
     * @notice Replaces the address of the [Levels](/docs/codex/Levels.md) instance to determine character levels.
     *
     * Requirements:
     * @param _levels    Address of the [Levels](/docs/codex/Levels.md) instance.
     */
    function setLevel(address _levels) public onlyOwner {
        levels = _levels;
    }

    /**
     * @notice Assigns new experience to the composed ID provided.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _amount   The amount of experience to add.
     */
    function assignExperience(bytes memory _id, uint256 _amount)
        public
        onlyAuthorized
    {
        require(
            ICivilizations(civilizations).exists(_id),
            "Experience: assignExperience() token not minted."
        );
        experience[_id] += _amount;
    }

    /**
     * @notice Assigns a new address as an authority to assign experience.
     *
     * Requirements:
     * @param _authority    Address to give authority.
     */
    function addAuthority(address _authority) public onlyOwner {
        authorized[_authority] = true;
    }

    /**
     * @notice Removes an authority to assign experience.
     *
     * Requirements:
     * @param _authority    Address to give authority.
     */
    function removeAuthority(address _authority) public onlyOwner {
        require(
            authorized[_authority],
            "Experience: removeAuthority() address is not authorized."
        );
        authorized[_authority] = false;
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to return the total experience of a composed ID.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _experience  Total experience of the character.
     */
    function getExperience(bytes memory _id)
        public
        view
        returns (uint256 _experience)
    {
        return experience[_id];
    }

    /**
     * @notice External function to return the level of a composed ID.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     *
     * @return _level   Level number of the character.
     */
    function getLevel(bytes memory _id) public view returns (uint256 _level) {
        return ILevels(levels).getLevel(experience[_id]);
    }

    /**
     * @notice External function to return the total experience required to reach the next level a composed ID.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _experience  Total experience required to reach the next level.
     */
    function getExperienceForNextLevel(bytes memory _id)
        public
        view
        returns (uint256 _experience)
    {
        return ILevels(levels).getExperience(getLevel(_id)) - experience[_id];
    }
}
