// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";
import "../interfaces/ICraft.sol";
import "../interfaces/ICivilizations.sol";

/**
 * @dev `Craft` is the contract to manage item crafting for Arising.
 */

// TODO
contract Craft is ICraft, Ownable, Pausable {
    // =============================================== Storage ========================================================

    /** @dev Map to track available recipes on the forge. **/
    mapping(uint256 => Recipe) public recipes;

    /** @dev Array to track all the recipes ids. **/
    uint256[] private _recipes;

    /** @dev The address of the `Gold` instance. **/
    address public gold;

    /** @dev Address of the `Civilizations` instance. **/
    address public civilizations;

    /** @dev Address of the `Experience` instance. **/
    address public experience;

    /** @dev Address of the `Stats` instance. **/
    address public stats;

    // =============================================== Modifiers ======================================================

    /**
     * @dev Checks if `msg.sender` is owner or allowed to manipulate a composed ID.
     */
    modifier onlyAllowed(bytes memory id) {
        require(
            ICivilizations(civilizations).exists(id),
            "Craft: can't get access to a non minted token."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, id),
            "Craft: msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Setters ========================================================

    /**
     * @dev Constructor.
     * @param _civilizations        The address of the `Civilizations` instance.
     * @param _experience           The address of the `Experience` instance.
     * @param _stats                The address of the `Experience` instance.
     * @param _gold                 The address of the `Gold` instance.
     */
    constructor(
        address _civilizations,
        address _experience,
        address _stats,
        address _gold
    ) {
        civilizations = _civilizations;
        experience = _experience;
        stats = _stats;
        gold = _gold;
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
