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
  string name;
  string description;
  enum IQuests.QuestType quest_type;
  address[] resources_reward;
  uint256[] resources_amounts;
  uint256 experience_reward;
  struct IStats.BasicStats stats_cost;
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
  uint256 fullfilment;
}

```

### pause

```solidity
function pause() external
```

See [Quests#pause](/docs/core/Quests.md#pause)

### unpause

```solidity
function unpause() external
```

See [Quests#unpause](/docs/core/Quests.md#unpause)

### disableQuest

```solidity
function disableQuest(uint256 _quest_id) external
```

See [Quests#disableQuest](/docs/core/Quests.md#disableQuest)

### enableQuest

```solidity
function enableQuest(uint256 _quest_id) external
```

See [Quests#enableQuest](/docs/core/Quests.md#enableQuest)

### addQuest

```solidity
function addQuest(string _name, string _description, enum IQuests.QuestType _quest_type, address[] _resources_reward, uint256[] _resources_amounts, uint256 _experience_reward, struct IStats.BasicStats _stats, uint256 _cooldown, uint256 _level_required) external
```

See [Quests#addQuest](/docs/core/Quests.md#addQuest)

### updateQuest

```solidity
function updateQuest(struct IQuests.Quest _quest) external
```

See [Quests#updateQuest](/docs/core/Quests.md#updateQuest)

### startQuest

```solidity
function startQuest(bytes _id, uint256 _quest_id, struct IStats.BasicStats _stats_consumed) external
```

See [Quests#startQuest](/docs/core/Quests.md#startQuest)

### claimQuest

```solidity
function claimQuest(bytes _id) external
```

See [Quests#claimQuest](/docs/core/Quests.md#claimQuest)

### getQuest

```solidity
function getQuest(uint256 _quest_id) external view returns (struct IQuests.Quest _quest)
```

See [Quests#getQuest](/docs/core/Quests.md#getQuest)

### getCharacterCurrentQuest

```solidity
function getCharacterCurrentQuest(bytes _id) external view returns (struct IQuests.CurrentQuest _quest)
```

See [Quests#getCharacterCurrentQuest](/docs/core/Quests.md#getCharacterCurrentQuest)
