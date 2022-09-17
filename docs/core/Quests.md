# Solidity API

## Quests

This contracts stores multiple quests and enables all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance
to obtain rewards and experience from them.

Implementation of the [IQuests](/docs/interfaces/IQuests.md) interface.

### civilizations

```solidity
address civilizations
```

_Address of the [Civilizations](/docs/core/Civilizations.md) instance. \*_

### experience

```solidity
address experience
```

_Address of the [Experience](/docs/core/Experience.md) instance. \*_

### stats

```solidity
address stats
```

_Address of the [Stats](/docs/core/Stats.md) instance. \*_

### gold

```solidity
address gold
```

_The address of the [Gold](/docs/gadgets/Gold.md) instance. \*_

### quests

```solidity
mapping(uint256 => struct IQuests.Quest) quests
```

Map to track all the quests.

### \_quests

```solidity
uint256[] _quests
```

Array to track a full list of quests IDs.

### character_quests

```solidity
mapping(bytes => struct IQuests.CurrentQuest) character_quests
```

_Map to track quests cooldowns for each character. \*_

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

### constructor

```solidity
constructor(address _civilizations, address _experience, address _stats, address _gold) public
```

Constructor.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |
| \_experience    | address | The address of the [Experience](/docs/core/Experience.md) instance.       |
| \_stats         | address | The address of the [Stats](/docs/core/Stats.md) instance.                 |
| \_gold          | address | The address of the [Gold](/docs/gadgets/Gold.md) instance.                |

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

### disableItem

```solidity
function disableItem(uint256 _id) public
```

Disables a quest.

Requirements:

| Name | Type    | Description      |
| ---- | ------- | ---------------- |
| \_id | uint256 | ID of the quest. |

### enableItem

```solidity
function enableItem(uint256 _id) public
```

Enables a quest.

Requirements:

| Name | Type    | Description      |
| ---- | ------- | ---------------- |
| \_id | uint256 | ID of the quest. |

### startQuest

```solidity
function startQuest(bytes id, uint256 quest) public
```

_Claims a recipe already crafted._

| Name  | Type    | Description                   |
| ----- | ------- | ----------------------------- |
| id    | bytes   | Composed ID of the character. |
| quest | uint256 |                               |

### claimQuest

```solidity
function claimQuest(bytes id) public
```

_Claims a recipe already crafted._

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |

### getItem

```solidity
function getItem(uint256 _id) public view returns (struct IQuests.Quest _quest)
```

Returns the full information of a quest.

Requirements:

| Name | Type    | Description      |
| ---- | ------- | ---------------- |
| \_id | uint256 | ID of the quest. |

| Name    | Type                 | Description             |
| ------- | -------------------- | ----------------------- |
| \_quest | struct IQuests.Quest | Full quest information. |

### \_isAvailableForQuest

```solidity
function _isAvailableForQuest(bytes id) internal view returns (bool)
```

_Internal function to check if the craft slot is available to craft._

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |

### \_isQuestClaimable

```solidity
function _isQuestClaimable(bytes id) internal view returns (bool)
```

_Internal function to check if a craft slot is ready to claim._

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |
