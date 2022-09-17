// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

import "../interfaces/ICivilizations.sol";
import "../interfaces/INames.sol";
import "../interfaces/IExperience.sol";

/**
 * @title Names
 * @notice This contract manages unique names for all characters in the [Civilizations](/docs/core/Civilizations.md) instance.
 * Some checks are based on the original Rarity names contract https://github.com/rarity-adventure/rarity-names/blob/main/contracts/rarity_names.sol
 * created by https://twitter.com/mat_nadler.
 *
 * @notice Implementation of the [INames](/docs/interfaces/INames.md) interface.
 */
contract Names is INames, Pausable, Ownable {
    // =============================================== Storage ========================================================
    /** @dev Address of the [Civilizations](/docs/core/Civilizations.md) implementation. **/
    address public civilizations;

    /** @dev Address of the [Experience](/docs/core/Experience.md) implementation. **/
    address public experience;

    /** @dev Map storing the names for each character. **/
    mapping(bytes => string) public names;

    /** @dev Names claimed. **/
    mapping(string => bool) public claimed_names;

    // =============================================== Modifiers ======================================================

    /**
     * @dev Checks if `msg.sender` is owner or allowed to manipulate a composed ID.
     */
    modifier onlyAllowed(bytes memory id) {
        require(
            ICivilizations(civilizations).exists(id),
            "Names: can't get access to a non minted token."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, id),
            "Names: msg.sender is not allowed to access this token."
        );
        _;
    }

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

    /**
     * @dev Assigns a name to a character and stores to prevent duplicates.
     *  @param id         Composed ID of the token.
     * @param name   Name to claim.
     */
    function claimName(bytes memory id, string memory name)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        require(
            IExperience(experience).getLevel(id) >= 5,
            "Name: claim name requires level 5."
        );
        require(
            bytes(names[id]).length == 0,
            "Name: token already have a name."
        );
        require(isNameValid(name), "Name: name trying to claim is not valid.");
        require(
            isNameAvailable(name),
            "Name: name trying to claim is already claimed."
        );
        claimed_names[toLowerCase(name)] = true;
        names[id] = name;
    }

    /**
     * @dev Changes a name for a character.
     *  @param id    Composed ID of the token.
     * @param newName   Name to replace with.
     */
    function replaceName(bytes memory id, string memory newName)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        require(
            IExperience(experience).getLevel(id) >= 5,
            "Name: replace a name requires level 5."
        );
        require(
            isNameValid(newName),
            "Name: name trying to replace with is not valid."
        );
        require(
            isNameAvailable(newName),
            "Name: name trying to replace with is already claimed."
        );
        string memory oldName = names[id];
        require(
            bytes(oldName).length != 0,
            "Name: can't replace name of token without a name."
        );
        claimed_names[toLowerCase(oldName)] = false;
        claimed_names[toLowerCase(newName)] = false;

        names[id] = newName;
    }

    /**
     * @dev Removes the name of the character.
     *  @param id    Composed ID of the token.
     */
    function clearName(bytes memory id) public whenNotPaused onlyAllowed(id) {
        string memory oldName = names[id];
        require(
            bytes(oldName).length != 0,
            "Name: can't clear name of token without a name."
        );
        claimed_names[toLowerCase(oldName)] = false;
        names[id] = "";
    }

    // =============================================== Getters ========================================================

    /**
     * @dev Returns the name of the composed ID.
     * @param id   Composed ID of the token.
     */
    function getTokenName(bytes memory id) public view returns (string memory) {
        return names[id];
    }

    /**
     * @dev Checks if a given name is available to use.
     * @param str   String to checked.
     */
    function isNameAvailable(string memory str) public view returns (bool) {
        return !claimed_names[toLowerCase(str)];
    }

    /**
     * @dev Checks if a given name is valid (Alphanumeric and spaces without leading or trailing space).
     * @param str   String to checked.
     */
    function isNameValid(string memory str) public pure returns (bool) {
        bytes memory b = bytes(str);
        if (b.length < 1) return false;
        if (b.length > 25) return false; // Cannot be longer than 25 characters
        if (b[0] == 0x20) return false; // Leading space
        if (b[b.length - 1] == 0x20) return false; // Trailing space

        bytes1 last_char = b[0];

        for (uint256 i; i < b.length; i++) {
            bytes1 char = b[i];

            if (char == 0x20 && last_char == 0x20) return false; // Cannot contain continous spaces

            if (
                !(char >= 0x30 && char <= 0x39) && //9-0
                !(char >= 0x41 && char <= 0x5A) && //A-Z
                !(char >= 0x61 && char <= 0x7A) && //a-z
                !(char == 0x20) //space
            ) return false;

            last_char = char;
        }

        return true;
    }

    /**
     * @dev Converts a string to lowercase.
     * @param str   String to convert.
     */
    function toLowerCase(string memory str)
        public
        pure
        returns (string memory)
    {
        bytes memory b_str = bytes(str);
        bytes memory b_lower = new bytes(b_str.length);
        for (uint256 i = 0; i < b_str.length; i++) {
            if ((uint8(b_str[i]) >= 65) && (uint8(b_str[i]) <= 90)) {
                b_lower[i] = bytes1(uint8(b_str[i]) + 32);
            } else {
                b_lower[i] = b_str[i];
            }
        }
        return string(b_lower);
    }
}
