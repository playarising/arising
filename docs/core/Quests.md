# Solidity API

## Quests

This contracts stores multiple quests and enables all the characters stored on the [Civilizations](/docs/core/Civilizations.md) instance
to obtain rewards and experience from them.

Implementation of the [IQuests](/docs/interfaces/IQuests.md) interface.

### civilizations

```solidity
address civilizations
```

_Address of the [Civilizations](/docs/core/Civilizations.md) instance. \*_

### experience

```solidity
address experience
```

_Address of the [Experience](/docs/core/Experience.md) instance. \*_

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
