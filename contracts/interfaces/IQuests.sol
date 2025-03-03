// SPDX-License-Identifier: MIT
pragma solidity 0.8.25;

import "../interfaces/IStats.sol";

/**
 * @title IQuests
 * @notice Interface for the [Quests](/docs/core/Quests.md) contract.
 */
interface IQuests {
    /**
     * @notice Internal enum to determine a quest type.
     *
    
     */
    enum QuestType {
        JOB,
        FARM,
        RAID
    }

    /**
     * @notice Internal struct for the quests information.
     *
     * Requirements:
     * @param id                    ID of the quest.
     * @param name                  Name of the quest.
     * @param description           Description of the quest.
     * @param quest_type            Type of the added quest.
     * @param resources_reward      Array of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances to reward for the quest.
     * @param resources_amounts     Array of amounts for each resource reward.
     * @param experience_reward     Amount of experience rewarded for the quest.
     * @param stats_cost            The total amount of stats available to consume for the quest.
     * @param cooldown              Number of seconds for the quest cooldown.
     * @param level_required        Minimum level required to start the quest.
     * @param available             Boolean to check if the quest is available.
     */
    struct Quest {
        uint256 id;
        string name;
        string description;
        QuestType quest_type;
        address[] resources_reward;
        uint256[] resources_amounts;
        uint256 experience_reward;
        IStats.BasicStats stats_cost;
        uint256 cooldown;
        uint256 level_required;
        bool available;
    }

    /**
     * @notice Internal struct to store the current character quest information.
     *
     * Requirements:
     * @param last_quest_id     ID of the last quest.
     * @param claimed_reward    Boolean to know if the quest reward is already claimed.
     * @param cooldown          Timestamp until the quest reward can be claimed.
     * @param fullfilment       Percentage of fulfillment of the quest based on the stats assigned for reward calculations.
     */
    struct CurrentQuest {
        uint256 last_quest_id;
        bool claimed_reward;
        uint256 cooldown;
        uint256 fullfilment;
    }

    /** @notice See [Quests#pause](/docs/core/Quests.md#pause) */
    function pause() external;

    /** @notice See [Quests#unpause](/docs/core/Quests.md#unpause) */
    function unpause() external;

    /** @notice See [Quests#disableQuest](/docs/core/Quests.md#disableQuest) */
    function disableQuest(uint256 _quest_id) external;

    /** @notice See [Quests#enableQuest](/docs/core/Quests.md#enableQuest) */
    function enableQuest(uint256 _quest_id) external;

    /** @notice See [Quests#addQuest](/docs/core/Quests.md#addQuest) */
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
    ) external;

    /** @notice See [Quests#updateQuest](/docs/core/Quests.md#updateQuest) */
    function updateQuest(Quest memory _quest) external;

    /** @notice See [Quests#startQuest](/docs/core/Quests.md#startQuest) */
    function startQuest(
        bytes memory _id,
        uint256 _quest_id,
        IStats.BasicStats memory _stats_consumed
    ) external;

    /** @notice See [Quests#claimQuest](/docs/core/Quests.md#claimQuest) */
    function claimQuest(bytes memory _id) external;

    /** @notice See [Quests#getQuest](/docs/core/Quests.md#getQuest) */
    function getQuest(
        uint256 _quest_id
    ) external view returns (Quest memory _quest);

    /** @notice See [Quests#getCharacterCurrentQuest](/docs/core/Quests.md#getCharacterCurrentQuest) */
    function getCharacterCurrentQuest(
        bytes memory _id
    ) external view returns (CurrentQuest memory _quest);
}
