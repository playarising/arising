# Solidity API

## Names

This contract manages unique names for all characters in the [Civilizations](/docs/core/Civilizations.md) instance.
Some checks are based on the original Rarity names contract https://github.com/rarity-adventure/rarity-names/blob/main/contracts/rarity_names.sol
created by https://twitter.com/mat_nadler.

Implementation of the [INames](/docs/interfaces/INames.md) interface.

### civilizations

```solidity
address civilizations
```

Address of the [Civilizations](/docs/core/Civilizations.md) instance.

### experience

```solidity
address experience
```

Address of the [Experience](/docs/core/Experience.md) instance.

### names

```solidity
mapping(bytes => string) names
```

Map to track the names of the characters.

### claimed_names

```solidity
mapping(string => bool) claimed_names
```

Map to track the names availability.

### onlyAllowed

```solidity
modifier onlyAllowed(bytes _id)
```

Checks against the [Civilizations](/docs/core/Civilizations.md) instance if the `msg.sender` is the owner or
has allowance to access a composed ID.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### ChangeName

```solidity
event ChangeName(bytes _id, string _name)
```

Event emmited when the character name is changed.

Requirements:

| Name   | Type   | Description                   |
| ------ | ------ | ----------------------------- |
| \_id   | bytes  | Composed ID of the character. |
| \_name | string | New name of the character.    |

### initialize

```solidity
function initialize(address _civilizations, address _experience) public
```

Initialize.

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

### claimName

```solidity
function claimName(bytes _id, string _name) public
```

Assigns a name to a character and marks it as claimed.

Requirements:

| Name   | Type   | Description                   |
| ------ | ------ | ----------------------------- |
| \_id   | bytes  | Composed ID of the character. |
| \_name | string | Name to assign and claim.     |

### replaceName

```solidity
function replaceName(bytes _id, string _new_name) public
```

Replaces the name of a character with a name already assigned.

Requirements:

| Name       | Type   | Description                        |
| ---------- | ------ | ---------------------------------- |
| \_id       | bytes  | Composed ID of the character.      |
| \_new_name | string | Name to replace for the character. |

### clearName

```solidity
function clearName(bytes _id) public
```

Removes the assigned name to the character.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### getCharacterName

```solidity
function getCharacterName(bytes _id) public view returns (string _name)
```

External function to get the assigned name of a character.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name   | Type   | Description                         |
| ------ | ------ | ----------------------------------- |
| \_name | string | The assigned name of the character. |

### isNameAvailable

```solidity
function isNameAvailable(string _name) public view returns (bool _available)
```

External function to check if a name is available to assign.

Requirements:

| Name   | Type   | Description        |
| ------ | ------ | ------------------ |
| \_name | string | The name to check. |

| Name        | Type | Description                               |
| ----------- | ---- | ----------------------------------------- |
| \_available | bool | Boolean to know if the name is available. |

### isNameValid

```solidity
function isNameValid(string _name) public pure returns (bool _available)
```

External function to check if a name is valid to assign.

Requirements:

| Name   | Type   | Description        |
| ------ | ------ | ------------------ |
| \_name | string | The name to check. |

| Name        | Type | Description                           |
| ----------- | ---- | ------------------------------------- |
| \_available | bool | Boolean to know if the name is valid. |

### toLowerCase

```solidity
function toLowerCase(string _name) public pure returns (string _lower_case)
```

External function to convert a name to lower case.

Requirements:

| Name   | Type   | Description          |
| ------ | ------ | -------------------- |
| \_name | string | The name to convert. |

| Name         | Type   | Description                               |
| ------------ | ------ | ----------------------------------------- |
| \_lower_case | string | The provided name as a lower case string. |

### \_authorizeUpgrade

```solidity
function _authorizeUpgrade(address newImplementation) internal virtual
```

Internal function make sure upgrade proxy caller is the owner.
