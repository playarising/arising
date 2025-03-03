// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

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
contract Names is
    INames,
    Initializable,
    PausableUpgradeable,
    OwnableUpgradeable,
    UUPSUpgradeable
{
    // =============================================== Storage ========================================================
    /** @notice Address of the [Civilizations](/docs/core/Civilizations.md) instance. */
    address public civilizations;

    /** @notice Address of the [Experience](/docs/core/Experience.md) instance. */
    address public experience;

    /** @notice Map to track the names of the characters. */
    mapping(bytes => string) public names;

    /** @notice Map to track the names availability. */
    mapping(string => bool) public claimed_names;

    // =============================================== Modifiers ======================================================

    /**
     * @notice Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
     * has allowance to access a composed ID.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    modifier onlyAllowed(bytes memory _id) {
        require(
            ICivilizations(civilizations).exists(_id),
            "Names: onlyAllowed() token not minted."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "Names: onlyAllowed() msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Events =========================================================

    /**
     * @notice Event emmited when the character name is changed.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _name     New name of the character.
     */
    event ChangeName(bytes _id, string _name);

    // =============================================== Setters ========================================================

    /**
     * @notice Initialize.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     */
    function initialize(
        address _civilizations,
        address _experience
    ) public initializer {
        __Ownable_init();
        __Pausable_init();
        __UUPSUpgradeable_init();

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
     * @notice Assigns a name to a character and marks it as claimed.
     *
     * Requirements:
     * @param _id       Composed ID of the character.
     * @param _name     Name to assign and claim.
     */
    function claimName(
        bytes memory _id,
        string memory _name
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            IExperience(experience).getLevel(_id) >= 5,
            "Name: claimName() not enough level."
        );
        require(
            bytes(names[_id]).length == 0,
            "Name: claimName() already named."
        );
        require(isNameValid(_name), "Name: claimName() invalid name.");
        require(
            isNameAvailable(_name),
            "Name: claimName() name not available."
        );
        claimed_names[toLowerCase(_name)] = true;
        names[_id] = _name;
        emit ChangeName(_id, _name);
    }

    /**
     * @notice Replaces the name of a character with a name already assigned.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     * @param _new_name     Name to replace for the character.
     */
    function replaceName(
        bytes memory _id,
        string memory _new_name
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            IExperience(experience).getLevel(_id) >= 5,
            "Name: replaceName() not enough level."
        );
        require(isNameValid(_new_name), "Name: replaceName() invalid name.");
        require(
            isNameAvailable(_new_name),
            "Name: replaceName() name not available."
        );
        string memory old_name = names[_id];
        require(
            bytes(old_name).length != 0,
            "Name: replaceName() no name assigned."
        );
        claimed_names[toLowerCase(old_name)] = false;
        claimed_names[toLowerCase(_new_name)] = false;

        names[_id] = _new_name;
        emit ChangeName(_id, _new_name);
    }

    /**
     * @notice Removes the assigned name to the character.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    function clearName(bytes memory _id) public whenNotPaused onlyAllowed(_id) {
        string memory old_name = names[_id];
        require(
            bytes(old_name).length != 0,
            "Name: clearName() no name assigned."
        );
        claimed_names[toLowerCase(old_name)] = false;
        names[_id] = "";
        emit ChangeName(_id, "");
    }

    // =============================================== Getters ========================================================

    /**
     * @notice External function to get the assigned name of a character.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     *
     * @return _name    The assigned name of the character.
     */
    function getCharacterName(
        bytes memory _id
    ) public view returns (string memory _name) {
        return names[_id];
    }

    /**
     * @notice External function to check if a name is available to assign.
     *
     * Requirements:
     * @param _name         The name to check.
     *
     * @return _available   Boolean to know if the name is available.
     */
    function isNameAvailable(
        string memory _name
    ) public view returns (bool _available) {
        return !claimed_names[toLowerCase(_name)];
    }

    /**
     * @notice External function to check if a name is valid to assign.
     *
     * Requirements:
     * @param _name         The name to check.
     *
     * @return _available   Boolean to know if the name is valid.
     */
    function isNameValid(
        string memory _name
    ) public pure returns (bool _available) {
        bytes memory b = bytes(_name);
        if (b.length < 1) return false;
        if (b.length > 25) return false;
        if (b[0] == 0x20) return false;
        if (b[b.length - 1] == 0x20) return false;

        bytes1 last_char = b[0];

        for (uint256 i; i < b.length; i++) {
            bytes1 char = b[i];

            if (char == 0x20 && last_char == 0x20) return false;

            if (
                !(char >= 0x30 && char <= 0x39) &&
                !(char >= 0x41 && char <= 0x5A) &&
                !(char >= 0x61 && char <= 0x7A) &&
                !(char == 0x20)
            ) return false;

            last_char = char;
        }

        return true;
    }

    /**
     * @notice External function to convert a name to lower case.
     *
     * Requirements:
     * @param _name         The name to convert.
     *
     * @return _lower_case   The provided name as a lower case string.
     */
    function toLowerCase(
        string memory _name
    ) public pure returns (string memory _lower_case) {
        bytes memory b_str = bytes(_name);
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

    // =============================================== Internal =======================================================

    /** @notice Internal function make sure upgrade proxy caller is the owner. */
    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyOwner {}
}
