# Solidity API

## Civilizations

This contract stores all the [BaseERC721](/docs/base/BaseERC721.md) instances usable on the environmne. The contract
is in charge of token ownership verifications and generating/storing composable IDs for each character.

Implementation of the [ICivilizations](/docs/interfaces/ICivilizations.md) interface.

### civilizations

```solidity
mapping(address => uint256) civilizations
```

Map to track the supported [BaseERC721](/docs/base/BaseERC721.md) instances.

### \_civilizations

```solidity
address[] _civilizations
```

Array to track the [BaseERC721](/docs/base/BaseERC721.md) IDs.

### \_minters

```solidity
mapping(address => uint256) _minters
```

Map to track the count of address mints.

### token

```solidity
address token
```

Constant for address of the `ERC20` token used to purchase.

### character_upgrades

```solidity
mapping(bytes => mapping(uint256 => bool)) character_upgrades
```

Map to track the character upgrades.

### upgrades

```solidity
mapping(uint256 => struct ICivilizations.Upgrade) upgrades
```

Map to track the upgrades information.

### constructor

```solidity
constructor(address _token) public
```

Constructor.

Requirements:

| Name    | Type    | Description                                     |
| ------- | ------- | ----------------------------------------------- |
| \_token | address | Address of the token used to purchase upgrades. |

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

### setInitializeUpgrade

```solidity
function setInitializeUpgrade(uint256 _upgrade_id, bool _available) public
```

Activates or deactivates an upgrade purchase.

Requirements:

| Name         | Type    | Description                     |
| ------------ | ------- | ------------------------------- |
| \_upgrade_id | uint256 | ID of the upgrade to change.    |
| \_available  | bool    | Boolean to activate/deactivate. |

### setUpgradePrice

```solidity
function setUpgradePrice(uint256 _upgrade_id, uint256 _price) public
```

Sets the price to purchase an upgrade.

Requirements:

| Name         | Type    | Description                              |
| ------------ | ------- | ---------------------------------------- |
| \_upgrade_id | uint256 | ID of the upgrade to change.             |
| \_price      | uint256 | Amount of tokens to pay for the upgrade. |

### setToken

```solidity
function setToken(address _token) public
```

Changes the token address to charge.

Requirements:

| Name    | Type    | Description                         |
| ------- | ------- | ----------------------------------- |
| \_token | address | Address of the new token to charge. |

### addCivilization

```solidity
function addCivilization(address _civilization) public
```

Adds a new [BaseERC721](/docs/base/BaseERC721.md) instance to the valid civilizations.

Requirements:

| Name           | Type    | Description                                                            |
| -------------- | ------- | ---------------------------------------------------------------------- |
| \_civilization | address | Address of the [BaseERC721](/docs/base/BaseERC721.md) instance to add. |

### mint

```solidity
function mint(address _civilization) public
```

Creates a new token of the valid civilizations list to the `msg.sender`.

Requirements:

| Name           | Type    | Description                                                            |
| -------------- | ------- | ---------------------------------------------------------------------- |
| \_civilization | address | Address of the [BaseERC721](/docs/base/BaseERC721.md) instance to add. |

### buyUpgrade

```solidity
function buyUpgrade(bytes _id, uint256 _upgrade_id) public
```

Purchase an upgrade and marks it as available for the provided composed ID.

Requirements:

| Name         | Type    | Description                    |
| ------------ | ------- | ------------------------------ |
| \_id         | bytes   | Composed ID of the character.  |
| \_upgrade_id | uint256 | ID of the upgrade to purchase. |

### withdraw

```solidity
function withdraw() public
```

Transfers the total amount of tokens stored in the contract to the owner .

### getID

```solidity
function getID(address _civilization) public view returns (uint256 _civilization_id)
```

External function to return the internal ID of a valid civilization.

Requirements:

| Name           | Type    | Description                                                            |
| -------------- | ------- | ---------------------------------------------------------------------- |
| \_civilization | address | Address of the [BaseERC721](/docs/base/BaseERC721.md) instance to add. |

