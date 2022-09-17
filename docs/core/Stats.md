# Solidity API

## Stats

This contract manages the stats points and pools for all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance.
The stats and the concept is based on the Cypher System for role playing games: http://cypher-system.com/.

_Implementation of the [IStats](/docs/interfaces/IStats.md) interface._

### REFRESH_COOLDOWN_SECONDS

```solidity
uint256 REFRESH_COOLDOWN_SECONDS
```

_Amount of seconds for refresh cooldown. \*_

### base

```solidity
mapping(bytes => struct IStats.BasicStats) base
```

_Map to store the base stats from composed IDs. \*_

### pool

```solidity
mapping(bytes => struct IStats.BasicStats) pool
```

_Map to store the pool stats from composed ID. \*_

### last_refresh

```solidity
mapping(bytes => uint256) last_refresh
```

_Map to store the the last refresh from composed ID. \*_

### refresher

```solidity
address refresher
```

_Implementation of the `Refresher` \*_

### vitalizer

```solidity
address vitalizer
```

_Implementation of the `Vitalizer` \*_

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` instance. \*_

### experience

```solidity
address experience
```

_Address of the `Experience` instance. \*_

### sacrifices

```solidity
mapping(bytes => uint256) sacrifices
```

_Map to track the amount of points sacrificed by a character. \*_

### refresher_usage_time

```solidity
mapping(bytes => uint256) refresher_usage_time
```

_Map to track the first refresher usage timestamp. \*_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

### constructor

```solidity
constructor(address _civilizations, address _experience) public
```

Constructor.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |
| \_experience    | address | The address of the [Experience](/docs/core/Experience.md) instance.       |

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

### setRefreshToken

```solidity
function setRefreshToken(address _token) public
```

_Sets the `Refresher` instance.
@param \_token address of the `Refresher` instance._

### setVitalizerToken

```solidity
function setVitalizerToken(address _token) public
```

_Sets the `Vitalizer` instance.
@param \_token address of the `Vitalizer` instance._

### consume

```solidity
function consume(bytes id, struct IStats.BasicStats stats) public
```

_Reduces stats points from the pool.
@param id Composed ID of the token.
@param stats Amount of points reducing._

### sacrifice

```solidity
function sacrifice(bytes id, struct IStats.BasicStats stats) public
```

_Reduces points to the base stats forever.
@param id Composed ID of the token.
@param stats Amount of points sacrificing._

### refresh

```solidity
function refresh(bytes id) public
```

_Performs a refresh filling the pool stats from the base stats.
@param id Composed ID of the token._

### refreshWithToken

```solidity
function refreshWithToken(bytes id) public
```

_Performs a refresh filling the pool stats from the base stats without cooldown spending `RefreshToken` (max 20 points per stat).
@param id Composed ID of the token._

### consumeVitalizer

```solidity
function consumeVitalizer(bytes id, struct IStats.BasicStats stats) public
```

_Consumes a vitalizer token to increase one point of a base stat.
@param id Composed ID of the token.
@param stats Amount of points increasing._

### assignPoints

```solidity
function assignPoints(bytes id, struct IStats.BasicStats stats) public
```

_Assigns the points to the base pool.
@param id Composed ID of the token.
@param stats Amount of points to assign._

### getBaseStats

```solidity
function getBaseStats(bytes id) public view returns (struct IStats.BasicStats)
```

_Returns the base stats of the composed ID.
@param id Composed ID of the token._

### getPoolStats

```solidity
function getPoolStats(bytes id) public view returns (struct IStats.BasicStats)
```

_Returns the available pool stats of the composed ID.
@param id Composed ID of the token._

### getAvailablePoints

```solidity
function getAvailablePoints(bytes id) public view returns (uint256)
```

_Returns the amount of points available to assign.
@param id Composed ID of the token._

### getNextRefreshTime

```solidity
function getNextRefreshTime(bytes id) public view returns (uint256)
```

_Returns the time for the next free refresh.
@param id Composed ID of the token._

### getNextRefreshWithTokenTime

```solidity
function getNextRefreshWithTokenTime(bytes id) public view returns (uint256)
```

_Returns the time of the refresh with tokens reset.
@param id Composed ID of the token._

### \_assignablePointsByLevel

```solidity
function _assignablePointsByLevel(uint256 level) internal pure returns (uint256)
```

_Returns the amount of total asignable points by level.
@param level Level number to check points._
