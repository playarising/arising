// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/Pausable.sol";

import "../interfaces/IQuests.sol";
import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";

/**
 * @title Quests
 * @notice This contracts stores multiple quests and enables all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance
 * to obtain rewards and experience from them.
 *
 * @notice Implementation of the [IQuests](/docs/interfaces/IQuests.md) interface.
 */
contract Quests is IQuests, Ownable, Pausable {
    // =============================================== Storage ========================================================

    /** @dev Address of the [Civilizations](/docs/core/Civilizations.md) instance. **/
    address public civilizations;

    /** @dev Address of the [Experience](/docs/core/Experience.md) instance. **/
    address public experience;

    /** @dev Address of the [Stats](/docs/core/Stats.md) instance. **/
    address public stats;

    /** @dev The address of the [Gold](/docs/gadgets/Gold.md) instance. **/
    address public gold;

    /** @notice Map to track all the quests. */
    mapping(uint256 => Quest) quests;

    /** @notice Array to track a full list of quests IDs. */
    uint256[] _quests;

    /** @dev Map to track quests cooldowns for each character. **/
    mapping(bytes => CurrentQuest) public character_quests;

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
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "Quests: onlyAllowed() msg.sender is not allowed to access this token."
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
     * @param _stats            The address of the [Stats](/docs/core/Stats.md) instance.
     * @param _gold             The address of the [Gold](/docs/gadgets/Gold.md) instance.
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

    /** @notice Pauses the contract */
    function pause() public onlyOwner {
        _pause();
    }

    /** @notice Resumes the contract */
    function unpause() public onlyOwner {
        _unpause();
    }

    /**
     * @notice Disables a quest.
     *
     * Requirements:
     * @param _id   ID of the quest.
     */
    function disableItem(uint256 _id) public onlyOwner {
        require(
            _id != 0 && _id <= _quests.length,
            "Quests: disableItem() invalid quest id."
        );
        quests[_id].available = false;
    }

    /**
     * @notice Enables a quest.
     *
     * Requirements:
     * @param _id   ID of the quest.
     */
    function enableItem(uint256 _id) public onlyOwner {
        require(
            _id != 0 && _id <= _quests.length,
            "Quests: enableItem() invalid quest id."
        );
        quests[_id].available = true;
    }

    /**
     * @dev Claims a recipe already crafted.
     * @param id    Composed ID of the character.
     */
    function startQuest(bytes memory id, uint256 quest)
        public
        whenNotPaused
        onlyAllowed(id)
    {
        require(
            _isAvailableForQuest(id),
            "Quest: startQuest() not available for quest."
        );
    }

    /**
     * @dev Claims a recipe already crafted.
     * @param id    Composed ID of the character.
     */
    function claimQuest(bytes memory id) public whenNotPaused onlyAllowed(id) {
        require(
            _isQuestClaimable(id),
            "Quest: claimQuest() not available to claim."
        );
    }

    // =============================================== Getters ========================================================

    /**
     * @notice Returns the full information of a quest.
     *
     * Requirements:
     * @param _id       ID of the quest.
     *
     * @return _quest    Full quest information.
     */
    function getItem(uint256 _id) public view returns (Quest memory _quest) {
        return quests[_id];
    }

    // =============================================== Internal =======================================================

    /**
     * @dev Internal function to check if the craft slot is available to craft.
     * @param id    Composed ID of the character.
     */
    function _isAvailableForQuest(bytes memory id)
        internal
        view
        returns (bool)
    {
        CurrentQuest memory q = character_quests[id];

        if (q.cooldown == 0) {
            return true;
        }

        return q.cooldown <= block.timestamp && q.claimed_reward;
    }

    /**
     * @dev Internal function to check if a craft slot is ready to claim.
     * @param id    Composed ID of the character.
     */
    function _isQuestClaimable(bytes memory id) internal view returns (bool) {
        CurrentQuest memory q = character_quests[id];
        return
            q.cooldown <= block.timestamp &&
            !q.claimed_reward &&
            q.last_quest_id != 0;
    }
}
