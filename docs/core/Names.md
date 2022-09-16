# Solidity API

## Names

_`Names` is a contract manage the names of Arising characters.
Some checks are based on the original Rarity names contract https://github.com/rarity-adventure/rarity-names/blob/main/contracts/rarity_names.sol
created by https://twitter.com/mat_nadler._

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` implementation. \*_

### experience

```solidity
address experience
```

_Address of the `Experience` implementation. \*_

### names

```solidity
mapping(bytes => string) names
```

_Map storing the names for each character. \*_

### claimed_names

```solidity
mapping(string => bool) claimed_names
```

_Names claimed. \*_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

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

### claimName

```solidity
function claimName(bytes id, string name) public
```

_Assigns a name to a character and stores to prevent duplicates.
@param id Composed ID of the token._

| Name | Type   | Description    |
| ---- | ------ | -------------- |
| id   | bytes  |                |
| name | string | Name to claim. |

### replaceName

```solidity
function replaceName(bytes id, string newName) public
```

_Changes a name for a character.
@param id Composed ID of the token._

| Name    | Type   | Description           |
| ------- | ------ | --------------------- |
| id      | bytes  |                       |
| newName | string | Name to replace with. |

### clearName

```solidity
function clearName(bytes id) public
```

_Removes the name of the character.
@param id Composed ID of the token._

### getTokenName

```solidity
function getTokenName(bytes id) public view returns (string)
```

_Returns the name of the composed ID._

| Name | Type  | Description               |
| ---- | ----- | ------------------------- |
| id   | bytes | Composed ID of the token. |

### isNameAvailable

```solidity
function isNameAvailable(string str) public view returns (bool)
```

_Checks if a given name is available to use._

| Name | Type   | Description        |
| ---- | ------ | ------------------ |
| str  | string | String to checked. |

### isNameValid

```solidity
function isNameValid(string str) public pure returns (bool)
```

_Checks if a given name is valid (Alphanumeric and spaces without leading or trailing space)._

| Name | Type   | Description        |
| ---- | ------ | ------------------ |
| str  | string | String to checked. |

### toLowerCase

```solidity
function toLowerCase(string str) public pure returns (string)
```

_Converts a string to lowercase._

| Name | Type   | Description        |
| ---- | ------ | ------------------ |
| str  | string | String to convert. |
