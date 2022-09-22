# Solidity API

## IStats

Interface for the [Stats](/docs/core/Stats.md) contract.

### BasicStats

```solidity
struct BasicStats {
  uint256 might;
  uint256 speed;
  uint256 intellect;
}

```

### pause

```solidity
function pause() external
```

See [Stats#pause](/docs/codex/Stats.md#pause)

### unpause

```solidity
function unpause() external
```

See [Stats#unpause](/docs/codex/Stats.md#unpause)

### setRefreshCooldown

```solidity
function setRefreshCooldown(uint256 _cooldown) external
```

See [Stats#setRefreshCooldown](/docs/codex/Stats.md#setRefreshCooldown)

### consume

```solidity
function consume(bytes _id, struct IStats.BasicStats _stats) external
```

See [Stats#consume](/docs/codex/Stats.md#consume)

### sacrifice

```solidity
function sacrifice(bytes _id, struct IStats.BasicStats _stats) external
```

See [Stats#sacrifice](/docs/codex/Stats.md#sacrifice)

### refresh

```solidity
function refresh(bytes _id) external
```

See [Stats#refresh](/docs/codex/Stats.md#refresh)

### refreshWithToken

```solidity
function refreshWithToken(bytes _id) external
```

See [Stats#refreshWithToken](/docs/codex/Stats.md#refreshWithToken)

### vitalize

```solidity
function vitalize(bytes _id, struct IStats.BasicStats _stats) external
```

See [Stats#vitalize](/docs/codex/Stats.md#vitalize)

### assignPoints

```solidity
function assignPoints(bytes _id, struct IStats.BasicStats _stats) external
```

See [Stats#assignPoints](/docs/codex/Stats.md#assignPoints)

### getBaseStats

```solidity
function getBaseStats(bytes _id) external view returns (struct IStats.BasicStats _stats)
```

See [Stats#getBaseStats](/docs/codex/Stats.md#getBaseStats)

### getPoolStats

```solidity
function getPoolStats(bytes _id) external view returns (struct IStats.BasicStats _stats)
```

See [Stats#getPoolStats](/docs/codex/Stats.md#getPoolStats)

### getAvailablePoints

```solidity
function getAvailablePoints(bytes _id) external view returns (uint256 _points)
```

See [Stats#getAvailablePoints](/docs/codex/Stats.md#getAvailablePoints)

### getNextRefreshTime

```solidity
function getNextRefreshTime(bytes _id) external view returns (uint256 _timestamp)
```

See [Stats#getNextRefreshTime](/docs/codex/Stats.md#getNextRefreshTime)

### getNextRefreshWithTokenTime

```solidity
function getNextRefreshWithTokenTime(bytes _id) external view returns (uint256 _timestamp)
```

See [Stats#getNextRefreshWithTokenTime](/docs/codex/Stats.md#getNextRefreshWithTokenTime)