| Name              | Type    | Description                        |
| ----------------- | ------- | ---------------------------------- |
| \_civilization_id | uint256 | Internal ID of the civilization. ] |

### getCharacterUpgrades

```solidity
function getCharacterUpgrades(bytes _id) public view returns (bool[3] _upgrades)
```

External function to return the upgrades for a composed ID.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name       | Type    | Description                         |
| ---------- | ------- | ----------------------------------- |
| \_upgrades | bool[3] | Array of booleans for each upgrade. |

### getUpgradeInformation

```solidity
function getUpgradeInformation(uint256 _upgrade_id) public view returns (struct ICivilizations.Upgrade _upgrade)
```

External function to return global status of an upgrade.

Requirements:

| Name         | Type    | Description        |
| ------------ | ------- | ------------------ |
| \_upgrade_id | uint256 | ID of the upgrade. |

| Name      | Type                          | Description          |
| --------- | ----------------------------- | -------------------- |
| \_upgrade | struct ICivilizations.Upgrade | Upgrade information. |

### getCivilizations

```solidity
function getCivilizations() public view returns (address[] _valid_civilizations)
```

Returns the list of valid civilizations instances.

| Name                  | Type      | Description                            |
| --------------------- | --------- | -------------------------------------- |
| \_valid_civilizations | address[] | Array of valid civilizations intances. |

### getTokenID

```solidity
function getTokenID(address _civilization, uint256 _token_id) public view returns (bytes _id)
```

Returns the composed ID of a token from a valid civilization.

Requirements:

| Name           | Type    | Description                                                            |
| -------------- | ------- | ---------------------------------------------------------------------- |
| \_civilization | address | Address of the [BaseERC721](/docs/base/BaseERC721.md) instance to add. |
| \_token_id     | uint256 | ID of the token to get the composed ID.                                |

| Name | Type  | Description               |
| ---- | ----- | ------------------------- |
| \_id | bytes | Composed ID of the token. |

### isAllowed

```solidity
function isAllowed(address _spender, bytes _id) public view returns (bool _allowed)
```

External function to check if the `msg.sender` can access a token.

Requirements:
@param \_spender Address to check ownership or allowance.

| Name      | Type    | Description                   |
| --------- | ------- | ----------------------------- |
| \_spender | address |                               |
| \_id      | bytes   | Composed ID of the character. |

| Name      | Type | Description                          |
| --------- | ---- | ------------------------------------ |
| \_allowed | bool | Boolean to check if access is valid. |

### exists

```solidity
function exists(bytes _id) public view returns (bool _exist)
```

External function to check a composed ID is already minted.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name    | Type | Description                              |
| ------- | ---- | ---------------------------------------- |
| \_exist | bool | Boolean to check if the token is minted. |

### ownerOf

```solidity
function ownerOf(bytes _id) public view returns (address _owner)
```

External function to return the owner a composed ID.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name    | Type    | Description                        |
| ------- | ------- | ---------------------------------- |
| \_owner | address | Address of the owner of the token. |

### \_addMint

```solidity
function _addMint(address _minter) internal
```

Internal function to add a mint count for the `msg.sender`.

Requirements:

| Name     | Type    | Description            |
| -------- | ------- | ---------------------- |
| \_minter | address | Address of the minter. |

### \_canMint

```solidity
function _canMint(address _minter) internal view returns (bool)
```

Internal function check if the `msg.sender` can mint a token.

Requirements:

| Name     | Type    | Description            |
| -------- | ------- | ---------------------- |
| \_minter | address | Address of the minter. |

### \_decodeID

```solidity
function _decodeID(bytes _id) internal pure returns (uint256 _civilization, uint256 _token_id)
```

Internal function to decode a composed ID to a civilization instance and token ID.

Requirements:

| Name | Type  | Description  |
| ---- | ----- | ------------ |
| \_id | bytes | Composed ID. |

| Name           | Type    | Description                          |
| -------------- | ------- | ------------------------------------ |
| \_civilization | uint256 | The internal ID of the civilization. |
| \_token_id     | uint256 | The token id of the composed ID.     |
