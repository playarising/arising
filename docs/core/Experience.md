# Solidity API

## Experience

_`Experience` is the contract to manage the storage of experience and missions from all the civilizations._

### experience

```solidity
mapping(bytes => uint256) experience
```

_Map to store the experience from composed ID. \*_

### levels

```solidity
address levels
```

_Address of the `Levels` implementation. \*_

### civilizations

```solidity
address civilizations
```

_Address of the `Civilizations` implementation. \*_

### authorized

```solidity
mapping(address => bool) authorized
```

_Map to store the list of authorized addresses to assign experience. \*_

### onlyAuthorized

```solidity
modifier onlyAuthorized()
```

_Checks if `msg.sender` is authorized to assign experience._

### constructor

```solidity
constructor(address _levels, address _civilizations) public
```

_Constructor._

| Name            | Type    | Description                                  |
| --------------- | ------- | -------------------------------------------- |
| \_levels        | address | The address of the `Levels` instance.        |
| \_civilizations | address | The address of the `Civilizations` instance. |

### assignExperience

```solidity
function assignExperience(bytes id, uint256 amount) public
```

_Adds experience to the character from a composed ID.
@param id Composed ID of the token._

### addAuthority

```solidity
function addAuthority(address authority) public
```

_Adds an authority to assign experience.
@param authority Address of the authority to assign._

### removeAuthority

```solidity
function removeAuthority(address authority) public
```

_Removes an authority to assign experience.
@param authority Address of the authority to remove._

### getExperience

```solidity
function getExperience(bytes id) public view returns (uint256)
```

_Returns the experience points of the token from a composed ID.
@param id Composed ID of the token._

### getLevel

```solidity
function getLevel(bytes id) public view returns (uint256)
```

_Returns the level of a token from a composed ID.
@param id Composed ID of the token._

### getExperienceForNextLevel

```solidity
function getExperienceForNextLevel(bytes id) public view returns (uint256)
```

_Returns the amount of experience required to reach the next level.
@param id Composed ID of the token._
