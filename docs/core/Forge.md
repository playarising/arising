# Solidity API

## Forge

This contract convets the raw resources into craftable material. It uses multiple instances of [BaseFungibleItem](/docs/base/BaseFungibleItem.md) items.
Each character has access to a maximum of three usable forges to convert the resources.

Implementation of the [IForge](/docs/interfaces/IForge.md) interface.

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

### stats

```solidity
address stats
```

_Address of the `Stats` instance. \*_

### recipes

```solidity
mapping(uint256 => struct IForge.Recipe) recipes
```

_Map to track available recipes on the forge. \*_

### \_recipes

```solidity
uint256[] _recipes
```

_Array to track all the recipes ids. \*_

### forges

```solidity
mapping(bytes => struct IForge.Forges) forges
```

_Map to track forges and cooldowns for each character. \*_

### token

```solidity
address token
```

_Address of the token used to charge the mint. \*_

### price

```solidity
uint256 price
```

_Price of forge upgrades. \*_

### gold

```solidity
address gold
```

_The address of the `Gold` instance. \*_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

### constructor

```solidity
constructor(address _civilizations, address _experience, address _stats, address _gold, address _token, uint256 _price) public
```

Constructor.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |
| \_experience    | address | The address of the [Experience](/docs/core/Experience.md) instance.       |
| \_stats         | address | The address of the [Stats](/docs/core/Stats.md) instance.                 |
| \_gold          | address | The address of the [Gold](/docs/gadgets/Gold.md) instance.                |
| \_token         | address | Address of the token used to purchase.                                    |
| \_price         | uint256 | Price for each token.                                                     |

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

### disableRecipe

```solidity
function disableRecipe(uint256 id) public
```

_Disables a forge recipe._

| Name | Type    | Description       |
| ---- | ------- | ----------------- |
| id   | uint256 | ID of the recipe. |

### enableRecipe

```solidity
function enableRecipe(uint256 id) public
```

_Enables a forge recipe._

| Name | Type    | Description       |
| ---- | ------- | ----------------- |
| id   | uint256 | ID of the recipe. |

### addRecipe

```solidity
function addRecipe(address[] materials, uint256[] amounts, struct IStats.BasicStats stats, uint256 cooldown, uint256 level_required, uint256 cost, uint256 experience_reward, address reward) public
```

_Adds a new recipe to the forge._

| Name              | Type                     | Description                                      |
| ----------------- | ------------------------ | ------------------------------------------------ |
| materials         | address[]                | Addresses of the raw resources for the creation. |
| amounts           | uint256[]                | Amounts for each raw resource.                   |
| stats             | struct IStats.BasicStats | Stat cost for the recipe.                        |
| cooldown          | uint256                  | Cooldown in seconds for the recipe.              |
| level_required    | uint256                  | Minimum level required.                          |
| cost              | uint256                  | Gold cost of the recipe.                         |
| experience_reward | uint256                  | Amount of experience rewarded.                   |
| reward            | address                  | Address of the reward contract.                  |

### buyUpgrade

```solidity
function buyUpgrade(bytes id) public
```

_Upgrades character to use another forge slot._

| Name | Type  | Description               |
| ---- | ----- | ------------------------- |
| id   | bytes | Composed ID of the token. |

### forge

```solidity
function forge(bytes id, uint256 recipe, uint256 _forge) public
```

_Forges a recipe._

| Name    | Type    | Description                   |
| ------- | ------- | ----------------------------- |
| id      | bytes   | Composed ID of the character. |
| recipe  | uint256 | ID of the recipe to forge.    |
| \_forge | uint256 | Number of the forge to use.   |

### claim

```solidity
function claim(bytes id, uint256 _forge) public
```

_Claims a recipe already forged._

| Name    | Type    | Description                   |
| ------- | ------- | ----------------------------- |
| id      | bytes   | Composed ID of the character. |
| \_forge | uint256 | Number of the forge to use.   |

### withdraw

```solidity
function withdraw() public
```

_Transfers the total amount of `token` stored in the contract to `owner`._

### getRecipe

```solidity
function getRecipe(uint256 id) public view returns (struct IForge.Recipe)
```

_Reurns the recipe information of a recipe id._

| Name | Type    | Description       |
| ---- | ------- | ----------------- |
| id   | uint256 | ID of the recipe. |

### getCharacterForge

```solidity
function getCharacterForge(bytes id, uint256 _forge) public view returns (struct IForge.Forge)
```

_Reurns the forge information of a composed ID._

| Name    | Type    | Description                   |
| ------- | ------- | ----------------------------- |
| id      | bytes   | Composed ID of the character. |
| \_forge | uint256 | ID of the forge.              |

### getCharacterForgesUpgrades

```solidity
function getCharacterForgesUpgrades(bytes id) public view returns (bool[3])
```

_Reurns an array of booleans for the character forges upgraded._

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |

### getCharacterForgesAvailability

```solidity
function getCharacterForgesAvailability(bytes id) public view returns (bool[3])
```

_Reurns an array of booleans for the character forges available to use._

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |

### \_isForgeAvailable

```solidity
function _isForgeAvailable(bytes id, uint256 _forge) internal view returns (bool)
```

_Internal function to check if a forge id is available to use._

| Name    | Type    | Description                   |
| ------- | ------- | ----------------------------- |
| id      | bytes   | Composed ID of the character. |
| \_forge | uint256 | Number of the forge to use.   |

### \_isForgeClaimable

```solidity
function _isForgeClaimable(bytes id, uint256 _forge) internal view returns (bool)
```

_Internal function to check if a forge id is ready to claim._

| Name    | Type    | Description                   |
| ------- | ------- | ----------------------------- |
| id      | bytes   | Composed ID of the character. |
| \_forge | uint256 | Number of the forge to use.   |

### \_assignRecipeToForge

```solidity
function _assignRecipeToForge(bytes id, uint256 _forge, struct IForge.Recipe r) internal
```

_Internal function to assign a recipe to a forge to create. This function assumes the forge trying to accessing
is available (upgraded) and usable._

| Name    | Type                 | Description                   |
| ------- | -------------------- | ----------------------------- |
| id      | bytes                | Composed ID of the character. |
| \_forge | uint256              | Number of the forge to use.   |
| r       | struct IForge.Recipe | Recipe to be assigned.        |

### \_claimForge

```solidity
function _claimForge(bytes id, uint256 _forge) internal returns (uint256)
```

_Internal function claim a reward from a forge._

| Name    | Type    | Description                   |
| ------- | ------- | ----------------------------- |
| id      | bytes   | Composed ID of the character. |
| \_forge | uint256 | Number of the forge to use.   |

### \_getForgeFromID

```solidity
function _getForgeFromID(bytes id, uint256 _forge) internal view returns (bool, struct IForge.Forge)
```

_Internal function to return a forge instance from a number._

| Name    | Type    | Description                   |
| ------- | ------- | ----------------------------- |
| id      | bytes   | Composed ID of the character. |
| \_forge | uint256 | Number of the forge to use.   |
