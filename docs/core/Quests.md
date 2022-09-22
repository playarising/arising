# Solidity API

## Quests

This contracts stores multiple quests and enables all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance
to obtain rewards and experience from them.

Implementation of the [IQuests](/docs/interfaces/IQuests.md) interface.

### civilizations

```solidity
address civilizations
```

Address of the [Civilizations](/docs/core/Civilizations.md) instance.

### experience

```solidity
address experience
```

Address of the [Experience](/docs/core/Experience.md) instance.

### stats

```solidity
address stats
```

Address of the [Stats](/docs/core/Stats.md) instance.

### quests

```solidity
mapping(uint256 => struct IQuests.Quest) quests
```

Map to track all the available quests.

### \_quests

```solidity
uint256[] _quests
```

Array to track a full list of quests IDs.

### character_quests

```solidity
mapping(bytes => struct IQuests.CurrentQuest) character_quests
```

Map to track current quests for all characters. \*

### onlyAllowed

```solidity
modifier onlyAllowed(bytes _id)
```

Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
has allowance to access a composed ID.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### AddQuest

```solidity
event AddQuest(uint256 _quest_id, string _name, string _description)
```

Event emmited when the [addQuest](#addQuest) function is called.

Requirements:

| Name          | Type    | Description            |
| ------------- | ------- | ---------------------- |
| \_quest_id    | uint256 | ID of the quest added. |
| \_name        | string  | Name of the quest.     |
| \_description | string  | Quest description      |

### QuestUpdate

```solidity
event QuestUpdate(uint256 _quest_id, string _name, string _description)
```

Event emmited when the [updateQuest](#updateQuest) function is called.

Requirements:

| Name          | Type    | Description            |
| ------------- | ------- | ---------------------- |
| \_quest_id    | uint256 | ID of the quest added. |
| \_name        | string  | Name of the quest.     |
| \_description | string  | Quest description      |

### EnableQuest

```solidity
event EnableQuest(uint256 _quest_id)
```

Event emmited when the [enableQuest](#enableQuest) function is called.

Requirements:

| Name       | Type    | Description              |
| ---------- | ------- | ------------------------ |
| \_quest_id | uint256 | ID of the quest enabled. |

### DisableQuest

```solidity
event DisableQuest(uint256 _quest_id)
```

Event emmited when the [disableQuest](#disableQuest) function is called.

Requirements:

| Name       | Type    | Description                |
| ---------- | ------- | -------------------------- |
| \_quest_id | uint256 | ID of the recipe disabled. |

### initialize

```solidity
function initialize(address _civilizations, address _experience, address _stats) public
```

Constructor.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |
| \_experience    | address | The address of the [Experience](/docs/core/Experience.md) instance.       |
| \_stats         | address | The address of the [Stats](/docs/core/Stats.md) instance.                 |

### pause

```solidity
function pause() public
```

Pauses the contract

### unpause

```solidity
function unpause() public
```

Resumes the contract

### disableQuest

```solidity
function disableQuest(uint256 _quest_id) public
```

Disables a quest for characters.

Requirements:

| Name       | Type    | Description      |
| ---------- | ------- | ---------------- |
| \_quest_id | uint256 | ID of the quest. |

### enableQuest

```solidity
function enableQuest(uint256 _quest_id) public
```

Enables a quest for characters.

Requirements:

| Name       | Type    | Description      |
| ---------- | ------- | ---------------- |
| \_quest_id | uint256 | ID of the quest. |

### addQuest

```solidity
function addQuest(string _name, string _description, enum IQuests.QuestType _quest_type, address[] _resources_reward, uint256[] _resources_amounts, uint256 _experience_reward, struct IStats.BasicStats _stats, uint256 _cooldown, uint256 _level_required) public
```

Adds a new quest for characters.

Requirements:

| Name                | Type                     | Description                                                                                    |
| ------------------- | ------------------------ | ---------------------------------------------------------------------------------------------- |
| \_name              | string                   | Name of the quest.                                                                             |
| \_description       | string                   | Description of the quest.                                                                      |
| \_quest_type        | enum IQuests.QuestType   | Type of the added quest.                                                                       |
| \_resources_reward  | address[]                | Array of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) instances to reward for the quest. |
| \_resources_amounts | uint256[]                | Array of amounts for each resource reward.                                                     |
| \_experience_reward | uint256                  | Amount of experience rewarded for the quest.                                                   |
| \_stats             | struct IStats.BasicStats | Stats to consume from the pool for the quest.                                                  |
| \_cooldown          | uint256                  | Number of seconds for the quest cooldown.                                                      |
| \_level_required    | uint256                  | Minimum level required to start the quest.                                                     |

### updateQuest

```solidity
function updateQuest(struct IQuests.Quest _quest) public
```

Updates a previously added quest.

Requirements:

| Name    | Type                 | Description                    |
| ------- | -------------------- | ------------------------------ |
| \_quest | struct IQuests.Quest | Full information of the quest. |

### startQuest

```solidity
function startQuest(bytes _id, uint256 _quest_id, struct IStats.BasicStats _stats_consumed) public
```

Starts a quest for the character provided.

Requirements:

| Name             | Type                     | Description                               |
| ---------------- | ------------------------ | ----------------------------------------- |
| \_id             | bytes                    | Composed ID of the character.             |
| \_quest_id       | uint256                  | ID of the quest.                          |
| \_stats_consumed | struct IStats.BasicStats | Amount of stats to consume for the quest. |

### claimQuest

```solidity
function claimQuest(bytes _id) public
```

Claims a finished quest for the character.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### getQuest

```solidity
function getQuest(uint256 _quest_id) public view returns (struct IQuests.Quest _quest)
```

Returns the full information of a quest.

Requirements:

| Name       | Type    | Description      |
| ---------- | ------- | ---------------- |
| \_quest_id | uint256 | ID of the quest. |

| Name    | Type                 | Description             |
| ------- | -------------------- | ----------------------- |
| \_quest | struct IQuests.Quest | Full quest information. |

### getCharacterCurrentQuest

```solidity
function getCharacterCurrentQuest(bytes _id) public view returns (struct IQuests.CurrentQuest _quest)
```

Returns the character current quest information.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name    | Type                        | Description                          |
| ------- | --------------------------- | ------------------------------------ |
| \_quest | struct IQuests.CurrentQuest | Current character quest information. |

### \_isAvailableForQuest

```solidity
function _isAvailableForQuest(bytes _id) internal view returns (bool _available)
```

Internal function to check if the character is able to start a quest.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name        | Type | Description                                                     |
| ----------- | ---- | --------------------------------------------------------------- |
| \_available | bool | Boolean to know if the character is available to start a quest. |

### \_isQuestClaimable

```solidity
function _isQuestClaimable(bytes _id) internal view returns (bool _claimable)
```

Internal function to check if the last character quest is claimable.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name        | Type | Description                                     |
| ----------- | ---- | ----------------------------------------------- |
| \_claimable | bool | Boolean to know if the last quest is claimable. |

### \_authorizeUpgrade

```solidity
function _authorizeUpgrade(address newImplementation) internal virtual
```

Internal function make sure upgrade proxy caller is the owner.
