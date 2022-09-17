// SPDX-License-Identifier: MIT
pragma solidity 0.8.17;

/**
 * @title IQuests
 * @notice Interface for the [Quests](/docs/core/Quests.md) contract.
 */
interface IQuests {
    enum QuestType {
        JOB,
        FARM,
        RAID
    }

    struct Quest {
        uint256 id;
        QuestType quest_type;
        uint256 gold_reward;
        address[] resources_reward;
        uint256[] resources_amounts;
        uint256 experience_reward;
        uint256 cooldown;
        uint256 level_required;
        bool available;
    }

    struct CurrentQuest {
        uint256 last_quest_id;
        bool claimed_reward;
        uint256 cooldown;
    }
}
