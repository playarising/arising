// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";

import "../base/BaseFungibleItem.sol";

import "../interfaces/IQuests.sol";
import "../interfaces/ICivilizations.sol";
import "../interfaces/IExperience.sol";
import "../interfaces/IStats.sol";
import "../interfaces/IBaseFungibleItem.sol";

/**
 * @title Quests
 * @notice This contracts stores multiple quests and enables all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance
 * to obtain rewards and experience from them.
 *
 * @notice Implementation of the [IQuests](/docs/interfaces/IQuests.md) interface.
 */
contract Quests is
    IQuests,
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

    /** @notice Address of the [Stats](/docs/core/Stats.md) instance. */
    address public stats;

    /** @notice Map to track all the available quests. */
    mapping(uint256 => Quest) quests;

    /** @notice Array to track a full list of quests IDs. */
    uint256[] _quests;

    /** @notice Map to track current quests for all characters. **/
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
            ICivilizations(civilizations).exists(_id),
            "Quests: onlyAllowed() token not minted."
        );
        require(
            ICivilizations(civilizations).isAllowed(msg.sender, _id),
            "Quests: onlyAllowed() msg.sender is not allowed to access this token."
        );
        _;
    }

    // =============================================== Events =========================================================

    /**
     * @notice Event emmited when the [addQuest](#addQuest) function is called.
     *
     * Requirements:
     * @param _quest_id     ID of the quest added.
     * @param _name         Name of the quest.
     * @param _description  Quest description
     */
    event AddQuest(uint256 _quest_id, string _name, string _description);

    /**
     * @notice Event emmited when the [updateQuest](#updateQuest) function is called.
     *
     * Requirements:
     * @param _quest_id     ID of the quest added.
     * @param _name         Name of the quest.
     * @param _description  Quest description
     */
    event QuestUpdate(uint256 _quest_id, string _name, string _description);

    /**
     * @notice Event emmited when the [enableQuest](#enableQuest) function is called.
     *
     * Requirements:
     * @param _quest_id    ID of the quest enabled.
     */
    event EnableQuest(uint256 _quest_id);

    /**
     * @notice Event emmited when the [disableQuest](#disableQuest) function is called.
     *
     * Requirements:
     * @param _quest_id    ID of the recipe disabled.
     */
    event DisableQuest(uint256 _quest_id);

    // =============================================== Setters ========================================================

    /**
     * @notice Constructor.
     *
     * Requirements:
     * @param _civilizations    The address of the [Civilizations](/docs/core/Civilizations.md) instance.
     * @param _experience       The address of the [Experience](/docs/core/Experience.md) instance.
     * @param _stats            The address of the [Stats](/docs/core/Stats.md) instance.
     */
    function initialize(
        address _civilizations,
        address _experience,
        address _stats
    ) public initializer {
        __Ownable_init();
        __Pausable_init();
        __UUPSUpgradeable_init();

        civilizations = _civilizations;
        experience = _experience;
        stats = _stats;
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
     * @notice Disables a quest for characters.
     *
     * Requirements:
     * @param _quest_id   ID of the quest.
     */
    function disableQuest(uint256 _quest_id) public onlyOwner {
        require(
            _quest_id != 0 && _quest_id <= _quests.length,
            "Quests: disableQuest() invalid quest id."
        );
        quests[_quest_id].available = false;
        emit DisableQuest(_quest_id);
    }

    /**
     * @notice Enables a quest for characters.
     *
     * Requirements:
     * @param _quest_id   ID of the quest.
     */
    function enableQuest(uint256 _quest_id) public onlyOwner {
        require(
            _quest_id != 0 && _quest_id <= _quests.length,
            "Quests: enableQuest() invalid quest id."
        );
        quests[_quest_id].available = true;
        emit EnableQuest(_quest_id);
    }

    /**
     * @notice Adds a new quest for characters.
     *
     * Requirements:
     * @param _name                 Name of the quest.
     * @param _description          Description of the quest.
     * @param _quest_type           Type of the added quest.
     * @param _resources_reward     Array of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances to reward for the quest.
     * @param _resources_amounts    Array of amounts for each resource reward.
     * @param _experience_reward    Amount of experience rewarded for the quest.
     * @param _stats                Stats to consume from the pool for the quest.
     * @param _cooldown             Number of seconds for the quest cooldown.
     * @param _level_required       Minimum level required to start the quest.
     */
    function addQuest(
        string memory _name,
        string memory _description,
        QuestType _quest_type,
        address[] memory _resources_reward,
        uint256[] memory _resources_amounts,
        uint256 _experience_reward,
        IStats.BasicStats memory _stats,
        uint256 _cooldown,
        uint256 _level_required
    ) public onlyOwner {
        uint256 _quest_id = _quests.length + 1;
        require(
            _resources_reward.length == _resources_amounts.length,
            "Quest: addQuest() materials and amounts not match."
        );
        quests[_quest_id] = Quest(
            _quest_id,
            _name,
            _description,
            _quest_type,
            _resources_reward,
            _resources_amounts,
            _experience_reward,
            _stats,
            _cooldown,
            _level_required,
            true
        );
        _quests.push(_quest_id);
        emit AddQuest(_quest_id, _name, _description);
    }

    /**
     * @notice Updates a previously added quest.
     *
     * Requirements:
     * @param _quest   Full information of the quest.
     */
    function updateQuest(Quest memory _quest) public onlyOwner {
        require(
            _quest.id != 0 && _quest.id <= _quests.length,
            "Quests: updateQuest() invalid quest id."
        );
        quests[_quest.id] = _quest;
        emit QuestUpdate(_quest.id, _quest.name, _quest.description);
    }

    /**
     * @notice Starts a quest for the character provided.
     *
     * Requirements:
     * @param _id               Composed ID of the character.
     * @param _quest_id         ID of the quest.
     * @param _stats_consumed   Amount of stats to consume for the quest.
     */
    function startQuest(
        bytes memory _id,
        uint256 _quest_id,
        IStats.BasicStats memory _stats_consumed
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            _quest_id != 0 && _quest_id <= _quests.length,
            "Quests: startQuest() invalid quest id."
        );
        require(
            _isAvailableForQuest(_id),
            "Quest: startQuest() not available for quest."
        );
        Quest memory _quest = quests[_quest_id];
        require(
            _quest.available,
            "Quests: startQuest() quest is not available."
        );
        require(
            IExperience(experience).getLevel(_id) >= _quest.level_required,
            "Quests: startQuest() not enough level."
        );
        IStats(stats).consume(_id, _stats_consumed);
        uint256 _total_stats_consumed = _stats_consumed.might +
            _stats_consumed.speed +
            _stats_consumed.intellect;
        uint256 _max_quest_stats = _quest.stats_cost.might +
            _quest.stats_cost.speed +
            _quest.stats_cost.intellect;
        uint256 _fullfilment = _total_stats_consumed >= _max_quest_stats
            ? 100
            : (_total_stats_consumed * 100) / _max_quest_stats;
        character_quests[_id] = CurrentQuest(
            _quest_id,
            false,
            block.timestamp + _quest.cooldown,
            _fullfilment
        );
    }

    /**
     * @notice Claims a finished quest for the character.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     */
    function claimQuest(
        bytes memory _id
    ) public whenNotPaused onlyAllowed(_id) {
        require(
            _isQuestClaimable(_id),
            "Quest: claimQuest() not available to claim."
        );

        character_quests[_id].claimed_reward = true;

        Quest memory _quest = quests[character_quests[_id].last_quest_id];

        uint256 _experience = (_quest.experience_reward *
            character_quests[_id].fullfilment) / 100;

        IExperience(experience).assignExperience(_id, _experience);

        for (uint256 i = 0; i < _quest.resources_reward.length; i++) {
            uint256 _amount = (_quest.resources_amounts[i] *
                character_quests[_id].fullfilment) / 100;

            IBaseFungibleItem(_quest.resources_reward[i]).mintTo(_id, _amount);
        }
    }

    // =============================================== Getters ========================================================

    /**
     * @notice Returns the full information of a quest.
     *
     * Requirements:
     * @param _quest_id       ID of the quest.
     *
     * @return _quest    Full quest information.
     */
    function getQuest(
        uint256 _quest_id
    ) public view returns (Quest memory _quest) {
        return quests[_quest_id];
    }

    /**
     * @notice Returns the character current quest information.
     *
     * Requirements:
     * @param _id   Composed ID of the character.
     *
     * @return _quest    Current character quest information.
     */
    function getCharacterCurrentQuest(
        bytes memory _id
    ) public view returns (CurrentQuest memory _quest) {
        return character_quests[_id];
    }

    // =============================================== Internal =======================================================

    /**
     * @notice Internal function to check if the character is able to start a quest.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _available   Boolean to know if the character is available to start a quest.
     */
    function _isAvailableForQuest(
        bytes memory _id
    ) internal view returns (bool _available) {
        if (character_quests[_id].cooldown == 0) {
            return true;
        }

        return
            character_quests[_id].cooldown <= block.timestamp &&
            character_quests[_id].claimed_reward;
    }

    /**
     * @notice Internal function to check if the last character quest is claimable.
     *
     * Requirements:
     * @param _id           Composed ID of the character.
     *
     * @return _claimable   Boolean to know if the last quest is claimable.
     */
    function _isQuestClaimable(
        bytes memory _id
    ) internal view returns (bool _claimable) {
        return
            character_quests[_id].cooldown <= block.timestamp &&
            !character_quests[_id].claimed_reward &&
            character_quests[_id].last_quest_id != 0;
    }

    /** @notice Internal function make sure upgrade proxy caller is the owner. */
    function _authorizeUpgrade(
        address newImplementation
    ) internal virtual override onlyOwner {}
}
