# Solidity API

## Stats

This contract manages the stats points and pools for all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance.
The stats and the concept is based on the Cypher System for role playing games: http://cypher-system.com/.

Implementation of the [IStats](/docs/interfaces/IStats.md) interface.

### REFRESH_COOLDOWN_SECONDS

```solidity
uint256 REFRESH_COOLDOWN_SECONDS
```

Constant amount of seconds for refresh cooldown. \*

### base

```solidity
mapping(bytes => struct IStats.BasicStats) base
```

Map track the base stats for characters.

### pool

```solidity
mapping(bytes => struct IStats.BasicStats) pool
```

Map track the pool stats for characters.

### last_refresh

```solidity
mapping(bytes => uint256) last_refresh
```

Map track the last refresh timestamps of the characters.

### refresher

```solidity
address refresher
```

Address of the Refresher [BaseGadgetToken](/docs/base/BaseGadgetToken.md) instance.

### vitalizer

```solidity
address vitalizer
```

Address of the Vitalizer [BaseGadgetToken](/docs/base/BaseGadgetToken.md) instance.

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

### equipment

```solidity
address equipment
```

Address of the [Equipment](/docs/core/Equipment.md) instance.

### sacrifices

```solidity
mapping(bytes => uint256) sacrifices
```

Map to track the amount of points sacrificed by a character.

### refresher_usage_time

```solidity
mapping(bytes => uint256) refresher_usage_time
```

Map to track the first refresher token usage timestamps.

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

### ChangedPoints

```solidity
event ChangedPoints(bytes _id, struct IStats.BasicStats _base_stats, struct IStats.BasicStats _pool_stats)
```

Event emmited when the character base or pool points change.

Requirements:

| Name         | Type                     | Description                   |
| ------------ | ------------------------ | ----------------------------- |
| \_id         | bytes                    | Composed ID of the character. |
| \_base_stats | struct IStats.BasicStats | Pool stat points              |
| \_pool_stats | struct IStats.BasicStats | Pool stat points.             |

### initialize

```solidity
function initialize(address _civilizations, address _experience, address _equipment, address _refresher, address _vitalizer) public
```

Initialize.

Requirements:

| Name            | Type    | Description                                                                         |
| --------------- | ------- | ----------------------------------------------------------------------------------- |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance.           |
| \_experience    | address | The address of the [Experience](/docs/core/Experience.md) instance.                 |
| \_equipment     | address | The address of the [Equipment](/docs/core/Equipment.md) instance.                   |
| \_refresher     | address | Address of the Refresher [BaseGadgetToken](/docs/base/BaseGadgetToken.md) instance. |
| \_vitalizer     | address | Address of the Vitalizer [BaseGadgetToken](/docs/base/BaseGadgetToken.md) instance. |

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

### setRefreshCooldown

```solidity
function setRefreshCooldown(uint256 _cooldown) public
```

Changes the amount of seconds of cooldown between refreshes.

Requirements:

| Name       | Type    | Description                                  |
| ---------- | ------- | -------------------------------------------- |
| \_cooldown | uint256 | Amount of seconds to wait between refreshes. |

### consume

```solidity
function consume(bytes _id, struct IStats.BasicStats _stats) public
```

Removes the amount of points available on the character pool stats.

Requirements:

| Name    | Type                     | Description                   |
| ------- | ------------------------ | ----------------------------- |
| \_id    | bytes                    | Composed ID of the character. |
| \_stats | struct IStats.BasicStats | Stats to consume.             |

### sacrifice

```solidity
function sacrifice(bytes _id, struct IStats.BasicStats _stats) public
```

Removes the amount of points available on the character base stats.

Requirements:

| Name    | Type                     | Description                   |
| ------- | ------------------------ | ----------------------------- |
| \_id    | bytes                    | Composed ID of the character. |
| \_stats | struct IStats.BasicStats | Stats to consume.             |

### refresh

```solidity
function refresh(bytes _id) public
```

Refills the pool stats for the character.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### refreshWithToken

```solidity
function refreshWithToken(bytes _id) public
```

Refills the pool stats for the character spending a Refresher [BaseGadgetToken](/docs/base/BaseGadgetToken.md) token.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### vitalize

```solidity
function vitalize(bytes _id, struct IStats.BasicStats _stats) public
```

Recovers a sacrificed point spending a Vitalizer [BaseGadgetToken](/docs/base/BaseGadgetToken.md) token.

Requirements:

| Name    | Type                     | Description                   |
| ------- | ------------------------ | ----------------------------- |
| \_id    | bytes                    | Composed ID of the character. |
| \_stats | struct IStats.BasicStats | Stats to sacrifice.           |

### assignPoints

```solidity
function assignPoints(bytes _id, struct IStats.BasicStats _stats) public
```

Increases points of the base pool based on new levels.

Requirements:

| Name    | Type                     | Description                   |
| ------- | ------------------------ | ----------------------------- |
| \_id    | bytes                    | Composed ID of the character. |
| \_stats | struct IStats.BasicStats | Stats to increase.            |

### getBaseStats

```solidity
function getBaseStats(bytes _id) public view returns (struct IStats.BasicStats _stats)
```

External function that returns the base points of a character.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name    | Type                     | Description                  |
| ------- | ------------------------ | ---------------------------- |
| \_stats | struct IStats.BasicStats | Base stats of the character. |

### getPoolStats

```solidity
function getPoolStats(bytes _id) public view returns (struct IStats.BasicStats _stats)
```

External function that returns the available pool points of a character.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name    | Type                     | Description                            |
| ------- | ------------------------ | -------------------------------------- |
| \_stats | struct IStats.BasicStats | Available pool stats of the character. |

### getAvailablePoints

```solidity
function getAvailablePoints(bytes _id) public view returns (uint256 _points)
```

External function that returns the assignable points of a character.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name     | Type    | Description                           |
| -------- | ------- | ------------------------------------- |
| \_points | uint256 | Number of points available to assign. |

### getNextRefreshTime

```solidity
function getNextRefreshTime(bytes _id) public view returns (uint256 _timestamp)
```

External function that returns the next refresher timestamp for a character.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name        | Type    | Description                                   |
| ----------- | ------- | --------------------------------------------- |
| \_timestamp | uint256 | Timestamp when the next refresh is available. |

### getNextRefreshWithTokenTime

```solidity
function getNextRefreshWithTokenTime(bytes _id) public view returns (uint256 _timestamp)
```

External function that returns the next refresher timestamp for a character when using a Refresher [BaseGadgetToken](/docs/base/BaseGadgetToken.md) token.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name        | Type    | Description                                   |
| ----------- | ------- | --------------------------------------------- |
| \_timestamp | uint256 | Timestamp when the next refresh is available. |

### \_assignablePointsByLevel

```solidity
function _assignablePointsByLevel(uint256 _level) internal pure returns (uint256 _points)
```

Internal function to get the amount of points assignable by a provided level.

Requirements:

| Name    | Type    | Description                         |
| ------- | ------- | ----------------------------------- |
| \_level | uint256 | Level to get the assignable points. |

| Name     | Type    | Description                                |
| -------- | ------- | ------------------------------------------ |
| \_points | uint256 | Amount of points spendable for this level. |

### \_authorizeUpgrade

```solidity
function _authorizeUpgrade(address newImplementation) internal virtual
```

Internal function make sure upgrade proxy caller is the owner.
