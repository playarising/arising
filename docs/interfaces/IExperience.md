# Solidity API

## IExperience

Interface for the [Experience](/docs/core/Experience.md) contract.

### assignExperience

```solidity
function assignExperience(bytes id, uint256 amount) external
```

### addAuthority

```solidity
function addAuthority(address authority) external
```

### removeAuthority

```solidity
function removeAuthority(address authority) external
```

### getExperience

```solidity
function getExperience(bytes id) external view returns (uint256)
```

### getLevel

```solidity
function getLevel(bytes id) external view returns (uint256)
```

### getExperienceForNextLevel

```solidity
function getExperienceForNextLevel(bytes id) external view returns (uint256)
```
