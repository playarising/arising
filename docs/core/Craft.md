# Solidity API

## Craft

This contract is used to store and craft recipes through the ecosystem. This is the only contract able to mint
items through the [Items](/docs/items/Items.md) `ERC1155` implementation.

Implementation of the [ICraft](/docs/interfaces/ICraft.md) interface.

### recipes

```solidity
mapping(uint256 => struct ICraft.Recipe) recipes
```

_Map to track available recipes on the forge. \*_

### \_recipes

```solidity
uint256[] _recipes
```

_Array to track all the recipes ids. \*_

### gold

```solidity
address gold
```

_The address of the [Gold](/docs/gadgets/Gold.md) instance. \*_

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

### stats

```solidity
address stats
```

_Address of the [Stats](/docs/core/Stats.md) instance. \*_

### items

```solidity
address items
```

_Address of the [Items](/docs/items/Items.md) instance. \*_

### slots

```solidity
mapping(bytes => struct ICraft.CraftSlot) slots
```

_Map to track craft slots and cooldowns for each character. \*_

### onlyAllowed

```solidity
modifier onlyAllowed(bytes id)
```

_Checks if `msg.sender` is owner or allowed to manipulate a composed ID._

### constructor

```solidity
constructor(address _civilizations, address _experience, address _stats, address _gold, address _items) public
```

Constructor.

Requirements:

| Name            | Type    | Description                                                               |
| --------------- | ------- | ------------------------------------------------------------------------- |
| \_civilizations | address | The address of the [Civilizations](/docs/core/Civilizations.md) instance. |
| \_experience    | address | The address of the [Experience](/docs/core/Experience.md) instance.       |
| \_stats         | address | The address of the [Stats](/docs/core/Stats.md) instance.                 |
| \_gold          | address | The address of the [Gold](/docs/gadgets/Gold.md) instance.                |
| \_items         | address | The address of the [Items](/docs/items/Items.md) instance.                |

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

_Disables a craft recipe._

| Name | Type    | Description       |
| ---- | ------- | ----------------- |
| id   | uint256 | ID of the recipe. |

### enableRecipe

```solidity
function enableRecipe(uint256 id) public
```

_Enables a craft recipe._

| Name | Type    | Description       |
| ---- | ------- | ----------------- |
| id   | uint256 | ID of the recipe. |

### craft

```solidity
function craft(bytes id, uint256 recipe) public
```

_Craft a recipe._

| Name   | Type    | Description                   |
| ------ | ------- | ----------------------------- |
| id     | bytes   | Composed ID of the character. |
| recipe | uint256 | ID of the recipe to craft.    |

### claim

```solidity
function claim(bytes id) public
```

_Claims a recipe already crafted._

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |

### getRecipe

```solidity
function getRecipe(uint256 id) public view returns (struct ICraft.Recipe)
```

_Reurns the recipe information of a recipe id._

| Name | Type    | Description       |
| ---- | ------- | ----------------- |
| id   | uint256 | ID of the recipe. |

### getCharacterSlot

```solidity
function getCharacterSlot(bytes id) public view returns (struct ICraft.CraftSlot)
```

_Reurns the craft slot information of a composed ID._

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |

### \_isSlotAvailable

```solidity
function _isSlotAvailable(bytes id) internal view returns (bool)
```

_Internal function to check if the craft slot is available to craft._

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |

### \_isSlotClaimable

```solidity
function _isSlotClaimable(bytes id) internal view returns (bool)
```

_Internal function to check if a craft slot is ready to claim._

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |

### \_assignRecipeToSlot

```solidity
function _assignRecipeToSlot(bytes id, struct ICraft.Recipe r) internal
```

_Internal function to assign a recipe to a slot for craft_

| Name | Type                 | Description                   |
| ---- | -------------------- | ----------------------------- |
| id   | bytes                | Composed ID of the character. |
| r    | struct ICraft.Recipe | Recipe to be assigned.        |

### \_claim

```solidity
function _claim(bytes id) internal returns (uint256)
```

_Internal function claim a reward from the slot and return the reward experience.
This function assumes the slot is claimable_

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| id   | bytes | Composed ID of the character. |
