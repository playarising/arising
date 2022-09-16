# Solidity API

## Levels

This contract is a static storage with utility functions to determine the level
table for the {Experience} contract.

_Implementation of the {ILevels} interface._

### levels

```solidity
mapping(uint256 => struct ILevels.Level) levels
```

Map to track the levels.

### constructor

```solidity
constructor() public
```

Constructor.

_Initializes the lable table._

### getLevel

```solidity
function getLevel(uint256 _experience) public view returns (uint256)
```

External function to return the level number from an experience amount.

Requirements:

| Name         | Type    | Description                    |
| ------------ | ------- | ------------------------------ |
| \_experience | uint256 | Amount of experience to check. |

| Name | Type    | Description |
| ---- | ------- | ----------- |
| [0]  | uint256 | uint256     |

### getExperience

```solidity
function getExperience(uint256 _level) public view returns (uint256)
```

External function to return the total amount of experience required to reach a level.

Requirements:

| Name    | Type    | Description                    |
| ------- | ------- | ------------------------------ |
| \_level | uint256 | Amount of experience to check. |

| Name | Type    | Description |
| ---- | ------- | ----------- |
| [0]  | uint256 | uint256     |
