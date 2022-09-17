# Solidity API

## Experience

This contract tracks and assigns experience of all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance.

Implementation of the [IExperience](/docs/interfaces/IExperience.md) interface.

### civilizations

```solidity
address civilizations
```

Address of the [Civilizations](/docs/core/Civilizations.md) instance.

### levels

```solidity
address levels
```

Address of the [Levels](/docs/codex/Levels.md) instance.

### authorized

```solidity
mapping(address => bool) authorized
```

_Map to store the list of authorized addresses to assign experience._

### experience

```solidity
mapping(bytes => uint256) experience
```

Map to track the experience of composed IDs.

### onlyAuthorized

```solidity
modifier onlyAuthorized()
```

Checks against if the `msg.sender` is authorized to assign experience.

### constructor

```solidity
constructor(address _levels, address _civilizations) public
```

Constructor.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_levels        | address | The address of the [Levels](/docs/codex/Levels.md) instance.              |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |

### setLevel

```solidity
function setLevel(address _levels) public
```

Replaces the address of the [Levels](/docs/codex/Levels.md) instance to determine character levels.

Requirements:

| Name     | Type    | Description                                              |
| -------- | ------- | -------------------------------------------------------- |
| \_levels | address | Address of the [Levels](/docs/codex/Levels.md) instance. |

### assignExperience

```solidity
function assignExperience(bytes _id, uint256 _amount) public
```

Assigns new experience to the composed ID provided.

Requirements:

| Name     | Type    | Description                      |
| -------- | ------- | -------------------------------- |
| \_id     | bytes   | Composed ID of the character.    |
| \_amount | uint256 | The amount of experience to add. |

### addAuthority

```solidity
function addAuthority(address _authority) public
```

Assigns a new address as an authority to assign experience.

Requirements:

| Name        | Type    | Description                |
| ----------- | ------- | -------------------------- |
| \_authority | address | Address to give authority. |

### removeAuthority

```solidity
function removeAuthority(address _authority) public
```

Removes an authority to assign experience.

Requirements:

| Name        | Type    | Description                |
| ----------- | ------- | -------------------------- |
| \_authority | address | Address to give authority. |

### getExperience

```solidity
function getExperience(bytes _id) public view returns (uint256 _experience)
```

External function to return the total experience of a composed ID.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name         | Type    | Description                        |
| ------------ | ------- | ---------------------------------- |
| \_experience | uint256 | Total experience of the character. |

### getLevel

```solidity
function getLevel(bytes _id) public view returns (uint256 _level)
```

External function to return the level of a composed ID.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name    | Type    | Description                    |
| ------- | ------- | ------------------------------ |
| \_level | uint256 | Level number of the character. |

### getExperienceForNextLevel

```solidity
function getExperienceForNextLevel(bytes _id) public view returns (uint256 _experience)
```

External function to return the total experience required to reach the next level a composed ID.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name         | Type    | Description                                        |
| ------------ | ------- | -------------------------------------------------- |
| \_experience | uint256 | Total experience required to reach the next level. |
