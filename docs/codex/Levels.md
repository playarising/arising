# Solidity API

## Levels

This contract is a static storage with utility functions to determine the level
table for the [Experience](/docs/core/Experience.md) contract.

Implementation of the [ILevels](/docs/interfaces/ILevels.md) interface.

### levels

```solidity
mapping(uint256 => struct ILevels.Level) levels
```

Map to track the levels.

### initialize

```solidity
function initialize() public
```

Initialize.
Initializes the lable table.

### getLevel

```solidity
function getLevel(uint256 _experience) public view returns (uint256 _level)
```

External function to return the level number from an experience amount.

Requirements:

| Name         | Type    | Description                    |
| ------------ | ------- | ------------------------------ |
| \_experience | uint256 | Amount of experience to check. |

| Name    | Type    | Description                              |
| ------- | ------- | ---------------------------------------- |
| \_level | uint256 | Level number of the provided experience. |

### getExperience

```solidity
function getExperience(uint256 _level) public view returns (uint256 _experience)
```

External function to return the total amount of experience required to reach a level.

Requirements:

| Name    | Type    | Description                    |
| ------- | ------- | ------------------------------ |
| \_level | uint256 | Amount of experience to check. |

| Name         | Type    | Description                                      |
| ------------ | ------- | ------------------------------------------------ |
| \_experience | uint256 | Experience required to reach the level provided. |

### \_authorizeUpgrade

```solidity
function _authorizeUpgrade(address newImplementation) internal virtual
```

Internal function make sure upgrade proxy caller is the owner.
