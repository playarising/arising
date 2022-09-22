# Solidity API

## Civilizations

This contract stores all the [BaseERC721](/docs/base/BaseERC721.md) instances usable on the environmne. The contract
is in charge of token ownership verifications and generating/storing composable IDs for each character.

Implementation of the [ICivilizations](/docs/interfaces/ICivilizations.md) interface.

### civilizations

```solidity
mapping(uint256 => address) civilizations
```

Map to track the supported [BaseERC721](/docs/base/BaseERC721.md) instances.

### civilizations_id

```solidity
mapping(address => uint256) civilizations_id
```

Map to track the IDs of the supported [BaseERC721](/docs/base/BaseERC721.md) instances.

### \_civilizations

```solidity
uint256[] _civilizations
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

### price

```solidity
uint256 price
```

Map to the price to mint characters.

### onlyCivilization

```solidity
modifier onlyCivilization()
```

Checks if the `msg.sender` is a civilization contract to emit a Transfer event.

### Summoned

```solidity
event Summoned(address _owner, bytes _id)
```

Event emmited when the [mint](#mint) function is called.

Requirements:

| Name    | Type    | Description                   |
| ------- | ------- | ----------------------------- |
| \_owner | address | Owner of the minted token.    |
| \_id    | bytes   | Composed ID of the character. |

### UpgradePurchased

```solidity
event UpgradePurchased(bytes _id, uint256 _upgrade_id)
```

Event emmited when the [buyUpgrade](#buyUpgrade) function is called.

Requirements:

| Name         | Type    | Description                   |
| ------------ | ------- | ----------------------------- |
| \_id         | bytes   | Composed ID of the character. |
| \_upgrade_id | uint256 | ID of the upgrade purchased.  |

### Transfer

```solidity
event Transfer(address _from, address _to, bytes _id)
```

Event emmited when the [transfer](#transfer) function is called.

Requirements:

| Name   | Type    | Description                                         |
| ------ | ------- | --------------------------------------------------- |
| \_from | address | Address of the character transfering the character. |
| \_to   | address | New owner of the character.                         |
| \_id   | bytes   | Composed ID of the character                        |

### initialize

```solidity
function initialize(address _token) public
```

Initialize.

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

### transfer

```solidity
function transfer(address _from, address _to, uint256 _token_id) public
```

Emits a transfer event to track the user of the character.

Requirements:

| Name       | Type    | Description                                                                              |
| ---------- | ------- | ---------------------------------------------------------------------------------------- |
| \_from     | address | Address of the character transfering the character.                                      |
| \_to       | address | New owner of the character.                                                              |
| \_token_id | uint256 | ID of the character in the context of a [BaseERC721](/docs/base/BaseERC721.md) instance. |

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

### setMintPrice

```solidity
function setMintPrice(uint256 _price) public
```

Sets the price to mint a character.

Requirements:

| Name    | Type    | Description                              |
| ------- | ------- | ---------------------------------------- |
| \_price | uint256 | Amount of tokens to pay for the upgrade. |

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
function mint(uint256 _civilization_id) public
```

Creates a new token of the valid civilizations list to the `msg.sender`.

Requirements:

| Name              | Type    | Description             |
| ----------------- | ------- | ----------------------- |
| \_civilization_id | uint256 | ID of the civilization. |

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

### getTokenID

```solidity
function getTokenID(uint256 _civilization_id, uint256 _token_id) public view returns (bytes _id)
```

Returns the composed ID of a token from a valid civilization.

Requirements:

| Name              | Type    | Description                             |
| ----------------- | ------- | --------------------------------------- |
| \_civilization_id | uint256 | ID of the civilization.                 |
| \_token_id        | uint256 | ID of the token to get the composed ID. |

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### isAllowed

```solidity
function isAllowed(address _spender, bytes _id) public view returns (bool _allowed)
```

External function to check if the `msg.sender` can access a token.

Requirements:

| Name      | Type    | Description                              |
| --------- | ------- | ---------------------------------------- |
| \_spender | address | Address to check ownership or allowance. |
| \_id      | bytes   | Composed ID of the character.            |

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

### \_authorizeUpgrade

```solidity
function _authorizeUpgrade(address newImplementation) internal virtual
```

Internal function make sure upgrade proxy caller is the owner.
