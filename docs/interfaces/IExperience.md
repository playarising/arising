# Solidity API

## IExperience

Interface for the [Experience](/docs/core/Experience.md) contract.

### assignExperience

```solidity
function assignExperience(bytes _id, uint256 _amount) external
```

See [Experience#assignExperience](/docs/core/Experience.md#assignExperience)

### addAuthority

```solidity
function addAuthority(address _authority) external
```

See [Experience#addAuthority](/docs/core/Experience.md#addAuthority)

### removeAuthority

```solidity
function removeAuthority(address _authority) external
```

See [Experience#removeAuthority](/docs/core/Experience.md#removeAuthority)

### getExperience

```solidity
function getExperience(bytes _id) external view returns (uint256 _experience)
```

See [Experience#getExperience](/docs/core/Experience.md#getExperience)

### getLevel

```solidity
function getLevel(bytes _id) external view returns (uint256 _level)
```

See [Experience#getLevel](/docs/core/Experience.md#getLevel)

### getExperienceForNextLevel

```solidity
function getExperienceForNextLevel(bytes _id) external view returns (uint256 _experience)
```

See [Experience#getExperienceForNextLevel](/docs/core/Experience.md#getExperienceForNextLevel)
