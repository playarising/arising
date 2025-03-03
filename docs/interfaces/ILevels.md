# Solidity API

## ILevels

Interface for the [Levels](/docs/codex/Levels.md) contract.

### Level

```solidity
struct Level {
  uint256 min;
  uint256 max;
}

```

### getLevel

```solidity
function getLevel(uint256 _experience) external view returns (uint256)
```

See [Levels#getLevel](/docs/codex/Levels.md#getLevel)

### getExperience

```solidity
function getExperience(uint256 _level) external view returns (uint256)
```

See [Levels#getExperience](/docs/codex/Levels.md#getExperience)
