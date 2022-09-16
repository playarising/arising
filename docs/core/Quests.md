# Solidity API

## Quests

_`Quests` is a contract to manage the different missions characters can do._

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

### constructor

```solidity
constructor(address _civilizations, address _experience) public
```

_Constructor._

| Name            | Type    | Description                                  |
| --------------- | ------- | -------------------------------------------- |
| \_civilizations | address | The address of the `Civilizations` instance. |
| \_experience    | address | The address of the `Experience` instance.    |

### pause

```solidity
function pause() public
```

_Pauses the contract_

### unpause

```solidity
function unpause() public
```

_Resumes the contract_
