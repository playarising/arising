# Solidity API

## Civilizations

_`Civilizations` is the contract that stores all the usable civilizations._

### \_civilizations

```solidity
mapping(address => uint256) _civilizations
```

_Map to store the civilizations implemented. \*_

### civilizations

```solidity
address[] civilizations
```

_Array to store the civilizations implemented. \*_

### \_minters

```solidity
mapping(address => uint256) _minters
```

_Map to track the amount of tokens minted by address. \*_

### token

```solidity
address token
```

_Address of the token used to charge the mint. \*_

### \_upgrades

```solidity
struct ICivilizations.UpgradedCharacters _upgrades
```

_Map to track the character upgrades. \*_

### upgrades

```solidity
mapping(uint256 => struct ICivilizations.Upgrade) upgrades
```

_Map to track upgrades information. \*_

### constructor

```solidity
constructor(address _token) public
```

_Constructor._

| Name    | Type    | Description                                  |
| ------- | ------- | -------------------------------------------- |
| \_token | address | Address of the token to charge for upgrades. |

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

### setInitializeUpgrade

```solidity
function setInitializeUpgrade(uint256 upgrade, bool available) public
```

_Marks an upgrade to available.
@param upgrade The ID of the upgrade.
@param available The availability of the upgrade._

### setUpgradePrice

```solidity
function setUpgradePrice(uint256 upgrade, uint256 price) public
```

_Adds a civilization to the contract.
@param upgrade The ID of the upgrade.
@param price The amount of tokens to charge for the upgrade._

### setToken

```solidity
function setToken(address _token) public
```

_Sets the `token` address.
@param \_token address of the token to use for charge._

### addCivilization

```solidity
function addCivilization(address _instance) public
```

_Adds a civilization to the contract.
@param \_instance Address of the `BaseERC721` instance._

### mint

```solidity
function mint(address _instance) public
```

_Mints a token.
@param \_instance Address of the `BaseERC721` instance to mint._

### buyUpgrade

```solidity
function buyUpgrade(bytes id, uint256 upgrade) public
```

_Marks a character upgrade as purchased._

| Name    | Type    | Description               |
| ------- | ------- | ------------------------- |
| id      | bytes   | Composed ID of the token. |
| upgrade | uint256 | Upgrade id.               |

### withdraw

```solidity
function withdraw() public
```

_Transfers the total amount of `token` stored in the contract to `owner`._

### getID

```solidity
function getID(address _instance) public view returns (uint256)
```

_Returns the internal ID for a civilization.
@param \_instance Address of the `BaseERC721` instance._

### getTokenUpgrades

```solidity
function getTokenUpgrades(bytes id) public view returns (struct ICivilizations.TokenUpgrades)
```

_Returns the upgrades for a composed ID.
@param id Composed token id._

### getUpgradeInformation

```solidity
function getUpgradeInformation(uint256 upgrade) public view returns (struct ICivilizations.Upgrade)
```

_Returns the upgrades for a composed ID.
@param upgrade ID of the upgrade._

### getCivilizations

```solidity
function getCivilizations() public view returns (address[])
```

_Returns the list of civilizations._

### getTokenID

```solidity
function getTokenID(address _instance, uint256 _id) public view returns (bytes)
```

_Returns a composed ID from a collection.
@param \_instance Address of the `BaseERC721` instance.
@param \_id The ID of the token inside the collection._

### isAllowed

```solidity
function isAllowed(address spender, bytes _id) public view returns (bool)
```

_Function to check if an address has allowance to a composed ID.
@param spender Address to check ownership or allowance.
@param \_id Composed token id._

### exists

```solidity
function exists(bytes _id) public view returns (bool)
```

_Function to check if a composed ID is already minted.
@param \_id Composed token id._

### ownerOf

```solidity
function ownerOf(bytes _id) public view returns (address)
```

_Function to returns the actual owner of a composed ID.
@param \_id Composed token id._

### \_addMinted

```solidity
function _addMinted(address _minter) internal
```

_Adds a mint to the mint counter for the address._

| Name     | Type    | Description         |
| -------- | ------- | ------------------- |
| \_minter | address | The minter address. |

### \_canMint

```solidity
function _canMint(address _minter) internal view returns (bool)
```

_Checks if an address is able to mint more tokens._

| Name     | Type    | Description         |
| -------- | ------- | ------------------- |
| \_minter | address | The minter address. |

### \_decomposeTokenID

```solidity
function _decomposeTokenID(bytes id) internal pure returns (uint256, uint256)
```

_Decompose a byte encoded token ID._

| Name | Type  | Description            |
| ---- | ----- | ---------------------- |
| id   | bytes | The composed token id. |
