# Solidity API

## Craft

This contract is used to store and craft recipes through the ecosystem. This is the only contract able to mint
items through the [Items](/docs/items/Items.md) `ERC1155` implementation.

Implementation of the [ICraft](/docs/interfaces/ICraft.md) interface.

### recipes

```solidity
mapping(uint256 => struct ICraft.Recipe) recipes
```

Map to track available recipes for craft.

### \_recipes

```solidity
uint256[] _recipes
```

Array to track all the recipes ids.

### gold

```solidity
address gold
```

The address of the [Gold](/docs/gadgets/Gold.md) instance.

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

### stats

```solidity
address stats
```

Address of the [Stats](/docs/core/Stats.md) instance.

### items

```solidity
address items
```

Address of the [Items](/docs/items/Items.md) instance.

### craft_slots

```solidity
mapping(bytes => struct ICraft.CraftSlot) craft_slots
```

Map to track craft slots and cooldowns for each character.

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
function disableRecipe(uint256 _recipe_id) public
```

Disables a recipe from beign crafted.

Requirements:

| Name        | Type    | Description       |
| ----------- | ------- | ----------------- |
| \_recipe_id | uint256 | ID of the recipe. |

### enableRecipe

```solidity
function enableRecipe(uint256 _recipe_id) public
```

Enables a recipe to be crafted.

Requirements:

| Name        | Type    | Description       |
| ----------- | ------- | ----------------- |
| \_recipe_id | uint256 | ID of the recipe. |

### craft

```solidity
function craft(bytes _id, uint256 _recipe_id) public
```

Initializes a recipe to be crafted.

Requirements:

| Name        | Type    | Description                   |
| ----------- | ------- | ----------------------------- |
| \_id        | bytes   | Composed ID of the character. |
| \_recipe_id | uint256 | ID of the recipe.             |

### claim

```solidity
function claim(bytes _id) public
```

Claims a recipe already crafted.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

### getRecipe

```solidity
function getRecipe(uint256 _recipe_id) public view returns (struct ICraft.Recipe _recipe)
```

Returns the full information of a recipe.

Requirements:

| Name        | Type    | Description       |
| ----------- | ------- | ----------------- |
| \_recipe_id | uint256 | ID of the recipe. |

| Name     | Type                 | Description                    |
| -------- | -------------------- | ------------------------------ |
| \_recipe | struct ICraft.Recipe | Full information of the recipe |

### getCharacterSlot

```solidity
function getCharacterSlot(bytes _id) public view returns (struct ICraft.CraftSlot _slot)
```

Returns character craft slot information.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name   | Type                    | Description                                  |
| ------ | ----------------------- | -------------------------------------------- |
| \_slot | struct ICraft.CraftSlot | Full information of character crafting slot. |

### \_isSlotAvailable

```solidity
function _isSlotAvailable(bytes _id) internal view returns (bool _available)
```

Internal check if the crafting slot is available to be used.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name        | Type | Description                               |
| ----------- | ---- | ----------------------------------------- |
| \_available | bool | Boolean to know if the slot is available. |

### \_isSlotClaimable

```solidity
function _isSlotClaimable(bytes _id) internal view returns (bool)
```

Internal check if the crafting slot is claimable.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name | Type | Description                                           |
| ---- | ---- | ----------------------------------------------------- |
| [0]  | bool | \_available Boolean to know if the slot is claimable. |

### \_assignRecipeToSlot

```solidity
function _assignRecipeToSlot(bytes _id, struct ICraft.Recipe _recipe) internal
```

Internal function that assigns a recipe to the crafting slot.

Requirements:

| Name     | Type                 | Description                   |
| -------- | -------------------- | ----------------------------- |
| \_id     | bytes                | Composed ID of the character. |
| \_recipe | struct ICraft.Recipe | Recipe to assign.             |

### \_claim

```solidity
function _claim(bytes _id) internal returns (uint256 _experience)
```

Internal function to claim the reward from the slot.

Requirements:

| Name | Type  | Description                   |
| ---- | ----- | ----------------------------- |
| \_id | bytes | Composed ID of the character. |

| Name         | Type    | Description                    |
| ------------ | ------- | ------------------------------ |
| \_experience | uint256 | Amount of experience rewarded. |
