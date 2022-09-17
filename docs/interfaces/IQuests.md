# Solidity API

## IQuests

Interface for the [Quests](/docs/core/Quests.md) contract.

### QuestType

```solidity
enum QuestType {
  JOB,
  FARM,
  RAID
}

```

### Quest

```solidity
struct Quest {
  uint256 id;
  enum IQuests.QuestType quest_type;
  uint256 gold_reward;
  address[] resources_reward;
  uint256[] resources_amounts;
  uint256 experience_reward;
  uint256 cooldown;
  uint256 level_required;
  bool available;
}
```

### CurrentQuest

```solidity
struct CurrentQuest {
  uint256 last_quest_id;
  bool claimed_reward;
  uint256 cooldown;
}

```
