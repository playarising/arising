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

### setRefreshToken

```solidity
function setRefreshToken(address _token) external
```

### setVitalizerToken

```solidity
function setVitalizerToken(address _token) external
```

### consume

```solidity
function consume(bytes id, struct IStats.BasicStats stats) external
```

### sacrifice

```solidity
function sacrifice(bytes id, struct IStats.BasicStats stats) external
```

### refresh

```solidity
function refresh(bytes id) external
```

### refreshWithToken

```solidity
function refreshWithToken(bytes id) external
```

### consumeVitalizer

```solidity
function consumeVitalizer(bytes id, struct IStats.BasicStats stats) external
```

### assignPoints

```solidity
function assignPoints(bytes id, struct IStats.BasicStats stats) external
```

### getBaseStats

```solidity
function getBaseStats(bytes id) external view returns (struct IStats.BasicStats)
```

### getPoolStats

```solidity
function getPoolStats(bytes id) external view returns (struct IStats.BasicStats)
```

### getAvailablePoints

```solidity
function getAvailablePoints(bytes id) external view returns (uint256)
```

### getNextRefreshTime

```solidity
function getNextRefreshTime(bytes id) external view returns (uint256)
```

### getNextRefreshWithTokenTime

```solidity
function getNextRefreshWithTokenTime(bytes id) external view returns (uint256)
```
